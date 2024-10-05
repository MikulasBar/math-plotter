mod pipeline_builder;
mod render_pass_builder;



pub use render_pass_builder::*;
pub use pipeline_builder::*;


use iced::widget::shader::wgpu::{
    self, util::{BufferInitDescriptor, DeviceExt}, BufferUsages, ColorTargetState, Device, ShaderModule
};

/// Only 2D vertices
pub const VERTEX2D_VERTEX_LAYOUT: wgpu::VertexBufferLayout<'_> = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
};

pub const STANDARD_COLOR_TARGET_STATE: ColorTargetState = ColorTargetState {
    format: wgpu::TextureFormat::Bgra8UnormSrgb,
    blend: Some(wgpu::BlendState::REPLACE),
    write_mask: wgpu::ColorWrites::ALL,
};

pub const STANDARD_MULTISAMPLE_STATE: wgpu::MultisampleState = wgpu::MultisampleState {
    count: 1,
    mask: !0,
    alpha_to_coverage_enabled: false,
};

pub fn shader_module(device: &Device, label: &str, src: &str) -> ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(src.into()),
    })
}

pub fn single_entry_bind_group<T>(
    device: &Device, 
    label: &str, 
    binding: u32,
    contents: &[T]
) -> (wgpu::BindGroup, wgpu::BindGroupLayout)
    where T: bytemuck::NoUninit
{
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(format!("{}-layout", label).as_str()),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let buffer = buffer_init(
        device, 
        format!("{}-buffer", label).as_str(), 
        BufferUsages::UNIFORM | BufferUsages::COPY_DST, 
        contents
    );

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding,
            resource: buffer.as_entire_binding(),
        }],
    });

    (bind_group, layout)
}

pub fn buffer_init<T>(
    device: &Device, 
    label: &str, 
    usage: wgpu::BufferUsages, 
    contents: &[T]
) -> wgpu::Buffer
    where T: bytemuck::NoUninit,
{
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::cast_slice(contents),
        usage,
    })
}


