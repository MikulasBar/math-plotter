use std::sync::{Arc, Mutex};
use iced::widget::shader::{self};
use super::element::Element;
use super::render_state::RenderState;

#[derive(Debug, Clone)]
pub struct Primitive {
    elements: Arc<Mutex<Vec<Element>>>,
    offset: glam::Vec2,
    zoom: f32,
}

impl Primitive {
    const RANGE: i32 = 100;

    pub fn new(elements: Arc<Mutex<Vec<Element>>>, offset: glam::Vec2, zoom: f32) -> Self {
        Primitive {
            elements,
            offset,
            zoom
        }
    }
}


impl shader::Primitive for Primitive {
    fn prepare(
        &self,
        format: shader::wgpu::TextureFormat,
        device: &shader::wgpu::Device,
        queue: &shader::wgpu::Queue,
        bounds: iced::Rectangle,
        target_size: iced::Size<u32>,
        scale_factor: f32,
        storage: &mut shader::Storage,
    ) {
        let range = Self::RANGE as f32;
        // Compute coords
        // Create the pipeline
        // store the pipeline
        let glam::Vec2 {
            x: ox,
            y: oy
        } = self.offset;

        let ox = 2.0 * ox / bounds.width as f32;
        let oy = 2.0 * oy / bounds.height as f32;


        let buffer: Vec<f32> = (-Self::RANGE..Self::RANGE)
            .map(|x| x as f32)
            .map(|x| x / range)
            .flat_map(|x| {
                let f_x = (x - ox).sin(); 
                let y = f_x - oy;
                [x, y]
            })
            .collect();


        if !storage.has::<RenderState>() {
            let render_state = RenderState::new(device);
            storage.store(render_state);
        } else {
            let render_state = storage.get_mut::<RenderState>().unwrap();
            render_state.graph.update_buffer(device, &buffer);
        }
    }

    fn render(
        &self,
        storage: &shader::Storage,
        target: &shader::wgpu::TextureView,
        target_size: iced::Size<u32>,
        viewport: iced::Rectangle<u32>,
        encoder: &mut shader::wgpu::CommandEncoder,
    ) {
        let render_pipeline = storage.get::<RenderState>().unwrap();

        render_pipeline.render(
            encoder,
            target,
            viewport,
            0..0,
            // 0..2 * (Self::RANGE as u32),
        );
    }
}