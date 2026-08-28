struct Uniforms {
    mvp: mat4x4<f32>
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
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    out.position = ubo.mvp * vec4<f32>(model.position, 1.0);
    out.uv = model.uv;
    
    out.normal = model.normal;

    return out;
}

struct FragOutput {
    @location(0) color: vec4<f32>,
    @location(1) normal: vec2<f32>
};

fn signNotZero(v: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(
        select(-1.0, 1.0, v.x >= 0.0),
        select(-1.0, 1.0, v.y >= 0.0)
    );
}

fn octEncode(n: vec3<f32>) -> vec2<f32> {
    var p = n.xy * (1.0 / (abs(n.x) + abs(n.y) + abs(n.z)));
    if (n.z < 0.0) {
        p = (1.0 - abs(p.yx)) * signNotZero(p);
    }
    return p;
}

@fragment
fn fs_main(in: VertexOutput) -> FragOutput {
    var out: FragOutput;

    out.color = vec4<f32>(textureSample(t_diffuse, s_diffuse, in.uv).rgb, 1.0);
    
    let normal_3d = normalize(in.normal);
    out.normal = octEncode(normal_3d);

    return out;
}