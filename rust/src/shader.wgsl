struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vertex(
    @location(0) a_position: vec2<f32>,
) -> VertexOutput {
    return VertexOutput(vec4<f32>(a_position, 0.0, 1.0));
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fragment() -> FragmentOutput {
    return FragmentOutput(vec4<f32>(1.0, 0.0, 0.0, 1.0));
}