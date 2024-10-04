use super::element::Element;
use super::events::*;
use super::imports::Message;
use crate::utilities::GlamVec2Ext;
use super::primitive::Primitive;
use glam::Vec2;
use std::sync::{Arc, Mutex};
use iced::advanced::graphics::core::event;
use iced::widget::shader::{self, Event as ShaderEvent};
use iced::{advanced, mouse};
use iced::event::Status as EventStatus;


pub struct Scene {
    elements: Arc<Mutex<Vec<Element>>>,
    pub offset: Vec2,
    pub zoom: f32,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            elements: Arc::new(Mutex::new(Vec::new())),
            offset: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl shader::Program<Message> for Scene {
    type State = State;
    type Primitive = Primitive;

    fn draw(
        &self,
        state: &Self::State,
        cursor: mouse::Cursor,
        bounds: iced::Rectangle,
    ) -> Self::Primitive {
        Primitive::new(self.elements.clone(), self.offset, self.zoom)
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: iced::widget::shader::Event,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
        _shell: &mut advanced::Shell<'_, Message>,
    ) -> (EventStatus, Option<Message>) {
        if !cursor.is_over(bounds) {
            return (EventStatus::Ignored, None);
        }

        match event {
            event!(KB PRESS: key) => {
                return (EventStatus::Captured, Some(Message::KeyPressed(key)))
            },

            event!(MOUSE LEFT_DOWN) => {
                let vector = Vec2::from_point(cursor.position().unwrap());
                *state = State::LeftButtonDown(vector);
            },

            event!(MOUSE LEFT_UP) => {
                *state = State::Idle;
            },

            event!(MOUSE MOVE: new_pos) => {
                match state {
                    State::LeftButtonDown(start) => {
                        let new_pos = Vec2::from_point(new_pos);
                        let offset = new_pos - *start;

                        // println!("Offset: {:?}", offset);
                        // println!("NewPos: {:?}", new_pos);

                        // update the cursor position
                        *state = State::LeftButtonDown(new_pos);

                        return (EventStatus::Captured, Some(Message::UpdateView(offset)));
                    },
                    _ => return (EventStatus::Ignored, None),
                }
            },

            _ => return (EventStatus::Ignored, None),
        }


        (EventStatus::Ignored, None)
    }
}

pub enum State {
    Idle,
    LeftButtonDown(Vec2),
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}