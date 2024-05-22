use std::ops::Add;
#[rustfmt::skip]
use iced::{
    mouse,
    widget::{canvas::{Frame, Path}, Canvas},
    Color, Point
};

#[derive(Debug, Clone)]
pub enum Graph2D {
    Point(Point, Color),
    Polygon(Vec<Point>, Color),
}

impl Graph2D {
    pub fn draw(&self, frame: &mut Frame, origin: Point) {
        let mut flipped = self.clone();
        flipped.flip_y();
        
        // mapping graph to origin
        match flipped + origin {
            Self::Point(point, color) => {
                let circle = Path::circle(point, 5.0);
                frame.fill(&circle, color)
            },
            Self::Polygon(points, color) => {
                if points.len() < 2 {
                    return;
                }

                let path = Path::new(|builder| {
                    builder.move_to(points[0]);

                    for p in points.iter().skip(1) {
                        builder.line_to(*p);
                    }
                    builder.close();
                });
                
                frame.fill(&path, color);
            }
        }
    }

    pub fn flip_y(&mut self) {
        match self {
            Self::Point(point, _) => point.y = -point.y,
            Self::Polygon(points, _) => points.iter_mut().for_each(|point| point.y = -point.y)
        }
    }
} 

impl Default for Graph2D {
    fn default() -> Self {
        Self::Point(Point::ORIGIN, Color::WHITE)
    }
}

impl Add<Point> for Graph2D {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        match self {
            Self::Point(point, color) => Self::Point(
                Point{x: point.x + rhs.x, y: point.y + rhs.y},
                color
            ),
            Self::Polygon(points, color) => {
                let points = points.iter()
                    .map(|p| Point{x: p.x + rhs.x, y: p.y + rhs.y})
                    .collect();

                Self::Polygon(points, color)
            }
        }
    }
} 
