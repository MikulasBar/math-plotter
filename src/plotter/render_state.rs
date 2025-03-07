mod graph;
mod background;
mod axis;
mod helpers;

use iced::widget::shader::wgpu::{self};

pub struct RenderState {
    pub background: background::State,
    pub graph: graph::State,
    pub axis: axis::State,
}


impl RenderState {
    pub fn new(device: &wgpu::Device, buffers: &[Vec<f32>]) -> Self {
        let background = background::State::new(device);
        let graph = graph::State::new(device, buffers);
        let axis = axis::State::new(device);

        Self {
            background,
            graph,
            axis,
        }
    }
    
    pub fn render(
        &self, 
        encoder: &mut wgpu::CommandEncoder, 
        target: &wgpu::TextureView, 
        bounds: iced::Rectangle<u32>
    ) {
        self.background.render(encoder, target, bounds);
        self.axis.render(encoder, target, bounds);
        self.graph.render(encoder, target, bounds);
    }
}

