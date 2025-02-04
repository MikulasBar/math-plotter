
@group(0) @binding(0) var<uniform> line_color: vec4<f32>;

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 0.0, 1.0);
}

@fragment
fn fs_main(
    @builtin(position) input: vec4<f32>,
) -> @location(0) vec4<f32> {
    return line_color;
}