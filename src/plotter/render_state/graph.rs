use super::helpers::*;
use bytemuck::NoUninit;
use iced::widget::shader::wgpu::{self, BufferUsages, LoadOp, Queue, ShaderStages, StoreOp};

pub struct State {
    pipeline: wgpu::RenderPipeline,
    buffers: Vec<wgpu::Buffer>,
    color_buffers: Vec<wgpu::Buffer>,
    config_groups: Vec<wgpu::BindGroup>,
}

impl State {
    const COLORS: &[[f32; 4]] = &[
        [0.0, 0.0, 1.0, 1.0], // Blue
        [0.0, 1.0, 0.0, 1.0], // Green
        [1.0, 0.0, 0.0, 1.0], // Red
        [1.0, 1.0, 0.0, 1.0], // Yellow
        [0.0, 1.0, 1.0, 1.0], // Cyan
        [1.0, 0.0, 1.0, 1.0], // Magenta
    ];

    pub fn new(device: &wgpu::Device, buffers: &[Vec<f32>]) -> Self {
        let shader_module = shader_module(
            device,
            "graph:shader_module",
            include_str!("shaders/graph.wgsl"),
        );
        let buffers = init_buffers(device, buffers);
        let (config_groups, config_group_layout, color_buffers) =
            init_config_groups(device, buffers.len());

        let pipeline = PipelineBuilder::new(device)
            .label("graph:pipeline")
            .layout("graph:pipeline_layout", &[&config_group_layout])
            .vertex(&shader_module, "vs_main", &[VERTEX2D_VERTEX_LAYOUT])
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
            buffers,
            color_buffers,
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
        render_pass.set_scissor_rect(bounds.x, bounds.y, bounds.width, bounds.height);

        // Viewport set is mandatory for the shader to work
        // Without it th shader will draw thing on the whole screen
        // Note that this doesn't fix the LoadOp::Clear issue
        render_pass.set_viewport(
            bounds.x as f32,
            bounds.y as f32,
            bounds.width as f32,
            bounds.height as f32,
            0.0,
            1.0,
        );

        for i in 0..self.buffers.len() {
            let buffer = &self.buffers[i];
            render_pass.set_bind_group(0, &self.config_groups[i], &[]);
            render_pass.set_vertex_buffer(0, buffer.slice(..));
            let vertex_count = (buffer.size() / std::mem::size_of::<[f32; 2]>() as u64) as u32;
            render_pass.draw(0..vertex_count, 0..1);
        }
    }

    pub fn update_buffers(&mut self, device: &wgpu::Device, queue: &Queue, buffers: &[Vec<f32>]) {
        for i in 0..buffers.len() {
            let data: &[u8] = bytemuck::cast_slice(&buffers[i]);
            if i < self.buffers.len() {
                // Check if the new data size exceeds the existing buffer size
                if data.len() > self.buffers[i].size() as usize {
                    // Recreate the buffer with the new size
                    let buffer = init_vertex_buffer(device, i, data);
                    self.buffers[i] = buffer;
                } else {
                    // Update the existing buffer
                    queue.write_buffer(&self.buffers[i], 0, data);
                }
            } else {
                // Create new buffers if needed
                let buffer = init_vertex_buffer(device, i, data);
                self.buffers.push(buffer);

                let color_buffer = init_color_buffer(device, i);
                let (config_group, _) = BindGroupBuilder::new(device, &format!("graph:config_group:{}", i))
                    .add_entry(0, ShaderStages::FRAGMENT, None, &color_buffer)
                    .build();
                self.color_buffers.push(color_buffer);
                self.config_groups.push(config_group);
            }
        }

        // Clean up the buffers if we have fewer inputs
        if buffers.len() < self.buffers.len() {
            self.buffers.truncate(buffers.len());
            self.color_buffers.truncate(buffers.len());
            self.config_groups.truncate(buffers.len());
        }
    }
}

fn init_buffers(device: &wgpu::Device, buffers: &[Vec<f32>]) -> Vec<wgpu::Buffer> {
    (0..buffers.len())
        .map(|i| init_vertex_buffer(device, i, &buffers[i]))
        .collect()
}

fn init_config_groups(
    device: &wgpu::Device,
    count: usize,
) -> (
    Vec<wgpu::BindGroup>,
    wgpu::BindGroupLayout,
    Vec<wgpu::Buffer>,
) {
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
    
    let mut groups = vec![];
    let mut color_buffers = vec![];

    for i in 0..count {
        let color_buffer = init_color_buffer(device, i);
        
        let (config_group, _) = BindGroupBuilder::new(device, &format!("graph:config_group:{}", i))
            .add_entry(0, ShaderStages::FRAGMENT, None, &color_buffer)
            .build();

        color_buffers.push(color_buffer);
        groups.push(config_group);
    }

    (groups, layout, color_buffers)
}

fn init_vertex_buffer<T: NoUninit>(device: &wgpu::Device, i: usize, data: &[T]) -> wgpu::Buffer {
    buffer_init(
        device,
        &format!("graph:buffer:{}", i),
        BufferUsages::VERTEX | BufferUsages::COPY_DST,
        data,
    )
}

fn init_color_buffer(device: &wgpu::Device, i: usize) -> wgpu::Buffer {
    buffer_init(
        device,
        &format!("graph:color_buffer:{}", i),
        BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        &State::COLORS[i % State::COLORS.len()],
    )
}