use iced::widget::shader::{self};
use iced::advanced::graphics::Viewport;
use super::render_state::RenderState;
#[derive(Debug, Clone)]
pub struct Primitive {
    buffer: Vec<[f32; 2]>,
    vertex_count: u32,
}

impl Primitive {
    pub fn new(buffer: Vec<[f32; 2]>) -> Self {
        Primitive {
            vertex_count: buffer.len() as u32,
            buffer,
        }
    }

    fn vertex_count(&self) -> u32 {
        self.vertex_count
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
        // because the viewport is the whole window, we need to scale down the graph just to fit the bounds of the widget
        let buffer = scale_to_bounds(&self.buffer, viewport, bounds);

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


// TODO: optimize this
// the vector is duplicated, i think it can be done without it
fn scale_to_bounds(buffer: &[[f32; 2]], viewport: &Viewport, bounds: &iced::Rectangle) -> Vec<f32> {
    let win_size = viewport.logical_size();
    let w_scale = bounds.width / win_size.width as f32;
    let h_scale = bounds.height / win_size.height as f32;

    buffer.iter()
        .flat_map(|[x, y]| {
            let x = x * w_scale;
            let y = y * h_scale;
            [x, y]
        })
        .collect()
}
