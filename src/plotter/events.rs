// #[rustfmt::skip]
// use iced::{
//     self,
//     Point,
//     mouse::{self, Button as MouseButton, Event as MouseEvent, ScrollDelta},
//     widget::canvas::{Event as CanvasEvent},
// };

// #[rustfmt::skip]
// use crate::{
//     plotter::view::View,
// };



#[macro_export]
macro_rules! event {
    (MOUSE_MOVE: $point:ident) => {
        CanvasEvent::Mouse(
            MouseEvent::CursorMoved {
                position: $point,
            }
        )
    };
    (MOUSE_LEFT_DOWN) => {
        CanvasEvent::Mouse(
            MouseEvent::ButtonPressed(MouseButton::Left)
        )
    };
    (MOUSE_LEFT_UP) => {
        CanvasEvent::Mouse(
            MouseEvent::ButtonReleased(MouseButton::Left)
        )
    };
    (MOUSE_SCROLL: $delta:ident) => {
        CanvasEvent::Mouse(
            MouseEvent::WheelScrolled {
                delta: $delta
            }
        )
    };
}
