struct PointLight {
    position_and_intensity: vec4<f32>,
    color_and_radius: vec4<f32>
};

struct CameraUniforms {
    inv_view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
    light_count: u32,
    instance_count: u32
};

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

@group(0) @binding(0) var t_color: texture_2d<f32>;
@group(0) @binding(1) var t_normals: texture_2d<f32>;
@group(0) @binding(2) var t_output: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(3) var t_depth: texture_depth_2d;

struct LightsArray { lights: array<PointLight, 16> };
@group(1) @binding(0) var<uniform> camera: CameraUniforms;
@group(1) @binding(1) var<uniform> light_buffer: LightsArray;
@group(1) @binding(2) var t_lut: texture_2d<f32>;
@group(1) @binding(3) var s_lut: sampler;

@group(2) @binding(0) var<storage, read> mesh_buffer: array<f32>;
@group(2) @binding(1) var<storage, read> bvh_nodes: array<BVHNode>;
@group(2) @binding(2) var<storage, read> instances: array<ModelInstance>;

fn signNotZero(v: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(select(-1.0, 1.0, v.x >= 0.0), select(-1.0, 1.0, v.y >= 0.0));
}

fn octDecode(e: vec2<f32>) -> vec3<f32> {
    var n = vec3<f32>(e.x, e.y, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0) {
        let t = (1.0 - abs(n.yx)) * signNotZero(n.xy);
        n.x = t.x; n.y = t.y;
    }
    return normalize(n);
}

fn get_vertex_position(vertex_idx: u32) -> vec3<f32> {
    let offset = vertex_idx * 8u; 
    return vec3<f32>(mesh_buffer[offset + 0u], mesh_buffer[offset + 1u], mesh_buffer[offset + 2u]);
}

fn ray_aabb_intersect(ro: vec3<f32>, inv_rd: vec3<f32>, aabb_min: vec3<f32>, aabb_max: vec3<f32>, max_t: f32) -> f32 {
    let t0 = (aabb_min - ro) * inv_rd;
    let t1 = (aabb_max - ro) * inv_rd;
    let tmin = min(t0, t1);
    let tmax = max(t0, t1);
    
    let tnear = max(max(tmin.x, tmin.y), tmin.z);
    let tfar = min(min(tmax.x, tmax.y), tmax.z);
    
    if (tnear < tfar && tnear < max_t && tfar > 0.0) {
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
    return t > 0.01 && t < max_t;
}

@compute @workgroup_size(16, 16, 1)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dimensions = textureDimensions(t_color);
    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) { return; }

    let coords = vec2<i32>(global_id.xy);
    let depth = textureLoad(t_depth, coords, 0); 
    let albedo = textureLoad(t_color, coords, 0).rgb;

    if (depth == 1.0) {
        textureStore(t_output, coords, vec4<f32>(albedo, 1.0));
        return;
    }

    let encoded_normal = textureLoad(t_normals, coords, 0).xy;
    let normal = octDecode(encoded_normal);

    let uv = vec2<f32>(global_id.xy) / vec2<f32>(dimensions);
    let ndc_pos = vec4<f32>(uv.x * 2.0 - 1.0, (1.0 - uv.y) * 2.0 - 1.0, depth, 1.0);
    var world_pos_hom = camera.inv_view_proj * ndc_pos;
    let world_pos = world_pos_hom.xyz / world_pos_hom.w; 

    var accumulated_light = vec3<f32>(0.0); 
    let instance_count = camera.instance_count;

    for (var i: u32 = 0u; i < camera.light_count; i++) {
        let light = light_buffer.lights[i];
        let light_pos = light.position_and_intensity.xyz;
        
        let light_dir = light_pos - world_pos;
        let distance = length(light_dir);
        let radius = light.color_and_radius.w;
        
        if (distance > radius) { continue; }

        let l = normalize(light_dir);
        let n_dot_l = dot(normal, l);
        if (n_dot_l <= 0.0) { continue; }

        let ray_origin_world = world_pos + (normal * 0.03) + (l * 0.05);
        var is_shadowed = false;

        for (var inst_idx = 0u; inst_idx < instance_count; inst_idx++) {
            let instance = instances[inst_idx];

            let local_ro = (instance.inv_model * vec4<f32>(ray_origin_world, 1.0)).xyz;
            let local_lp = (instance.inv_model * vec4<f32>(light_pos, 1.0)).xyz;
            
            let local_ray_dir = local_lp - local_ro;
            let local_max_t = length(local_ray_dir);
            let local_rd = local_ray_dir / local_max_t;
            let inv_rd = 1.0 / local_rd;

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
                    stack[stack_ptr] = node.left_first;
                    stack_ptr++;
                    stack[stack_ptr] = node.left_first + 1u;
                    stack_ptr++;
                }
            }

            if (is_shadowed) { break; } 
        }

        if (is_shadowed) { continue; }

        let intensity = light.position_and_intensity.w;
        let color = light.color_and_radius.xyz;
        
        let min_dist_sq = 0.04; 
        var attenuation = intensity / (distance * sqrt(distance) + min_dist_sq);

        let distance_ratio = distance / radius;
        let distance_ratio_sq = distance_ratio * distance_ratio;
        let distance_ratio_quad = distance_ratio_sq * distance_ratio_sq;
        let falloff = clamp(1.0 - distance_ratio_quad, 0.0, 1.0);
        let smooth_falloff = falloff * falloff;

        attenuation *= smooth_falloff;

        accumulated_light += color * n_dot_l * attenuation;
    }

    //let final_color = apply_lut(apply_dithering(albedo * accumulated_light, vec2<f32>(f32(coords.x), f32(coords.y))));
    //let final_color = apply_lut(albedo * accumulated_light);

    let hdr_color = albedo * accumulated_light;

    let sdr_color = hdr_color / (hdr_color + vec3<f32>(1.0));

    let final_color = apply_lut(sdr_color);
    textureStore(t_output, coords, vec4<f32>(final_color, 1.0));
}

fn apply_lut(color: vec3<f32>) -> vec3<f32> {
    let block_x = floor(color.b * 31.0 + 0.5);

    let u = (block_x * 32.0 + color.r * 31.0 + 0.5) / 1024.0;
    let v = (color.g * 31.0 + 0.5) / 32.0;

    return textureSampleLevel(t_lut, s_lut, vec2<f32>(u, v), 0.0).rgb;
}

fn apply_dithering(color: vec3<f32>, coords: vec2<f32>) -> vec3<f32> {
    let x = u32(coords.x) % 4u;
    let y = u32(coords.y) % 4u;

    let bayer = array<f32, 16>(
        0.0/16.0,  8.0/16.0,  2.0/16.0, 10.0/16.0,
       12.0/16.0,  4.0/16.0, 14.0/16.0,  6.0/16.0,
        3.0/16.0, 11.0/16.0,  1.0/16.0,  9.0/16.0,
       15.0/16.0,  7.0/16.0, 13.0/16.0,  5.0/16.0
    );

    let dither_value = bayer[y * 4u + x] - 0.5;

    let color_steps = 16.0;

    let dithered_color = color + vec3<f32>(dither_value / color_steps);

    return clamp(dithered_color, vec3<f32>(0.0), vec3<f32>(1.0));
}