use std::{default, vec};
use iced::{event, futures::stream::Skip, widget::{canvas::path::lyon_path::Position, shader::wgpu::naga::back}};
#[rustfmt::skip]
use iced::{
    mouse::{self, Event as MouseEvent, Button as MouseButton},
    widget::{
        canvas::{
            Cache, Frame, Geometry, Path,
            Event as CanvasEvent,
            event::Status as CanvasStatus,
        },
        Canvas, canvas
    },
    Color, Length, Point, Rectangle, Renderer, Theme
};

// macros
use crate::event;

#[rustfmt::skip]
use crate::{
    utilities::{self, draw_background},
    graph::Graph2D,
    vector::*,
    events::*,
    view::View,
};

pub struct Plotter2D {
    graphs: Vec<Graph2D>,
    view: View,
    cache: Cache,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum State {
    #[default]
    Idle,
    LeftButtonDown(Vec2),
}

impl Plotter2D {
    pub fn new() -> Self {
        Self {
            ..Self::default()
        }
    }

    // pub fn canvas(&self, width: Length, height: Length) -> Canvas<&Self, Message> {
    //     canvas(self)
    //         .width(width)
    //         .height(height)
    // }

    // pub fn update_view(&mut self, offset: Point) {
    //     self.view.offset -= offset;
    // }

    pub fn push(&mut self, graph: Graph2D) {
        self.graphs.push(graph);
    }

    pub fn add_graphs(&mut self, graphs: Vec<Graph2D>) {
        self.graphs.extend(graphs);
    }

    pub fn clear_graphs(&mut self) {
        self.graphs.clear();
    }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    // pub fn clear(&mut self) {
    //     self.clear_graphs();
    //     self.clear_cache();
    // }

    fn draw_graphs(&self, frame: &mut Frame, origin: Vec2) {
        self.graphs.iter().for_each(|graph| {
            graph.draw(frame, origin);
        });
    }

    pub fn translate_graphs(&mut self, translation: Vec2) {
        (&mut self.graphs).into_iter()
            .for_each(|g: &mut Graph2D| g.translate(translation))
    }

    pub fn add_control_points(&mut self) {
        let center = Graph2D::Point(Vec2::ZERO, Color::WHITE);
        let right = Graph2D::Point(Vec2::UNIT_X * 100.0, Color::WHITE);
        let up = Graph2D::Point(Vec2::UNIT_Y * 100.0, Color::WHITE);
    
        self.add_graphs(vec![center, right, up])
    }
}

impl Default for Plotter2D {
    fn default() -> Self {
        Self {
            graphs: vec![],
            view: View::default(),
            cache: Cache::default(),
        }
    }
}

impl canvas::Program<Message> for Plotter2D {
    type State = State;

    fn update(
            &self,
            state: &mut Self::State,
            event: CanvasEvent,
            bounds: Rectangle,
            cursor: mouse::Cursor,
    ) -> (CanvasStatus, Option<Message>) {
        if !cursor.is_over(bounds) {
            return (CanvasStatus::Ignored, None);
        }
        
        match event {
            event!(LEFT_BUTTON_PRESSED) => {
                let pos = cursor.position().unwrap().into();
                *state = State::LeftButtonDown(pos);
                (CanvasStatus::Captured, None)
            },
            event!(LEFT_BUTTON_RELEASED) => {
                *state = State::Idle;
                (CanvasStatus::Captured, None)
            },
            event!(MOUSE_MOVE: new_pos) => {
                match *state {
                    State::LeftButtonDown(old_pos) => {
                        let translation = Vec2::from(new_pos) - old_pos;
                        *state = State::LeftButtonDown(new_pos.into());
                        return (CanvasStatus::Captured, Some(Message::Translate(translation)))
                    },
                    _ => (),
                }
                (CanvasStatus::Ignored, None)
            },
            _ => (CanvasStatus::Ignored, None),
        }
    }


    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let bg_color = Color::from_rgb8(0x36, 0x39, 0x3F);
        
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            draw_background(frame, bg_color);      

            // Draw graphs
            let origin = Vec2::new(bounds.width, bounds.height) / 2.0;
            self.draw_graphs(frame, origin);
        });

        vec![geometry]
    }
}