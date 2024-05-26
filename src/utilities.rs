use std::ops::Sub;
use rand::random;

#[rustfmt::skip]
use iced::{
    self,
    widget::canvas::{self, Frame, Path},
    Color, Point
};

use crate::{
    plotter::Plotter2D,
    vector::Vec2,
    graph::Graph2D,
};

pub fn draw_background(frame: &mut Frame, color: Color) {
    let path = Path::rectangle(Point::ORIGIN, frame.size());
    frame.fill(&path, color);
}

/// returns a random f32 between -1.0 and 1.0
pub fn rnd_signed() -> f32 {
    let sign = if random::<bool>() { 1.0 } else { -1.0 };
    sign * random::<f32>()
}

pub fn rnd_color() -> Color {
    Color {
        r: random::<f32>(),
        g: random::<f32>(),
        b: random::<f32>(),
        a: 1.0,
    }
}

