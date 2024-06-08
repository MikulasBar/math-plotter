use smol_str::SmolStr;

#[rustfmt::skip]
use iced::{
    self,
    Point,
    mouse::{self, Button as MouseButton, Event as MouseEvent},
    widget::canvas::{Event as CanvasEvent},
    keyboard::{
        self,
        key::Key,
        Location as KeyLocation,
        Modifiers as KeyboardModifiers,
        Event as KeyboardEvent,
        
    },
};
use crate::vector::Vec2;

#[derive(Debug, Clone)]
pub enum Message {
    Redraw,
    Translate(Vec2)
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




// pub const CANVAS_KEY_R_PRESSED: CanvasEvent = CanvasEvent::Keyboard(
//     KeyboardEvent::KeyPressed {
//         key: KEY_R,
//         location: KeyLocation::Standard,
//         modifiers: KeyboardModifiers::empty(),
//         text: Some(TEXT_R),
//     }
// );


// const KEY_R: Key = Key::Character(TEXT_R);
// const TEXT_R: SmolStr = SmolStr::new_static("r");
