use super::element::Element;
use super::primitive::Primitive;
use std::sync::{Arc, Mutex};
use glam::Mat2;
use iced::widget::shader::{self};
use iced::mouse;

pub struct Scene {
    elements: Arc<Mutex<Vec<Element>>>,
    transform: Mat2,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            elements: Arc::new(Mutex::new(Vec::new())),
            transform: Mat2::IDENTITY,
        }
    }
}

impl<M> shader::Program<M> for Scene {
    type State = ();

    type Primitive = Primitive;

    fn draw(
        &self,
        state: &Self::State,
        cursor: mouse::Cursor,
        bounds: iced::Rectangle,
    ) -> Self::Primitive {
        Primitive::new(self.elements.clone(), self.transform)
    }
}