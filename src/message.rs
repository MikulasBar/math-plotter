
use crate::plotter::view::View;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateView(View),
    InputChanged(String),
    InputSubmitted,
}
