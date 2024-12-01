use iced::widget::shader::wgpu::{self, BufferUsages, LoadOp, StoreOp};
use super::helpers::*;

pub struct State {
    pipeline: wgpu::RenderPipeline,
    buffer: wgpu::Buffer,
}


impl State {
    pub fn new(device: &wgpu::Device, vertices: &[f32]) -> Self {
        let shader_module = shader_module(device, "graph:shader_module", include_str!("shaders/graph.wgsl"));
        let buffer = buffer_init(device, "graph:buffer", BufferUsages::VERTEX, vertices);

        let pipeline = PipelineBuilder::new(device)
            .label("graph:pipeline")
            .layout("graph:pipeline_layout", &[])
            .vertex(&shader_module, "vs_main", &[VERTEX2D_VERTEX_LAYOUT])
            .fragment(&shader_module, "fs_main", &[Some(STANDARD_COLOR_TARGET_STATE)])
            .primitive(wgpu::PrimitiveTopology::LineStrip)
            .multisample(STANDARD_MULTISAMPLE_STATE)
            .build();

        Self {
            pipeline,
            buffer,
        }
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        bounds: iced::Rectangle<u32>,
        vertex_range: std::ops::Range<u32>,
    ) {
        let mut render_pass = RenderPassBuilder::new()
            .label("graph:render_pass")
            .color_attachment(target, LoadOp::Load, StoreOp::Store)
            .build(encoder);

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.buffer.slice(..));
        render_pass.set_scissor_rect(
            bounds.x,
            bounds.y,
            bounds.width, 
            bounds.height
        );

        // Viewport set is mandatory for the shader to work
        // Without it th shader will draw thing on the whole screen
        // Note that this doesn't fix the LoadOp::Clear issue
        render_pass.set_viewport(
            bounds.x as f32,
            bounds.y as f32, 
            bounds.width as f32,
            bounds.height as f32,
            0.0,
            1.0
        );

        render_pass.draw(vertex_range, 0..1);
    }

    pub fn update_buffer(&mut self, device: &wgpu::Device, buffer: &[f32]) {
        self.buffer = buffer_init(device, "graph:buffer", BufferUsages::VERTEX, buffer);
    }
}