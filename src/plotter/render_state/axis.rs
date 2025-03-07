use glam::Vec2;
use iced::widget::shader::wgpu::{self, BufferUsages, Device, LoadOp, Queue, ShaderStages, StoreOp};

use super::helpers::*;

pub struct State {
    pipeline: wgpu::RenderPipeline,
    buffer: wgpu::Buffer,
    offset_buffer: wgpu::Buffer,
    config_group: wgpu::BindGroup,
}

impl State {
    const POINTS: [f32; 8] = [-1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 1.0];
    const COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

    pub fn new(device: &Device, offset: Vec2) -> Self {
        let shader_module = super::helpers::shader_module(
            device,
            "axis:shader_module",
            include_str!("shaders/axis.wgsl"),
        );

        let buffer = buffer_init(device, "axis:buffer", BufferUsages::VERTEX, &Self::POINTS);

        let offset_buffer = buffer_init(
            device,
            "axis:offset_buffer",
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            &[offset.x, offset.y],
        );

        let color_buffer = buffer_init(
            device,
            "axis:color_buffer",
            BufferUsages::UNIFORM,
            &Self::COLOR,
        );

        let (config_group, config_group_layout) =
            BindGroupBuilder::new(device, "axis:config_group")
                .add_entry(0, ShaderStages::VERTEX, None, &offset_buffer)
                .add_entry(1, ShaderStages::FRAGMENT, None, &color_buffer)
                .build();

        let pipeline = PipelineBuilder::new(device)
            .label("axis:pipeline")
            .vertex(&shader_module, "vs_main", &[VERTEX2D_VERTEX_LAYOUT])
            .layout("axis:pipeline_layout", &[&config_group_layout])
            .fragment(
                &shader_module,
                "fs_main",
                &[Some(STANDARD_COLOR_TARGET_STATE)],
            )
            .primitive(wgpu::PrimitiveTopology::LineList)
            .multisample(STANDARD_MULTISAMPLE_STATE)
            .build();

        Self {
            pipeline,
            buffer,
            offset_buffer,
            config_group,
        }
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
        render_pass.set_bind_group(0, &self.config_group, &[]);
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

    pub fn update_offset(&mut self, queue: &Queue, bounds: &iced::Rectangle<f32>, offset: Vec2) {
        let scale_x = 2.0 / bounds.width;
        let scale_y = 2.0 / bounds.height;
        let offset_data = [offset.x * scale_x, -offset.y * scale_y];
        
        queue.write_buffer(&self.offset_buffer, 0, bytemuck::cast_slice(&offset_data));
    }
}
