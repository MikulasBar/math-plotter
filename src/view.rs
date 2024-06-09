use crate::{
    vector::Vec2,
};


#[derive(Debug, Clone)]
pub struct View {
    pub offset: Vec2,
    // zoom: f32,
}

impl View {
    pub fn new(offset: Vec2) -> Self {
        Self {
            offset
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            // zoom: 1.0
        }
    }
}