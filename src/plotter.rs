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
    cache: Cache,
    view: View,
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

    pub fn update_view(&mut self, view: View) {
        self.view = view;
    }

    pub fn add_graphs(&mut self, graphs: Vec<Graph2D>) {
        self.graphs.extend(graphs);
    }

    // pub fn clear_graphs(&mut self) {
    //     self.graphs.clear();
    // }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    fn draw_graphs(&self, frame: &mut Frame, origin: Vec2) {
        self.graphs.iter().for_each(|graph| {
            graph.draw(frame, origin, &self.view);
        });
    }

    // pub fn translate_graphs(&mut self, translation: Vec2) {
    //     (&mut self.graphs).into_iter()
    //         .for_each(|g: &mut Graph2D| g.translate(translation))
    // }

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
            cache: Cache::default(),
            view: View::default(),
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
                let pos: Vec2 = cursor.position().unwrap().into();
                *state = State::LeftButtonDown(pos);
            },
            event!(LEFT_BUTTON_RELEASED) => {
                *state = State::Idle;
            },
            event!(MOUSE_MOVE: new_pos) => {
                if let State::LeftButtonDown(old_pos) = *state {
                    // let offset: Vec2 = cursor.position_in(bounds).unwrap().into();
                    let offset: Vec2 = Vec2::from(new_pos) - old_pos;
                    let old_offset = self.view.offset;

                    let view = View::new(old_offset + offset);

                    *state = State::LeftButtonDown(new_pos.into());

                    return (CanvasStatus::Captured, Some(Message::UpdateView(view)))
                }
                return (CanvasStatus::Ignored, None)
            },
            _ => (),
        }
        (CanvasStatus::Ignored, None)
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