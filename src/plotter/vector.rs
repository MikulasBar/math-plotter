use std::ops::{Add, Sub, Mul, Div};
use iced::{self, Point};

use crate::{
    utilities::rnd_signed,
    plotter::view::View,
};



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const UNIT_X: Self = Self { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Self = Self { x: 0.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn translate(&mut self, translation: Vec2) {
        *self = *self + translation;
    }

    pub fn scale(&mut self, factor: f32) {
        *self = *self * factor;
    }

    /// Prepare the vector for drawing on the canvas.
    /// 
    /// Flips the Y coordinate so the Y increases upwards.
    /// 
    /// Translate the vector according to `view` and `origin`.
    /// 
    /// Converts [`Vec2`] to [`Point`].
    pub fn prepare_for_drawing(&self, origin: Vec2, view: View) -> Point {
        let View {
            offset,
            zoom,
            ..
        } = view;

        let vector = self.flip_y() * zoom + origin + offset;
        Point::from(vector)
    }

    /// Flips the x coordinate of the vector.
    /// 
    /// Doesnt effect the `self` vector but returns a new vector.
    pub fn flip_x(&self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }

    /// Flips the y coordinate of the vector.
    /// Doesnt effect the `self` vector but returns a new vector.
    pub fn flip_y(&self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
        }
    }

    pub fn random(factor: f32) -> Vec2 {
        let x = rnd_signed();
        let y = rnd_signed();
        Vec2{x, y} * factor
    }
}

mod froms {
    use super::*;

    impl From<Vec2> for Point {
        fn from(vector: Vec2) -> Self {
            Point::new(vector.x, vector.y)
        }
    }
    
    impl From<&Vec2> for Point {
        fn from(vector: &Vec2) -> Self {
            Point::new(vector.x, vector.y)
        }
    }
    
    impl From<Point> for Vec2 {
        fn from(point: Point) -> Self {
            Self {
                x: point.x,
                y: point.y,
            }
        }
    }

    impl From<(f32, f32)> for Vec2 {
        fn from(value: (f32, f32)) -> Self {
            Self {
                x: value.0,
                y: value.1,
            }
        }
    }
}

mod ops {
    use super::*;

    impl Add for Vec2 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Sub for Vec2 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }


    impl Mul<f32> for Vec2 {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self {
            Self {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    impl Div<f32> for Vec2 {
        type Output = Self;

        fn div(self, rhs: f32) -> Self::Output {
            Self {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }
}