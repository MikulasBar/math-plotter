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

        let f = |x: f32| x.sin();

        // these formulas are derived from the previous commits on the main branch :D
        let buffer: Vec<[f32; 2]> = (-Self::RANGE..Self::RANGE)
            .map(|x| x as f32)
            .map(|x| x / range)
            .map(|x| {
                let fx = f((x - off_x) / self.zoom); 
                let y = fx * self.zoom - off_y;
                [x, y]
            })
            .collect();

        // TODO: optimize this, additive vector is allocated, pass only the iterator if possible
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
            // event!(KB PRESS: key) => {
            //     return (EventStatus::Captured, Some(Message::KeyPressed(key)))
            // },

            event!(MOUSE LEFT_DOWN) if cursor.is_over(bounds) => {
                let vector = Vec2::from_point(cursor.position().unwrap());
                *state = State::LeftButtonDown(vector);
            },

            event!(MOUSE LEFT_UP) => {
                *state = State::Idle;
            },

            event!(MOUSE MOVE: new_pos) => {
                let State::LeftButtonDown(start) = state else {return (EventStatus::Ignored, None)};

                let new_pos = Vec2::from_point(new_pos);
                let offset = new_pos - *start;
                let new_offset = self.offset + offset;

                *state = State::LeftButtonDown(new_pos);

                return (EventStatus::Captured, Some(Message::UpdateView(new_offset, self.zoom)));
            },

            event!(MOUSE SCROLL: delta) if cursor.is_over(bounds) => {
                // cursor is relative to the center of the bounds
                let cursor_pos = Vec2::from_iced_vec(cursor.position_over(bounds).unwrap() - bounds.center()) - self.offset;

                // zoom is used as scale factor so we use the zoom = zoom*(1 - c*delta)
                // instead of zoom = zoom + c*delta (also zoom would go negative)
                let new_zoom = self.zoom * (1.0 + delta_to_zoom(delta));
                // TODO: make documetation for this formula
                let new_offset = self.offset + cursor_pos * (1.0 - new_zoom / self.zoom); 
                return (EventStatus::Captured, Some(Message::UpdateView(new_offset, new_zoom)));
            },

            _ => (),
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
        ScrollDelta::Lines { y, .. } => y * 0.1,
        ScrollDelta::Pixels { y, .. } => y * 1.0,
    }
}