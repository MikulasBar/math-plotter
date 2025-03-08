use iced::widget::shader;
use iced::advanced::graphics::Viewport;
use super::render_state::RenderState;
#[derive(Clone, Debug)]
pub struct Primitive {
    graphs: Vec<Vec<f32>>,
    axises: Vec<f32>,
}

impl Primitive {
    pub fn new(graphs: Vec<Vec<f32>>, axises: Vec<f32>) -> Self {
        Primitive {
            graphs,
            axises,
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
            let render_state = RenderState::new(device, &self.graphs, &self.axises);
            storage.store(render_state);
            return;
        }

        let render_state = storage.get_mut::<RenderState>().unwrap();
        render_state.update_data(device, queue, &self.graphs, &self.axises);
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
        );
    }
}


