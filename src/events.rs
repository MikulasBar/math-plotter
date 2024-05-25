#[rustfmt::skip]
use iced::{
    self,
    mouse::{self, Button, Event as MouseEvent},
    widget::canvas::{Event as CanvasEvent},
    touch::Event as TouchEvent,
};


#[derive(Debug, Clone)]
pub enum Message {
    Redraw,
}

/// A canvas event that represents a left button press
pub const CANVAS_LEFT_BUTTON_PRESSED: CanvasEvent = CanvasEvent::Mouse(
    MouseEvent::ButtonPressed(Button::Left)
);
