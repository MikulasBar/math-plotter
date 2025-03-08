mod axis;
mod background;
mod graph;
mod helpers;

use iced::widget::shader::wgpu;

pub struct RenderState {
    pub background: background::State,
    pub graph: graph::State,
    pub axis: axis::State,
}

impl RenderState {
    pub fn new(device: &wgpu::Device, graphs: &[Vec<f32>], axises: &[f32]) -> Self {
        let background = background::State::new(device);
        let graph = graph::State::new(device, graphs);
        let axis = axis::State::new(device, axises);

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
        bounds: iced::Rectangle<u32>,
    ) {
        self.background.render(encoder, target, bounds);
        self.axis.render(encoder, target, bounds);
        self.graph.render(encoder, target, bounds);
    }

    pub fn update_data(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        buffers: &[Vec<f32>],
        axises: &[f32],
    ) {
        self.axis.update_buffers(queue, axises);
        self.graph.update_buffers(device, queue, buffers);
    }
}
