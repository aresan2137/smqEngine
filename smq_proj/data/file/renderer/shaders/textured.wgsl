struct UboData0 {
    proj: mat4x4f
}

struct UboData1 {
    view: mat4x4f
}

struct DUboData2 {
    model: mat4x4f
}

@group(0) @binding(0) var<uniform> data0: UboData0;

@group(1) @binding(0) var<uniform> data1: UboData1;

@group(2) @binding(0) var<uniform> data2: DUboData2;
@group(2) @binding(1) var t_base: texture_2d<f32>;
@group(2) @binding(2) var s_base: sampler;

struct VertexData {
    @location(0) position: vec3f,
    @location(1) uv: vec2f,
    @location(2) normal: vec3f
}

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) uv: vec2f,
    @location(1) normal: vec3f,
    @location(2) position: vec3f
}

@vertex
fn vs_main(in: VertexData) -> VertexOutput {
    var out: VertexOutput;

    out.pos = data0.proj * data1.view * data2.model * vec4f(in.position, 1.0);
    out.uv = in.uv;
    out.normal = (data2.model * vec4f(in.normal, 1.0)).xyz;
    out.position = (data2.model * vec4f(in.position, 1.0)).xyz;

    return out;
}

struct FragmentOutput {
    @location(0) color: vec4f,
    @location(1) position: vec4f,
    @location(2) normal: vec4f,
}

fn signNotZero(v: vec2f) -> vec2f {
    let x = select(-1.0, 1.0, v.x >= 0.0);
    let y = select(-1.0, 1.0, v.y >= 0.0);
    return vec2f(x, y);
}

fn encodeOctahedral(n_in: vec3f) -> vec2f {
    var n = n_in; 
    
    n /= (abs(n.x) + abs(n.y) + abs(n.z));
    
    if (n.z < 0.0) {
        let folded = (1.0 - abs(vec2f(n.y, n.x))) * signNotZero(vec2f(n.x, n.y));
        
        n.x = folded.x;
        n.y = folded.y;
    }
    
    return vec2f(n.x, n.y);
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;

    out.color = vec4f(textureSample(t_base, s_base, in.uv).xyz, 0.0);

    out.position = vec4f(in.position, 1.0);

    out.normal = vec4f(encodeOctahedral(normalize(in.normal)), 0.0, 0.0);

    return out; 
}