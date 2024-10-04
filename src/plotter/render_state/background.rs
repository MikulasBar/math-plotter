use iced::widget::shader::wgpu::{self, BufferUsages, Device, LoadOp, StoreOp, PrimitiveTopology};
use super::builders::*;

pub struct State {
    pipeline: wgpu::RenderPipeline,
    buffer: wgpu::Buffer,
    color_bind: wgpu::BindGroup,
}

impl State {
    const DEFAULT_COLOR:    [u8; 4] = [0x36, 0x39, 0x3F, 0xFF];
    const BLUE:             [u8; 4] = [0x00, 0x00, 0xFF, 0xFF];
    const CORNERS: [f32; 8] = [
        -1.0, -1.0,
        -1.0, 1.0,
        1.0, -1.0,
        1.0, 1.0,
    ];

    pub fn new(device: &Device) -> Self {
        let shader_module = shader_module(
            device, 
            "background-shader-module", 
            include_str!("shaders/background.wgsl")
        );

        let color = Self::BLUE.into_iter().map(f32::from).collect::<Vec<f32>>();
        let (color_bind, color_bind_layout) = single_entry_bind_group(
            device,
            "background-color-bind-group",
            0,
            &color,
        );

        let buffer = buffer_init(
            device, 
            "background-buffer", 
            BufferUsages::VERTEX, 
            &Self::CORNERS
        );

        let pipeline = PipelineBuilder::new(device)
            .label("background-pipeline")
            .layout("background-pipeline-layout", &[&color_bind_layout])
            .vertex(&shader_module, "vs_main", &[VERTEX2D_VERTEX_LAYOUT])
            .fragment(&shader_module, "fs_main", &[Some(STANDARD_COLOR_TARGET_STATE)])
            .primitive(PrimitiveTopology::TriangleStrip)
            .multisample(STANDARD_MULTISAMPLE_STATE)
            .build();
        
        Self {
            pipeline,
            buffer,
            color_bind,
        }
    }

    pub fn render(
        &self, 
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::TextureView,
        viewport: iced::Rectangle<u32>,
    ) {
        let mut render_pass = RenderPassBuilder::new()
            .label("background-render-pass")
            .color_attachment(frame, LoadOp::Load, StoreOp::Store)
            .build(encoder);

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.buffer.slice(..));
        render_pass.set_bind_group(0, &self.color_bind, &[]);
        render_pass.set_scissor_rect(
            viewport.x, 
            viewport.y, 
            viewport.width, 
            viewport.height
        );
        render_pass.draw(0..4, 0..1);
    }
}