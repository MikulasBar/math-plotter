use iced::widget::shader::wgpu::{self, BufferUsages, Device, LoadOp, StoreOp};

use super::helpers::*;

pub struct State {
    pipeline: wgpu::RenderPipeline,
    buffer: wgpu::Buffer,
}

impl State {
    const POINTS: [f32; 8] = [-1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 1.0];

    pub fn new(device: &Device) -> Self {
        let shader_module = super::helpers::shader_module(
            device,
            "axis:shader_module",
            include_str!("shaders/axis.wgsl"),
        );
        let buffer = buffer_init(device, "axis:buffer", BufferUsages::VERTEX, &Self::POINTS);

        let pipeline = PipelineBuilder::new(device)
            .label("axis:pipeline")
            .vertex(&shader_module, "vs_main", &[VERTEX2D_VERTEX_LAYOUT])
            .fragment(
                &shader_module,
                "fs_main",
                &[Some(STANDARD_COLOR_TARGET_STATE)],
            )
            .primitive(wgpu::PrimitiveTopology::LineList)
            .multisample(STANDARD_MULTISAMPLE_STATE)
            .build();

        Self { pipeline, buffer }
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        bounds: iced::Rectangle<u32>,
    ) {
        let mut render_pass = RenderPassBuilder::new()
            .label("axis:render_pass")
            .color_attachment(target, LoadOp::Load, StoreOp::Store)
            .build(encoder);

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.buffer.slice(..));
        render_pass.set_viewport(
            bounds.x as f32,
            bounds.y as f32,
            bounds.width as f32,
            bounds.height as f32,
            0.0,
            1.0,
        );
        
        render_pass.draw(0..4, 0..1);
    }
}
