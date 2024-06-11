use std::vec;

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
    vector::*,
    events::*,
};
    
#[rustfmt::skip]
use super::{
    view::View,
    graph::Graph2D,
    settings::Settings,
    builder::Builder,
};

pub struct Plotter2D {
    pub graphs: Vec<Graph2D>,
    pub settings: Settings,
    pub cache: Cache,
    pub view: View,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum State {
    #[default]
    Idle,
    LeftButtonDown(Vec2),
}

impl Plotter2D {
    pub fn builder() -> Builder {
        Builder::default()
    }

    // pub fn canvas(&self, width: Length, height: Length) -> Canvas<&Self, Message> {
    //     canvas(self)
    //         .width(width)
    //         .height(height)
    // }

    pub fn update_view(&mut self, view: View) {
        self.view = view;
    }

    // pub fn add_graphs(&mut self, graphs: Vec<Graph2D>) {
    //     self.graphs.extend(graphs);
    // }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    fn draw_graphs(&self, frame: &mut Frame, origin: &Vec2) {
        self.graphs.iter().for_each(|graph| {
            graph.draw(frame, origin, &self.view);
        });
    }

    fn draw_background(&self, frame: &mut Frame) {
        let color = self.settings.background;
        let path = Path::rectangle(Point::ORIGIN, frame.size());
        frame.fill(&path, color);
    }

    // fn draw_plane(&self, frame: &mut Frame) {
    //     self.draw
    // }
}

impl Default for Plotter2D {
    fn default() -> Self {
        Self {
            graphs: vec![],
            cache: Cache::default(),
            view: View::default(),
            settings: Settings::default(),
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
            event!(MOUSE_LEFT_DOWN) => {
                let pos: Vec2 = cursor.position().unwrap().into();
                *state = State::LeftButtonDown(pos);
            },
            event!(MOUSE_LEFT_UP) => {
                *state = State::Idle;
            },
            event!(MOUSE_MOVE: new_pos) => {
                if let State::LeftButtonDown(old_pos) = *state {
                    let old_offset = self.view.offset;
                    let offset: Vec2 = Vec2::from(new_pos) - old_pos;

                    let view = View {
                        offset: old_offset + offset,
                        ..self.view
                    };

                    // update cursor position
                    *state = State::LeftButtonDown(new_pos.into());

                    return (CanvasStatus::Captured, Some(Message::UpdateView(view)))
                }
            },
            event!(MOUSE_SCROLL: delta) => {
                let old_zoom = self.view.zoom;
                let zoom = View::zoom_from_delta(delta);
                
                let view = View {
                    zoom: old_zoom + zoom,
                    ..self.view
                };

                return (CanvasStatus::Captured, Some(Message::UpdateView(view)))
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
        
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            self.draw_background(frame);      

            // Draw graphs
            let origin = Vec2::new(bounds.width, bounds.height) / 2.0;
            self.draw_graphs(frame, &origin);
        });

        vec![geometry]
    }
}