mod pipeline_builder;
mod render_pass_builder;
mod bind_group_builder;


pub use render_pass_builder::*;
pub use pipeline_builder::*;
pub use bind_group_builder::*;



use iced::widget::shader::wgpu::{
    self, util::{BufferInitDescriptor, DeviceExt}, ColorTargetState, Device, ShaderModule
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

pub fn color_to_f32(color: [u8; 4]) -> [f32; 4] {
    [
        srgb_to_linear(color[0] as f32 / 255.0),
        srgb_to_linear(color[1] as f32 / 255.0),
        srgb_to_linear(color[2] as f32 / 255.0),
        color[3] as f32 / 255.0,
    ]
}

fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}