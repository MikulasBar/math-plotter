use iced::Transformation;



#[derive(Debug, Clone)]
pub enum Message {
    Transform_View(Transformation),
    InputChanged(String),
    InputSubmitted,
}
