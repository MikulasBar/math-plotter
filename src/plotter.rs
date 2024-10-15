mod primitive;
mod events;
mod scene;
mod element;
mod render_state;


use crate::message::Message;
use scene::Scene;
use iced::widget::{shader, Shader};

// use builder::{Builder, Unsized};

#[derive(Default)]
pub struct Plotter {
    scene: Scene
}

impl Plotter {
    pub fn new() -> Plotter {
        Plotter::default()
    }
}


impl Plotter {
    pub fn get_widget(&self) -> Shader<Message, &Scene> {
        shader(&self.scene)
    }

    pub fn update_view(&mut self, offset: glam::Vec2, zoom: f32) {
        self.scene.offset = offset;
        self.scene.zoom = zoom;
    }

    pub fn with_size(&self, width: f32, height: f32) -> Shader<Message, &Scene> {
        self.get_widget()
            .width(width)
            .height(height)
    }
}
