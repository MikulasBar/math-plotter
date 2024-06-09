use iced::mouse::ScrollDelta;

use crate::{
    vector::Vec2,
};


#[derive(Debug, Clone)]
pub struct View {
    pub offset: Vec2,
    pub zoom: f32,
}

impl View {
    pub fn new(offset: Vec2, zoom: f32) -> Self {
        Self {
            offset,
            zoom,
        }
    }

    pub fn zoom_from_delta(delta: ScrollDelta) -> f32 {
        match delta {
            ScrollDelta::Lines { y, .. } => y * 0.1,
            ScrollDelta::Pixels { y, .. } => y * 0.01,
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            zoom: 1.0
        }
    }
}