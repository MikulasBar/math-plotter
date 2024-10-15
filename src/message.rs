
#[derive(Debug, Clone)]
pub enum Message {
    KeyPressed(iced::keyboard::Key),
    UpdateView(glam::Vec2, f32),
}
