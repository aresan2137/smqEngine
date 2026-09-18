
struct UcLight {
    position: vec3f,
    color: vec3f,
    power: f32
}

struct UboDefferedInfo {
    camera_position: vec3f,
    light_count: u32,
    lights: array<UcLight, 16>,
    instance_count: u32
}

struct BVHNode {
    aabb_min: vec3<f32>,
    left_first: u32,
    aabb_max: vec3<f32>,
    tri_count: u32
};

struct ModelInstance {
    inv_model: mat4x4<f32>,
    bvh_root_node: u32
};

@group(0) @binding(0) var<uniform> info: UboDefferedInfo;
@group(0) @binding(1) var t_0: texture_2d<f32>;
@group(0) @binding(2) var t_1: texture_2d<f32>;
@group(0) @binding(3) var t_2: texture_2d<f32>;
@group(0) @binding(4) var<storage, read> mesh_buffer: array<f32>;
@group(0) @binding(5) var<storage, read> bvh_nodes: array<BVHNode>;
@group(0) @binding(6) var<storage, read> instances: array<ModelInstance>;

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

fn get_vertex_position(vertex_idx: u32) -> vec3<f32> {
    let base = vertex_idx * 8u;
    return vec3<f32>(mesh_buffer[base], mesh_buffer[base + 1u], mesh_buffer[base + 2u]);
}

fn ray_aabb_intersect(ro: vec3<f32>, inv_rd: vec3<f32>, aabb_min: vec3<f32>, aabb_max: vec3<f32>, max_t: f32) -> f32 {
    let t0 = (aabb_min - ro) * inv_rd;
    let t1 = (aabb_max - ro) * inv_rd;
    
    let tmin_vec = min(t0, t1);
    let tmax_vec = max(t0, t1);
    
    let tnear = max(max(tmin_vec.x, tmin_vec.y), tmin_vec.z);
    let tfar = min(min(tmax_vec.x, tmax_vec.y), tmax_vec.z);
    
    if (tfar >= max(tnear, 0.0) && tnear < max_t) {
        return max(tnear, 0.0); 
    }
    return 9999999.0;
}

fn ray_triangle_intersect(ro: vec3<f32>, rd: vec3<f32>, v0: vec3<f32>, v1: vec3<f32>, v2: vec3<f32>, max_t: f32) -> bool {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let h = cross(rd, edge2);
    let a = dot(edge1, h);
    
    if (a > -0.00001 && a < 0.00001) { return false; } 
    
    let f = 1.0 / a;
    let s = ro - v0;
    let u = f * dot(s, h);
    if (u < 0.0 || u > 1.0) { return false; }
    
    let q = cross(s, edge1);
    let v = f * dot(rd, q);
    if (v < 0.0 || u + v > 1.0) { return false; }
    
    let t = f * dot(edge2, q);
    return t > 0.001 && t < max_t;
}

fn calculate_blinn_phong(normal: vec3f, V: vec3f, L: vec3f, albedo: vec3f, roughness: f32, metallic: f32, light_color: vec3f) -> vec3f {
    let H = normalize(V + L);
    let NdotL = max(dot(normal, L), 0.0);
    let NdotH = max(dot(normal, H), 0.0);

    let shininess = exp2(10.0 * (1.0 - roughness));

    let diffuse = albedo * (1.0 - metallic);
    
    let specular_color = mix(vec3f(0.04), albedo, metallic);

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
        let light = info.lights[i];
        let light_pos = light.position;

        let light_dir = light_pos - position;
        let distance = length(light_dir);
        let L = light_dir / distance; 

        let n_dot_l = dot(normal, L);
        if (n_dot_l <= 0.0) { continue; }

        let attenuation = 1.0 / pow(distance + 0.04, 1.8);
        let light_color = light.color * light.power * attenuation; 

        let n_dot_v = max(abs(dot(normal, V)), 0.001);
        let ray_origin_world = position + normal * 0.001;

        var is_shadowed = false;

        for (var inst_idx = 0u; inst_idx < info.instance_count; inst_idx++) {
            let instance = instances[inst_idx];

            let local_ro = (instance.inv_model * vec4<f32>(ray_origin_world, 1.0)).xyz;
            let local_lp = (instance.inv_model * vec4<f32>(light_pos, 1.0)).xyz;
            
            let local_ray_dir = local_lp - local_ro;
            let local_max_t = length(local_ray_dir);
            let local_rd = local_ray_dir / local_max_t;
            
            let local_rd_safe = select(local_rd, vec3<f32>(0.0000001), abs(local_rd) < vec3<f32>(0.0000001));
            let inv_rd = 1.0 / local_rd_safe;

            var stack: array<u32, 32>;
            var stack_ptr: u32 = 0u;
            
            stack[stack_ptr] = instance.bvh_root_node;
            stack_ptr++;

            while (stack_ptr > 0u) {
                stack_ptr--;
                let node_idx = stack[stack_ptr];
                let node = bvh_nodes[node_idx];

                let dist = ray_aabb_intersect(local_ro, inv_rd, node.aabb_min, node.aabb_max, local_max_t);
                if (dist > local_max_t) { continue; }

                if (node.tri_count > 0u) {
                    for (var j = 0u; j < node.tri_count; j++) {
                        let tri_idx = node.left_first + j;
                        
                        let v0 = get_vertex_position(tri_idx * 3u + 0u);
                        let v1 = get_vertex_position(tri_idx * 3u + 1u);
                        let v2 = get_vertex_position(tri_idx * 3u + 2u);

                        if (ray_triangle_intersect(local_ro, local_rd, v0, v1, v2, local_max_t)) {
                            is_shadowed = true;
                            break;
                        }
                    }
                    if (is_shadowed) { break; }
                } else {
                    if (stack_ptr + 2u <= 32u) {
                        stack[stack_ptr] = node.left_first;
                        stack_ptr++;
                        stack[stack_ptr] = node.left_first + 1u;
                        stack_ptr++;
                    }
                }
            }

            if (is_shadowed) { break; } 
        }

        if (is_shadowed) { continue; }

        final_color += calculate_blinn_phong(normal, V, L, color, roughness, metallic, light_color);
    }

    final_color = tonemap_aces(final_color);

    final_color = pow(final_color, vec3f(1.0 / 2.2));

    final_color = floor(final_color * 32.0) / 32.0;

    return vec4f(final_color, 1.0);
}