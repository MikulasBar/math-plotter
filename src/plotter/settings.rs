use iced::Color;



#[derive(Debug, Clone)]
pub struct Settings {
    pub axis: Color,
    pub background: Color,
}

impl Settings {
    // pub fn new() -> Self {
    //     Self::default()
    // }

    // pub fn set_axis(mut self, axis: Color) -> Self {
    //     self.axis = axis;
    //     self
    // }

    // pub fn set_background(mut self, background: Color) -> Self {
    //     self.background = background;
    //     self
    // }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            axis: AXIS_COLOR,
            background: BG_COLOR,
        }
    }
}

const BG_COLOR: Color = Color::from_rgb(
    0x36 as f32 / 255.0,
    0x39 as f32 / 255.0,
    0x3F as f32 / 255.0,
);

const AXIS_COLOR: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
);
