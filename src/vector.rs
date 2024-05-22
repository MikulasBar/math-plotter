use iced::Point;
use std::ops::{Add, Sub, AddAssign, SubAssign};

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self{x, y}
    }
}

impl Into<Point> for Vec2 {
    fn into(self) -> Point {
        Point{x: self.x, y: self.y}
    }
}

impl From<Point> for Vec2 {
    fn from(point: Point) -> Self {
        Self{x: point.x, y: point.y}
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

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

