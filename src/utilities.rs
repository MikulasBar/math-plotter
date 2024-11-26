

pub trait GlamVec2Ext {
    fn to_point(&self) -> iced::Point;
    fn to_iced_vec(&self) -> iced::Vector;
    fn from_point(point: iced::Point) -> Self;
    fn from_iced_vec(vector: iced::Vector) -> Self;
}

impl GlamVec2Ext for glam::Vec2 {
    fn to_point(&self) -> iced::Point {
        iced::Point {
            x: self.x,
            y: self.y,
        }
    }

    fn to_iced_vec(&self) -> iced::Vector {
        iced::Vector {
            x: self.x,
            y: self.y,
        }
    }

    fn from_point(point: iced::Point) -> Self {
        glam::Vec2::new(point.x, point.y)
    }

    fn from_iced_vec(vector: iced::Vector) -> Self {
        glam::Vec2::new(vector.x, vector.y)
    }
}

