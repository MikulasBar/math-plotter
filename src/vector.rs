
use iced::{self, Point};
use crate::{
    utilities::rnd_signed,
    view::View,
};
use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T = f32> {
    pub x: T,
    pub y: T,
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

    /// Prepare the vector for drawing on the canvas <br>
    /// Flips the Y coordinate so the Y increases upwards <br>
    /// Translate the vector according to `view` and `origin` <br>
    /// Converts [`Vec2`] to [`Point`]
    pub fn prepare_for_drawing(&self, origin: &Vec2, view: &View) -> Point {
        let vector = self.flip_y() + *origin + view.offset;
        Point::from(vector)
    }

    /// Flips the x coordinate of the vector <br>
    /// Doesnt effect the `self` vector but returns a new vector
    pub fn flip_x(&self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }

    /// Flips the y coordinate of the vector <br>
    /// Doesnt effect the `self` vector but returns a new vector
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

// impl AddAssign for Vec2 {
//     fn add_assign(&mut self, rhs: Self) {
//         *self = *self + rhs;
//     }
// }

// impl SubAssign for Vec2 {
//     fn sub_assign(&mut self, rhs: Self) {
//         *self = *self - rhs;
//     }
// }


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