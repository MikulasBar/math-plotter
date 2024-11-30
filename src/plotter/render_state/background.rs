use super::helpers::*;
use iced::widget::shader::wgpu::{
    self, BufferUsages, Device, LoadOp, PrimitiveTopology, ShaderStages, StoreOp,
};


// We need to render background manually.
// LoadOp::Clear cleares the whole screen no matter how the viewport is set.
// So we need to draw a rectangle with the background color. 

pub struct State {
    pipeline: wgpu::RenderPipeline,
    buffer: wgpu::Buffer,
    color_group: wgpu::BindGroup,
}

impl State {
    const DEFAULT_COLOR: [u8; 4] = [0x36, 0x39, 0x3F, 0xFF];
    const BLUE: [u8; 4] = [0x00, 0x00, 0xFF, 0xFF];
    const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];
    const POINTS: [f32; 8] = [-1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0];

    pub fn new(device: &Device) -> Self {
        let shader_module = shader_module(
            device,
            "background:shader_module",
            include_str!("shaders/background.wgsl"),
        );

        let (color_group, color_group_layout) =
            BindGroupBuilder::new(device, "background:color_group")
                .add_entry(
                    "background:color_group:color",
                    0,
                    BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                    ShaderStages::FRAGMENT,
                    None,
                    &color_to_f32(Self::DEFAULT_COLOR),
                )
                .build();

        let buffer = buffer_init(
            device,
            "background:buffer",
            BufferUsages::VERTEX,
            &Self::POINTS,
        );

        let pipeline = PipelineBuilder::new(device)
            .label("background:pipeline")
            .layout("background:pipeline_layout", &[&color_group_layout])
            .vertex(&shader_module, "vs_main", &[VERTEX2D_VERTEX_LAYOUT])
            .fragment(
                &shader_module,
                "fs_main",
                &[Some(STANDARD_COLOR_TARGET_STATE)],
            )
            .primitive(PrimitiveTopology::TriangleStrip)
            .multisample(STANDARD_MULTISAMPLE_STATE)
            .build();

        Self {
            pipeline,
            buffer,
            color_group,
        }
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        bounds: iced::Rectangle<u32>,
    ) {
        let mut render_pass = RenderPassBuilder::new()
            .label("background:render_pass")
            .color_attachment(target, LoadOp::Load, StoreOp::Store)
            .build(encoder);

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.buffer.slice(..));
        render_pass.set_bind_group(0, &self.color_group, &[]);
        render_pass.set_scissor_rect(bounds.x, bounds.y, bounds.width, bounds.height);
        // render_pass.set_viewport(
        //     bounds.x as f32,
        //     bounds.y as f32,
        //     bounds.width as f32,
        //     bounds.height as f32,
        //     0.0,
        //     1.0,
        // );
        render_pass.draw(0..4, 0..1);
    }
}
