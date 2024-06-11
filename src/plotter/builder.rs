#[rustfmt::skip]
use iced::{
    Color,
};

#[rustfmt::skip]
use crate::{
    vector::Vec2,
};

#[rustfmt::skip]
use super::{
    graph::Graph2D,
    settings::Settings,
    plotter::Plotter2D,
};

pub struct Builder {
    settings: Settings,
    graphs: Vec<Graph2D>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            graphs: Vec::new(),
        }
    }
}

impl Builder {
    pub fn add_graphs(mut self, graphs: Vec<Graph2D>) -> Self {
        self.graphs.extend(graphs);
        self
    }

    pub fn add_control_points(mut self) -> Self {
        let center = Graph2D::Point(Vec2::ZERO, Color::WHITE);
        let right = Graph2D::Point(Vec2::UNIT_X * 100.0, Color::WHITE);
        let up = Graph2D::Point(Vec2::UNIT_Y * 100.0, Color::WHITE);
    
        self.graphs.extend(vec![center, right, up]);

        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.settings.background = color;
        self
    }

    pub fn build(self) -> Plotter2D {
        let builder = self;
        Plotter2D {
            settings: builder.settings,
            graphs: builder.graphs,
            ..Plotter2D::default()
        }
    }
}


