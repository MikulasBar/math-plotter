use glam::Vec2;
use iced::widget::shader::{self};
use iced::advanced::graphics::Viewport;
use super::render_state::RenderState;
#[derive(Clone, Debug)]
pub struct Primitive {
    buffers: Vec<Vec<f32>>,
    offset: glam::Vec2,
}

impl Primitive {
    pub fn new(buffers: Vec<Vec<f32>>, offset: Vec2) -> Self {
        Primitive {
            buffers,
            offset,
        }
    }
}

impl shader::Primitive for Primitive {
    fn prepare(
        &self,
        device: &shader::wgpu::Device,
        queue: &shader::wgpu::Queue,
        _format: shader::wgpu::TextureFormat,
        storage: &mut shader::Storage,
        _bounds: &iced::Rectangle,
        _viewport: &Viewport,
    ) {
        if !storage.has::<RenderState>() {
            let render_state = RenderState::new(device, &self.buffers, self.offset);
            storage.store(render_state);
            return;
        }

        let render_state = storage.get_mut::<RenderState>().unwrap();
        render_state.update_data(device, queue, &self.buffers, self.offset);
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
            // 0..self.vertex_count(),
        );
    }
}


