use iced::widget::shader::wgpu::{self, BufferUsages, LoadOp, ShaderStages, StoreOp};
use super::helpers::*;

pub struct State {
    pipeline: wgpu::RenderPipeline,
    buffers: Vec<wgpu::Buffer>,
    color_group: wgpu::BindGroup,
}


impl State {
    const BLUE: [u8; 4] = [0x00, 0x00, 0xFF, 0xFF];

    pub fn new(device: &wgpu::Device, buffers: &[Vec<f32>]) -> Self {
        let shader_module = shader_module(device, "graph:shader_module", include_str!("shaders/graph.wgsl"));
        let buffers = init_buffers(device, buffers);

        let (color_group, color_group_layout) = BindGroupBuilder::new(device, "graph:color_group")
            .add_entry(
                "graph:color_group:color",
                0,
                BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                ShaderStages::FRAGMENT,
                None,
                &color_to_f32(Self::BLUE),
            )
            .build();

        let pipeline = PipelineBuilder::new(device)
            .label("graph:pipeline")
            .layout("graph:pipeline_layout", &[&color_group_layout])
            .vertex(&shader_module, "vs_main", &[VERTEX2D_VERTEX_LAYOUT])
            .fragment(&shader_module, "fs_main", &[Some(STANDARD_COLOR_TARGET_STATE)])
            .primitive(wgpu::PrimitiveTopology::LineStrip)
            .multisample(STANDARD_MULTISAMPLE_STATE)
            .build();

        Self {
            pipeline,
            buffers,
            color_group,
        }
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        bounds: iced::Rectangle<u32>,
        // vertex_range: std::ops::Range<u32>,
    ) {
        let mut render_pass = RenderPassBuilder::new()
            .label("graph:render_pass")
            .color_attachment(target, LoadOp::Load, StoreOp::Store)
            .build(encoder);

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.color_group, &[]);
        // render_pass.set_vertex_buffer(0, self.buffer.slice(..));
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

        // render_pass.draw(vertex_range, 0..1);

        for buffer in &self.buffers {
            render_pass.set_vertex_buffer(0, buffer.slice(..));
            let vertex_count = (buffer.size() / std::mem::size_of::<[f32; 2]>() as u64) as u32;
            render_pass.draw(0..vertex_count, 0..1);
        }
    }

    pub fn update_buffers(&mut self, device: &wgpu::Device, buffers: &[Vec<f32>]) {
        self.buffers = init_buffers(device, buffers);
    }
}

fn init_buffers(device: &wgpu::Device, buffers: &[Vec<f32>]) -> Vec<wgpu::Buffer> {
    buffers.iter()
        .enumerate()
        .map(|(i, b)| {
            buffer_init(device, &format!("graph:buffer:{}", i), BufferUsages::VERTEX, b)
        })
        .collect()
}