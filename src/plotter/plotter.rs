use super::imports::*;
use super::scene::Scene;
use iced::widget::{shader, Shader, shader::Program};

use builder::{Builder, Unsized};

const BG_COLOR: Color = Color::from_rgb(
    0x36 as f32 / 255.0,
    0x39 as f32 / 255.0,
    0x3F as f32 / 255.0,
);

const AXIS_COLOR: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
);

pub struct Plotter {
    scene: Scene
}


impl Plotter {
    pub fn get_widget(&self) -> Shader<Message, &Scene> {
        shader(&self.scene)
    }

    pub fn update_view(&mut self, offset: glam::Vec2) {
        self.scene.offset += offset;
    }
}



mod builder {

    use std::marker::PhantomData;

    use super::{Plotter};

    #[rustfmt::skip]
    use super::super::imports::{
        Color,
        Size,
    };

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

impl Default for Plotter {
    fn default() -> Self {
        Plotter {
            scene: Scene::default(),
        }
    }
}