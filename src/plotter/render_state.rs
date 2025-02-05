mod graph;
mod background;
mod helpers;

use iced::widget::shader::wgpu::{self};

pub struct RenderState {
    pub background: background::State,
    pub graph: graph::State,
}


impl RenderState {
    pub fn new(device: &wgpu::Device, buffers: &[Vec<f32>]) -> Self {
        let background = background::State::new(device);
        let graph = graph::State::new(device, buffers);

        Self {
            background,
            graph,
        }
    }
    
    pub fn render(
        &self, 
        encoder: &mut wgpu::CommandEncoder, 
        target: &wgpu::TextureView, 
        bounds: iced::Rectangle<u32>
    ) {
        self.background.render(encoder, target, bounds);
        self.graph.render(encoder, target, bounds);
    }
}

