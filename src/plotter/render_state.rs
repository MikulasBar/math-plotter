mod graph;
mod background;
mod helpers;

use iced::widget::shader::wgpu::{self};

pub struct RenderState {
    pub background: background::State,
    pub graph: graph::State,
}


impl RenderState {
    pub fn new(device: &wgpu::Device, buffer: &[f32]) -> Self {
        let background = background::State::new(device);
        let graph = graph::State::new(device, buffer);

        Self {
            background,
            graph,
        }
    }

    pub fn render(
        &self, 
        encoder: &mut wgpu::CommandEncoder, 
        target: &wgpu::TextureView, 
        bounds: iced::Rectangle<u32>, 
        graph_range: std::ops::Range<u32>)
    {
        self.background.render(encoder, target, bounds);
        self.graph.render(encoder, target, bounds, graph_range);
    }
}

