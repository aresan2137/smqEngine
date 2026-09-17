
struct UcLight {
    position: vec3f,
    color: vec3f,
    power: f32
}

struct UboDefferedInfo {
    camera_position: vec3f,
    light_count: u32,
    lights: array<UcLight, 16>
}

@group(0) @binding(0) var<uniform> info: UboDefferedInfo;
@group(0) @binding(1) var t_0: texture_2d<f32>;
@group(0) @binding(2) var t_1: texture_2d<f32>;
@group(0) @binding(3) var t_2: texture_2d<f32>;

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) uv: vec2f
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32((vertex_index << 1u) & 2u);
    let y = f32(vertex_index & 2u);
    out.uv = vec2<f32>(x, y);
    out.position = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    return out;
}

fn signNotZero(v: vec2f) -> vec2f {
    let x = select(-1.0, 1.0, v.x >= 0.0);
    let y = select(-1.0, 1.0, v.y >= 0.0);
    return vec2f(x, y);
}

fn decodeOctahedral(e: vec2f) -> vec3f {
    var n = vec3f(e.x, e.y, 1.0 - abs(e.x) - abs(e.y));
    
    if (n.z < 0.0) {
        let folded = (1.0 - abs(vec2f(n.y, n.x))) * signNotZero(vec2f(n.x, n.y));
        n.x = folded.x;
        n.y = folded.y;
    }
    
    return normalize(n);
}

fn calculate_blinn_phong(normal: vec3f, V: vec3f, L: vec3f, albedo: vec3f, roughness: f32, metallic: f32, light_color: vec3f) -> vec3f {
    let H = normalize(V + L);
    let NdotL = max(dot(normal, L), 0.0);
    let NdotH = max(dot(normal, H), 0.0);

    let shininess = exp2(10.0 * (1.0 - roughness));

    let diffuse_color = albedo * (1.0 - metallic);
    
    let specular_color = mix(vec3f(0.04), albedo, metallic);

    let diffuse = diffuse_color;
    var specular_intensity = pow(NdotH, shininess);
    
    specular_intensity = specular_intensity * ((shininess + 8.0) / (8.0 * 3.14159265359));
    
    let specular = specular_color * specular_intensity;

    return (diffuse + specular) * light_color * NdotL;
}

fn tonemap_aces(x: vec3f) -> vec3f {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), vec3f(0.0), vec3f(1.0));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let px = vec2<i32>(in.uv * vec2<f32>(384.0, 216.0));

    let color_raw = textureLoad(t_0, px, 0);
    let position_raw = textureLoad(t_1, px, 0);
    let normal_raw = textureLoad(t_2, px, 0);

    let normal = decodeOctahedral(normal_raw.xy);
    let normal_map = normal_raw.zw;
    let position = position_raw.xyz;
    let color = color_raw.rgb;
    let roughness = clamp(position_raw.w, 0.05, 1.0);
    let metallic = color_raw.w;

    let V = normalize(info.camera_position - position);

    var final_color = vec3f(0.0);

    for (var i: u32 = 0; i < info.light_count; i++) {
        let light_pos = info.lights[i].position;

        let L = normalize(light_pos - position); 
    
        let distance = length(light_pos - position);
        let attenuation = 1.0 / (distance * sqrt(distance));
        let light_color = info.lights[i].color * info.lights[i].power * attenuation; 

        final_color += calculate_blinn_phong(normal, V, L, color, roughness, metallic, light_color);
    }

    final_color = tonemap_aces(final_color);

    final_color = pow(final_color, vec3f(1.0 / 2.2));

    return vec4f(final_color, 1.0);
}