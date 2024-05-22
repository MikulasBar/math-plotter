use rand::random;
use iced::{
    self,
    widget::canvas::{self, Path, Frame},
    Color, Point,
};

pub fn background(frame: &mut Frame, color: Color) {
    let canvas_path = Path::rectangle(Point::ORIGIN, frame.size());
    frame.fill(&canvas_path, color);
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

/// This function maps a point to a new point with a given origin <br>
/// This function flips the y-axis so the coordinates are more intuitive (X increases rightway, Y increases upwards)
pub fn map_with_origin(point: Point, origin: Point) -> Point {
    Point {
        x: point.x + origin.x,
        y: -point.y + origin.y,
    }
}