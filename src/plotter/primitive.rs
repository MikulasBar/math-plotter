use iced::widget::shader::{self};
use super::render_state::RenderState;

#[derive(Debug, Clone)]
pub struct Primitive {
    buffer: Vec<f32>,
}

impl Primitive {
    pub fn new(buffer: Vec<f32>) -> Self {
        Primitive {
            buffer
        }
    }
}


impl shader::Primitive for Primitive {
    fn prepare(
        &self,
        device: &shader::wgpu::Device,
        _queue: &shader::wgpu::Queue,
        _format: shader::wgpu::TextureFormat,
        storage: &mut shader::Storage,
        _bounds: &iced::Rectangle,
        viewport: &iced::advanced::graphics::Viewport,
    ) {
        if !storage.has::<RenderState>() {
            let render_state = RenderState::new(device);
            storage.store(render_state);
        } else {
            let render_state = storage.get_mut::<RenderState>().unwrap();
            render_state.graph.update_buffer(device, &self.buffer);
        }
    }

    fn render(
        &self,
        encoder: &mut shader::wgpu::CommandEncoder,
        storage: &shader::Storage,
        target: &shader::wgpu::TextureView,
        viewport: &iced::Rectangle<u32>,
    ) {
        let render_state = storage.get::<RenderState>().unwrap();

        render_state.render(
            encoder,
            target,
            *viewport,
            0..0,
            // 0..2 * (Self::RANGE as u32),
        );
    }
}