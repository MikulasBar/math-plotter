#[rustfmt::skip]
use iced::{
    Color,
};

#[derive(Debug, Clone)]
pub struct Settings {
    pub show_axes: bool,
    pub background: Color,
}

impl Settings {
    pub fn new(show_axes: bool, background: Color) -> Self {
        Self {
            show_axes,
            background,
        }
    }
}

const BG_COLOR: Color = Color::from_rgb(
    0x36 as f32 / 255.0,
    0x39 as f32 / 255.0,
    0x3F as f32 / 255.0,
);

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_axes: true,
            background: BG_COLOR,
        }
    }
}