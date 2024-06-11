

#[rustfmt::skip]
use iced::{
    mouse,
    widget::{canvas::{Frame, Path}, Canvas},
    Color, Point
};

#[rustfmt::skip]
use crate::{
    vector::Vec2,
    utilities,
};

#[rustfmt::skip]
use super::{
    view::View
};



#[derive(Debug, Clone)]
pub enum Graph2D {
    Point(Vec2, Color),
    Polygon(Vec<Vec2>, Color),
}

impl Graph2D {
    pub fn draw(&self, frame: &mut Frame, origin: &Vec2, view: &View) {
        match self {
            Self::Point(vector, color) => {
                let point = vector.prepare_for_drawing(*origin, view);
                let circle = Path::circle(point, 5.0);
                frame.fill(&circle, *color)
            },
            Self::Polygon(vectors, color) => {
                // cannot draw a polygon with less than 2 points
                // 2 points will be a line
                if vectors.len() < 2 {
                    return;
                }

                let points: Vec<Point> = vectors.iter()
                    .map(|v| v.prepare_for_drawing(*origin, view))
                    .collect();

                let path = Path::new(|builder| {
                    builder.move_to(points[0]);

                    for p in points.iter().skip(1) {
                        builder.line_to(*p);
                    }
                    builder.close();
                });
                
                frame.fill(&path, *color);
            }
        }
    }

    pub fn random_points(num: u32, factor: f32, color_fn: fn() -> Color) -> Vec<Self> {
        (0..num).into_iter()
            .map(|_| {
                let vector = Vec2::random(factor);
                let color = color_fn();
                Self::Point(vector, color)
            })
            .collect()
    }

    fn mutate_vecs(&mut self, f: impl Fn(&mut Vec2)) {
        match self {
            Self::Point(vector, _) => f(vector),
            Self::Polygon(vectors, _) => {
                vectors.into_iter()
                    .for_each(f)
            },
        }
    }

    pub fn translate(&mut self, translation: Vec2) {
        self.mutate_vecs(|v| v.translate(translation));
    }

    pub fn scale(&mut self, factor: f32) {
        self.mutate_vecs(|v| v.scale(factor))
    }
} 

impl Default for Graph2D {
    fn default() -> Self {
        Self::Point(Vec2::ZERO, Color::WHITE)
    }
}



