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
macro_rules! mouse_event {
    (MOVE: $point:ident) => {
        MouseEvent::CursorMoved {
            position: $point,
        }
    }; 
    (LEFT_DOWN) => {
        MouseEvent::ButtonPressed(MouseButton::Left)
    };
    (LEFT_UP) => {
        MouseEvent::ButtonReleased(MouseButton::Left)
    };
    (SCROLL: $delta:ident) => {
        MouseEvent::WheelScrolled {
            delta: $delta
        }
    };
}

#[macro_export]
macro_rules! event {
    (MOUSE $($tt:tt)*) => {
        CanvasEvent::Mouse(
            mouse_event!($($tt)*)
        )
    };
}

