mod primitive;
mod events;
mod scene;
mod element;
mod render_state;


use crate::message::Message;
use scene::Scene;
use iced::widget::{shader, Shader};

#[derive(Default)]
pub struct Plotter {
    scene: Scene
}

impl Plotter {
    pub fn new() -> Plotter {
        Plotter::default()
    }

    pub fn get_widget(&self) -> Shader<Message, &Scene> {
        shader(&self.scene)
    }

    pub fn update_view(&mut self, offset: glam::Vec2, zoom: f32) {
        self.scene.offset = offset;
        self.scene.zoom = zoom;
    }

    pub fn with_size<T, U>(&self, width: T, height: U) -> Shader<Message, &Scene>
    where
        T: Into<iced::Length>,
        U: Into<iced::Length>
    {
        self.get_widget()
            .width(width)
            .height(height)
    }
}
