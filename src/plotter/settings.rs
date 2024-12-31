use iced::Color;



#[derive(Debug, Clone)]
pub struct Settings {
    pub background: Color,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: BG_COLOR,
        }
    }
}

const BG_COLOR: Color = Color::from_rgb(
    0x36 as f32 / 255.0,
    0x39 as f32 / 255.0,
    0x3F as f32 / 255.0,
);
