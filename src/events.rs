use smol_str::SmolStr;

#[rustfmt::skip]
use iced::{
    self,
    mouse::{self, Button, Event as MouseEvent},
    widget::canvas::{Event as CanvasEvent},
    keyboard::{
        self,
        key::Key,
        Location as KeyLocation,
        Modifiers as KeyboardModifiers,
        Event as KeyboardEvent,
        
    },
};


#[derive(Debug, Clone)]
pub enum Message {
    Redraw,
}

/// A canvas event that represents a left button press
pub const CANVAS_LEFT_BUTTON_PRESSED: CanvasEvent = CanvasEvent::Mouse(
    MouseEvent::ButtonPressed(Button::Left)
);

pub const CANVAS_KEY_R_PRESSED: CanvasEvent = CanvasEvent::Keyboard(
    KeyboardEvent::KeyPressed {
        key: KEY_R,
        location: KeyLocation::Standard,
        modifiers: KeyboardModifiers::empty(),
        text: Some(TEXT_R),
    }
);


const KEY_R: Key = Key::Character(TEXT_R);
const TEXT_R: SmolStr = SmolStr::new_static("r");

