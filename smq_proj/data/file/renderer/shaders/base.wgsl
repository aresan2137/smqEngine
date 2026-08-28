struct UboData0 {
    proj: mat4x4f
}

struct UboData1 {
    view: mat4x4f
}

struct UboData2 {
    model: mat4x4f
}

@group(0) @binding(0) var<uniform> data0: UboData0;
@group(1) @binding(0) var<uniform> data1: UboData1;
@group(2) @binding(0) var<uniform> data2: UboData2;

struct VertexData {
    @location(0) position: vec3f,
    @location(1) uv: vec2f,
    @location(2) normal: vec3f
}

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) normal: vec3f
}

@vertex
fn vs_main(in: VertexData) -> VertexOutput {
    var out: VertexOutput;

    out.pos = data0.proj * data1.view * data2.model * vec4f(in.position, 1.0);
    out.normal = in.normal;

    return out;
}

const light = vec3f(5.0, 5.0, 3.0);

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {

    let dit = dot(normalize(in.normal), normalize(light));
    let lit = max(dit, 0.0);

    return vec4f(lit, lit, lit, 1.0); 
}