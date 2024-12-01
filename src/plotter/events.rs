// pub use iced::widget::shader::Event as ShaderEvent;
// pub use iced::keyboard::Event as KeyboardEvent;
pub use iced::mouse::Event as MouseEvent;
pub use iced::mouse::Button as MouseButton;

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

// macro_rules! keyboard_event {
//     (PRESS: $key:ident) => {
//         KeyboardEvent::KeyPressed {
//             key: $key,
//             ..
//         }
//     };
// }


macro_rules! event {
    (MOUSE $($tt:tt)*) => {
        ShaderEvent::Mouse(
            mouse_event!($($tt)*)
        )
    };

    (KB $($tt:tt)*) => {
        ShaderEvent::Keyboard(
            keyboard_event!($($tt)*)
        )
    };
}


pub(super) use {
    event,
    mouse_event,
    // keyboard_event,
};
