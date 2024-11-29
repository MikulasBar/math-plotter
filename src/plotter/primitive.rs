use iced::widget::shader::{self};
use iced::advanced::graphics::Viewport;
use super::render_state::RenderState;
#[derive(Clone, Debug)]
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
        viewport: &Viewport,
    ) {
        if !storage.has::<RenderState>() {
            let render_state = RenderState::new(device, &self.buffer);
            storage.store(render_state);
            return;
        }

        let render_state = storage.get_mut::<RenderState>().unwrap();
        render_state.graph.update_buffer(device, &self.buffer);
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
            0..self.vertex_count(),
        );
    }
}


