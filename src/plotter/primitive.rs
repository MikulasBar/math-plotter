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
        _format: shader::wgpu::TextureFormat,
        device: &shader::wgpu::Device,
        _queue: &shader::wgpu::Queue,
        _bounds: iced::Rectangle,
        _target_size: iced::Size<u32>,
        _scale_factor: f32,
        storage: &mut shader::Storage,
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
        storage: &shader::Storage,
        target: &shader::wgpu::TextureView,
        _target_size: iced::Size<u32>,
        viewport: iced::Rectangle<u32>,
        encoder: &mut shader::wgpu::CommandEncoder,
    ) {
        let render_state = storage.get::<RenderState>().unwrap();

        render_state.render(
            encoder,
            target,
            viewport,
            0..0,
            // 0..2 * (Self::RANGE as u32),
        );
    }
}