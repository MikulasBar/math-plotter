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
}

#[allow(dead_code)]
mod builder {

    use std::marker::PhantomData;

    
    use super::Plotter;
    
    pub struct Unsized;

    pub struct Sized;

    pub struct Builder<S> {
        _marker: PhantomData<S>
    }

    impl<S> Builder<S> {
        // pub fn add_elements(mut self, elements: Vec<Element>) -> Self {
        //     self.elements.extend(elements);
        //     self
        // }

        pub fn new() -> Builder<Unsized> {
            Builder {
                _marker: PhantomData
            }
        }

        // pub fn add_sin_test(mut self) -> Self {
        //     let red_color = Color::from_rgb8(255, 0, 0);
        //     let sin = (|x: f32| x.sin()) as fn(f32) -> f32;

        //     self.elements.push((sin, red_color).into());
        //     self
        // }

        pub fn size(self, width: f32, height: f32) -> Builder<Sized> {

            Builder {
                _marker: PhantomData,
            }
        }

        // pub fn add_control_points(mut self) -> Self {
        //     let center = Element::from( (Vector::ZERO, Color::WHITE) );
        //     let right = Element::from( (Vector::UNIT_X * 100.0, Color::WHITE) );
        //     let up = Element::from( (Vector::UNIT_Y * 100.0, Color::WHITE) );
        
        //     self.elements.extend(vec![center, right, up]);

        //     self
        // }

        // pub fn background(mut self, color: Color) -> Self {
        //     self.settings.background = color;
        //     self
        // }
    }

    impl Builder<Sized> {
        pub fn build(self) -> Plotter {
            Plotter {
                ..Plotter::default()
            }
        }
    }
}

