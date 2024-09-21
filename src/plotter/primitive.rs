use std::sync::{Arc, Mutex};
use iced::{widget::shader::{self}};
use super::{element::Element};
use super::render_pipeline::RenderPipeline;
use glam::Mat2;

#[derive(Debug, Clone)]
pub struct Primitive {
    elements: Arc<Mutex<Vec<Element>>>,
    transform: Mat2,
}

impl Primitive {
    pub fn new(elements: Arc<Mutex<Vec<Element>>>, transform: Mat2) -> Self {
        Primitive {
            elements,
            transform,
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
        // Compute coords
        // Create the pipeline
        // store the pipeline

        
        if !storage.has::<RenderPipeline>() {
            let render_pipeline = RenderPipeline::new(device, &[]);
            storage.store(render_pipeline);
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
        let render_pipeline = storage.get::<RenderPipeline>().unwrap();

        render_pipeline.render(
            encoder,
            target,
            viewport,
            0..0,
        );
    }
}