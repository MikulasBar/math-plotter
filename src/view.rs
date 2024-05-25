use crate::{
    vector::Vec2,
};


pub struct View {
    offset: Vec2,
    // zoom: f32,
}

impl Default for View {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            // zoom: 1.0
        }
    }
}