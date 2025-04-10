// This assumes you're using a custom 2D material with a uniform for `time`

// Declare the global uniform for `time`
@group(0) @binding(0)
var<uniform> time: f32;

// Custom material color uniform
@group(2) @binding(0)
var<uniform> material: CustomMaterial;

struct CustomMaterial {
    color: vec4<f32>,
};

// Vertex input structure: position, blend color
struct VertexInput {
    @location(0) position: vec3<f32>,  // Use 2D position (x, y, z for z=0)
    @location(1) blend_color: vec4<f32>,  // Vertex color (used for blending)
};

// Vertex output structure
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,  // 2D clip position (transformed)
    @location(0) blend_color: vec4<f32>,  // Color for blending
};

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // Apply the ViewProj matrix to the position (transformed 2D position)
    out.clip_position = ViewProj * vec4<f32>(vertex.position.xy, 0.0, 1.0);
    out.blend_color = vertex.blend_color;
    return out;
}

// Fragment shader: Color blending with `time` and vertex color
@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let speed = 2.0;
    let t_1 = sin(time * speed) * 0.5 + 0.5;
    let t_2 = cos(time * speed);

    let distance_to_center = distance(input.clip_position.xy, vec2<f32>(0.5, 0.5)) * 1.4;

    let red = vec3<f32>(0.627955, 0.224863, 0.125846);
    let green = vec3<f32>(0.86644, -0.233887, 0.179498);
    let blue = vec3<f32>(0.701674, 0.274566, -0.169156);
    let white = vec3<f32>(1.0, 0.0, 0.0);

    let mixed = mix(mix(red, blue, t_1), mix(green, white, t_2), distance_to_center);

    return vec4<f32>(oklab_to_linear_srgb(mixed), 1.0);
}
