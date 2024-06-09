use smol_str::SmolStr;

#[rustfmt::skip]
use iced::{
    self,
    Point,
    mouse::{self, Button as MouseButton, Event as MouseEvent},
    widget::canvas::{Event as CanvasEvent},
};
use crate::vector::Vec2;
use crate::view::View;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateView(View)
}

#[macro_export]
macro_rules! event {
    (MOUSE_MOVE: $point:ident) => {
        CanvasEvent::Mouse(
            MouseEvent::CursorMoved {
                position: $point,
            }
        )
    };
    (LEFT_BUTTON_PRESSED) => {
        CanvasEvent::Mouse(
            MouseEvent::ButtonPressed(MouseButton::Left)
        )
    };
    (LEFT_BUTTON_RELEASED) => {
        CanvasEvent::Mouse(
            MouseEvent::ButtonReleased(MouseButton::Left)
        )
    };
}