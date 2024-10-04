use rand::random;

/// returns a random f32 between -1.0 and 1.0
pub fn rnd_signed() -> f32 {
    let sign = if random::<bool>() { 1.0 } else { -1.0 };
    sign * random::<f32>()
}

// pub fn rnd_color() -> Color {
//     Color {
//         r: random::<f32>(),
//         g: random::<f32>(),
//         b: random::<f32>(),
//         a: 1.0,
//     }
// }

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