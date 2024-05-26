use std::vec;
use iced::widget::shader::wgpu::naga::back;
#[rustfmt::skip]
use iced::{
    mouse::{self, Event as MouseEvent},
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

#[rustfmt::skip]
use crate::{
    utilities::{self, draw_background},
    graph::Graph2D,
    vector::Vec2,
    events::*,
    view::View,
};

pub struct Plotter2D {
    graphs: Vec<Graph2D>,
    view: View,
    cache: Cache,
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

    fn draw_graphs(&self, frame: &mut Frame, origin: Vec2) {
        self.graphs.iter().for_each(|graph| {
            graph.draw(frame, origin);
        });
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
    type State = ();

    fn update(
            &self,
            _state: &mut Self::State,
            event: CanvasEvent,
            bounds: Rectangle,
            cursor: mouse::Cursor,
    ) -> (CanvasStatus, Option<Message>) {
        if event == CANVAS_KEY_R_PRESSED {
            (CanvasStatus::Captured, Some(Message::Redraw))
        } else {
            (CanvasStatus::Ignored, None)
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