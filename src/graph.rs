use std::{ops::Add, vec};
#[rustfmt::skip]
use iced::{
    mouse,
    widget::{canvas::{Frame, Path}, Canvas},
    Color, Point
};
use crate::vector::Vec2;


#[derive(Debug, Clone)]
pub enum Graph2D {
    Point(Vec2, Color),
    Polygon(Vec<Vec2>, Color),
}

impl Graph2D {
    pub fn draw(&self, frame: &mut Frame, origin: Vec2) {
        match self {
            Self::Point(vector, color) => {
                let point = vector.prepare_for_drawing(&origin);
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
                    .map(|v| v.prepare_for_drawing(&origin))
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

    pub fn _translate_to(&mut self, origin: &Vec2) {
        use self::Graph2D::*;

        match self {
            Point(vector, _) => *vector += *origin,
            Polygon(vectors, _) => {
                *vectors = vectors.iter()
                    .map(|v| *v + *origin)
                    .collect();
            },
        }
    }
} 

impl Default for Graph2D {
    fn default() -> Self {
        Self::Point(Vec2::ZERO, Color::WHITE)
    }
}



