use std::ops::Deref;

use iced::widget::shader::{self};
use super::render_state::RenderState;

#[derive(Debug, Clone)]
pub struct Primitive {
    buffer: Vec<f32>,
}

impl Primitive {
    pub fn new(buffer: Vec<f32>) -> Self {
        Primitive {
            buffer,
        }
    }

    fn vertex_count(&self) -> u32 {
        self.buffer.len() as u32 / 2
    }
}

impl shader::Primitive for Primitive {
    fn prepare(
        &self,
        device: &shader::wgpu::Device,
        _queue: &shader::wgpu::Queue,
        _format: shader::wgpu::TextureFormat,
        storage: &mut shader::Storage,
        bounds: &iced::Rectangle,
        viewport: &iced::advanced::graphics::Viewport,
    ) {
        // because the viewport is the whole window, we need to scale down the graph just to fit the bounds of the widget
        let win_size = viewport.logical_size();
        let buffer: Vec<f32> = self.buffer.iter()
            .copied()
            .enumerate()
            .map(|(i, n)| {
                if i % 2 == 0 {
                    n / win_size.width as f32 * bounds.width
                } else {
                    n / win_size.height as f32 * bounds.height
                }
            })
            .collect();

        if !storage.has::<RenderState>() {
            let render_state = RenderState::new(device, &buffer);
            storage.store(render_state);
            return;
        }

        let render_state = storage.get_mut::<RenderState>().unwrap();
        render_state.graph.update_buffer(device, &buffer);
    }

    fn render(
        &self,
        encoder: &mut shader::wgpu::CommandEncoder,
        storage: &shader::Storage,
        target: &shader::wgpu::TextureView,
        bounds: &iced::Rectangle<u32>,
    ) {
        let render_state = storage.get::<RenderState>().unwrap();

        render_state.render(
            encoder,
            target,
            *bounds,
            // 0..0,
            0..self.vertex_count(),
        );
    }
}