// Imports for sprite shaders
#import bevy_sprite::mesh_functions::{get_view_proj, get_model}

// Uniform for custom color
struct CustomMaterial {
    color: vec4<f32>,
};
@group(2) @binding(0) var<uniform> material: CustomMaterial;

// Vertex input struct
struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec2<f32>,
    @location(1) blend_color: vec4<f32>,
};

// Vertex output to fragment shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // Convert 2D position into vec4 with z=0.0 and w=1.0
    let local_pos = vec4<f32>(vertex.position, 0.0, 1.0);

    // Apply model and view-projection transform
    let model = get_model(vertex.instance_index);
    let view_proj = get_view_proj();
    out.clip_position = view_proj * model * local_pos;

    out.blend_color = vertex.blend_color;
    return out;
}

struct FragmentInput {
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    return material.color * input.blend_color;
}
