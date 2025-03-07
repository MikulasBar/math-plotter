use iced::widget::shader::wgpu::{self, BufferUsages, LoadOp, ShaderStages, StoreOp};
use super::helpers::*;

pub struct State {
    pipeline: wgpu::RenderPipeline,
    buffers: Vec<wgpu::Buffer>,
    config_groups: Vec<wgpu::BindGroup>,
}


impl State {
    // const BLUE: [u8; 4] = [0x00, 0x00, 0xFF, 0xFF];
    const COLORS: &[[f32; 4]] = &[
        [0.0, 0.0, 1.0, 1.0], // Blue
        [0.0, 1.0, 0.0, 1.0], // Green
        [1.0, 0.0, 0.0, 1.0], // Red
        [1.0, 1.0, 0.0, 1.0], // Yellow
        [0.0, 1.0, 1.0, 1.0], // Cyan
        [1.0, 0.0, 1.0, 1.0], // Magenta
        [1.0, 1.0, 1.0, 1.0], // White
        [0.5, 0.5, 0.5, 1.0], // Gray
    ];

    pub fn new(device: &wgpu::Device, buffers: &[Vec<f32>]) -> Self {
        let shader_module = shader_module(device, "graph:shader_module", include_str!("shaders/graph.wgsl"));
        let buffers = init_buffers(device, buffers);
        let (config_groups, config_group_layout) = init_config_groups(device, buffers.len());

        let pipeline = PipelineBuilder::new(device)
            .label("graph:pipeline")
            .layout("graph:pipeline_layout", &[&config_group_layout])
            .vertex(&shader_module, "vs_main", &[VERTEX2D_VERTEX_LAYOUT])
            .fragment(&shader_module, "fs_main", &[Some(STANDARD_COLOR_TARGET_STATE)])
            .primitive(wgpu::PrimitiveTopology::LineStrip)
            .multisample(STANDARD_MULTISAMPLE_STATE)
            .build();

        Self {
            pipeline,
            buffers,
            config_groups,
        }
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        bounds: iced::Rectangle<u32>,
    ) {
        let mut render_pass = RenderPassBuilder::new()
            .label("graph:render_pass")
            .color_attachment(target, LoadOp::Load, StoreOp::Store)
            .build(encoder);

        render_pass.set_pipeline(&self.pipeline);
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

        // println!("Buffers count {}", self.buffers.len());
        for i in 0..self.buffers.len() {
            let buffer = &self.buffers[i];
            render_pass.set_bind_group(0, &self.config_groups[i], &[]);
            render_pass.set_vertex_buffer(0, buffer.slice(..));
            let vertex_count = (buffer.size() / std::mem::size_of::<[f32; 2]>() as u64) as u32;
            render_pass.draw(0..vertex_count, 0..1);
        }
    }

    pub fn update_buffers(&mut self, device: &wgpu::Device, buffers: &[Vec<f32>]) {
        self.buffers = init_buffers(device, buffers);
        self.config_groups = init_config_groups(device, buffers.len()).0;
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

fn init_config_groups(device: &wgpu::Device, count: usize) -> (Vec<wgpu::BindGroup>, wgpu::BindGroupLayout) {
    let mut groups = vec![];

    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("graph:config_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    for i in 0..count {
        let color_buffer = buffer_init(
            device,
            &format!("graph:config_group:color:{}", i),
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            &State::COLORS[i % State::COLORS.len()]
        );
        
        let (config_group, _) = BindGroupBuilder::new(device, &format!("graph:config_group:{}", i))
            .add_entry(0, ShaderStages::FRAGMENT, None, &color_buffer)
            .build();

        groups.push(config_group);
    }
    
    (groups, layout)
}