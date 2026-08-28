struct Uniforms {
    mvp: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> ubo: Uniforms;

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec3<f32>
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    var pos = ubo.mvp * vec4<f32>(model.position, 1.0);

    out.clip_position = pos;
    out.uv = model.uv;
    out.normal = model.normal;
    out.world_pos = model.position;

    return out;
}

struct FragOutput {
    @location(0) color: vec4<f32>
};

@fragment
fn fs_main(in: VertexOutput) -> FragOutput {
    var out: FragOutput;
    
    let tex_color = textureSample(t_diffuse, s_diffuse, in.uv);

    let N = normalize(in.normal);

    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.4));

    let diffuse_strength = max(dot(N, light_dir), 0.0);

    let ambient = 0.2;
    let light_factor = diffuse_strength + ambient;

    out.color = vec4<f32>(tex_color.rgb * light_factor, tex_color.a);

    return out;
}