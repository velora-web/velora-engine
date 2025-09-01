// UI rendering shader for Velora Engine

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct UIUniforms {
    transform: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: UIUniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Apply transformation matrix
    output.position = uniforms.transform * vec4<f32>(input.position, 1.0);
    
    // Pass through color and texture coordinates
    output.color = input.color;
    output.tex_coords = input.tex_coords;
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simple color output for now
    // In the future, this could support textures, gradients, etc.
    return input.color;
}
