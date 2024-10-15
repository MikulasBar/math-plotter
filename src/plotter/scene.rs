use super::element::Element;
use super::events::*;
use crate::message::Message;
use crate::utilities::GlamVec2Ext;
use super::primitive::Primitive;
use glam::Vec2;
use iced::widget::shader::{self, Event as ShaderEvent};
use iced::{advanced, mouse};
use iced::event::Status as EventStatus;
use iced::mouse::ScrollDelta;


pub struct Scene {
    elements: Vec<Element>,
    pub offset: Vec2,
    pub zoom: f32,
}

impl Scene {
    const RANGE: i32 = 100;
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            elements: Vec::new(),
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
        _state: &Self::State,
        _cursor: mouse::Cursor,
        bounds: iced::Rectangle,
    ) -> Self::Primitive {
        let range = Self::RANGE as f32;

        let off_x = 2.0 * self.offset.x / bounds.width as f32;
        let off_y = 2.0 * self.offset.y / bounds.height as f32;

        let buffer: Vec<f32> = (-Self::RANGE..Self::RANGE)
            .map(|x| x as f32)
            .map(|x| x / range)
            .flat_map(|x| {
                let fx = (x - off_x).sin(); 
                let y = fx - off_y;
                [x, y]
            })
            .collect();

        Primitive::new(buffer)
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: iced::widget::shader::Event,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
        _shell: &mut advanced::Shell<'_, Message>,
    ) -> (EventStatus, Option<Message>) {
        // if !cursor.is_over(bounds) {
        //     // *state = State::Idle;
        //     return (EventStatus::Captured, None);
        // }

        match event {
            event!(KB PRESS: key) => {
                return (EventStatus::Captured, Some(Message::KeyPressed(key)))
            },

            event!(MOUSE LEFT_DOWN) => {
                if !cursor.is_over(bounds) {
                    return (EventStatus::Ignored, None);
                }
                
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
                        let new_offset = self.offset + offset;

                        // update the cursor position
                        *state = State::LeftButtonDown(new_pos);
                        

                        return (EventStatus::Captured, Some(Message::UpdateView(new_offset, self.zoom)));
                    },
                    _ => return (EventStatus::Ignored, None),
                }
            },

            event!(MOUSE SCROLL: delta) => {
                let zoom = self.zoom + delta_to_zoom(delta);
                return (EventStatus::Captured, Some(Message::UpdateView(self.offset, zoom)));
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


fn delta_to_zoom(delta: ScrollDelta) -> f32 {
    match delta {
        ScrollDelta::Lines { y, .. } => y,
        ScrollDelta::Pixels { y, .. } => y,
    }
}