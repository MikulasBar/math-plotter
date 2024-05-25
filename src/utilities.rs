use std::ops::Sub;
use rand::random;

#[rustfmt::skip]
use iced::{
    self,
    widget::canvas::{self, Cache, Frame, Geometry, Path},
    Color, Point, Rectangle, Renderer
};

pub fn draw_background(frame: &mut Frame, color: Color) {
    let path = Path::rectangle(Point::ORIGIN, frame.size());
    frame.fill(&path, color);
}

fn rnd_signed() -> f32 {
    let sign = if random::<bool>() { 1.0 } else { -1.0 };
    sign * random::<f32>()
}

pub fn rnd_point(factor: f32) -> Point {
    let x = rnd_signed() * factor;
    let y = rnd_signed() * factor;
    Point{x, y}
}

pub fn rnd_color() -> Color {
    Color {
        r: random::<f32>(),
        g: random::<f32>(),
        b: random::<f32>(),
        a: 1.0,
    }
}