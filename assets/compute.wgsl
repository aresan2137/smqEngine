struct Light {
    pos: vec3<f32>,
    power: f32,
    color: vec3<f32>,
    max_reach: f32,
}

struct LightUniforms {
    light_count: u32,
    _pad0: f32,
    _pad1: f32,
    _pad2: f32,
    lights: array<Light, 32>,
}

struct BVHNode {
    min_xyz: vec3<f32>,
    left_first: u32, 
    max_xyz: vec3<f32>,
    tri_count: u32
}

@group(0) @binding(0) var t_albedo: texture_2d<f32>;
@group(0) @binding(1) var t_position: texture_2d<f32>;
@group(0) @binding(2) var t_normal: texture_2d<f32>;
@group(0) @binding(3) var t_output: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(4) var<uniform> u_lights: LightUniforms;
@group(0) @binding(5) var<storage, read> vertex_buffer: array<f32>;
@group(0) @binding(6) var<storage, read> bvh_nodes: array<BVHNode>;
@group(0) @binding(7) var t_lut: texture_2d<f32>;
@group(0) @binding(8) var s_lut: sampler;

fn ray_aabb_intersect(ro: vec3<f32>, rd: vec3<f32>, bmin: vec3<f32>, bmax: vec3<f32>, t_max: f32) -> bool {
    let inv_d = 1.0 / rd;
    let t0 = (bmin - ro) * inv_d;
    let t1 = (bmax - ro) * inv_d;
    
    let tmin = min(t0, t1);
    let tmax = max(t0, t1);
    
    let t_near = max(max(tmin.x, tmin.y), tmin.z);
    let t_far = min(min(tmax.x, tmax.y), tmax.z);
    
    return t_near <= t_far && t_far > 0.0 && t_near < t_max;
}

fn ray_triangle_intersect(ro: vec3<f32>, rd: vec3<f32>, v0: vec3<f32>, v1: vec3<f32>, v2: vec3<f32>, t: ptr<function, f32>) -> bool {
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
    
    *t = f * dot(edge2, q);
    return *t > 0.00001;
}

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(t_albedo);
    if (id.x >= size.x || id.y >= size.y) { return; }

    let coord = vec2<u32>(id.xy);

    let raw_normal = textureLoad(t_normal, coord, 0).xyz;

    if (dot(raw_normal, raw_normal) < 0.001) {
        textureStore(t_output, coord, vec4<f32>(0.0, 0.0, 0.0, 1.0));
        return;
    }

    let normal = normalize(raw_normal);
    let albedo_color = textureLoad(t_albedo, coord, 0);
    let world_pos = textureLoad(t_position, coord, 0).xyz;

    var final_color = albedo_color.rgb;
    var lighting = vec3<f32>(0.0);
    let num_lights = u_lights.light_count;
    let stride_floats = 8u;

    for (var i: u32 = 0u; i < num_lights; i = i + 1u) {
        let light = u_lights.lights[i];
        
        let delta_pos = light.pos - world_pos;
        let dist = length(delta_pos);
        let dir = delta_pos / dist;
        
        let falloff = 1.0 / (dist * 0.5 + 0.01);
        let n_dot_l = max(dot(normal, dir), 0.1);
        
        var shadow = 1.0;
        let shadow_ro = world_pos + normal * 0.00001; 

        if (n_dot_l > 0.1) {
            var stack: array<u32, 32>;
            var stack_ptr = 0u;
            
            stack[stack_ptr] = 0u;
            stack_ptr = stack_ptr + 1u;

            while (stack_ptr > 0u) {
                stack_ptr = stack_ptr - 1u;
                let node_idx = stack[stack_ptr];
                let node = bvh_nodes[node_idx];

                if (ray_aabb_intersect(shadow_ro, dir, node.min_xyz, node.max_xyz, dist)) {
                    if (node.tri_count > 0u) {
                        for (var t = 0u; t < node.tri_count; t = t + 1u) {
                            let t_idx = node.left_first + t;
                            let base_v = t_idx * 3u;
                            
                            let i0 = (base_v + 0u) * stride_floats;
                            let i1 = (base_v + 1u) * stride_floats;
                            let i2 = (base_v + 2u) * stride_floats;

                            let v0 = vec3<f32>(vertex_buffer[i0], vertex_buffer[i0 + 1u], vertex_buffer[i0 + 2u]);
                            let v1 = vec3<f32>(vertex_buffer[i1], vertex_buffer[i1 + 1u], vertex_buffer[i1 + 2u]);
                            let v2 = vec3<f32>(vertex_buffer[i2], vertex_buffer[i2 + 1u], vertex_buffer[i2 + 2u]);
                            
                            var t_tri = 0.0;
                            if (ray_triangle_intersect(shadow_ro, dir, v0, v1, v2, &t_tri)) {
                                if (t_tri < dist) {
                                    shadow = 0.1;
                                    break;
                                }
                            }
                        }
                        if (shadow == 0.1) { break; }
                    } else {
                        stack[stack_ptr] = node.left_first + 1u;
                        stack_ptr = stack_ptr + 1u;
                        stack[stack_ptr] = node.left_first;
                        stack_ptr = stack_ptr + 1u;
                    }
                }
            }
        } else {
            shadow = 0.1;
        }
        
        lighting += falloff * light.color * light.power * n_dot_l * shadow;
    }
    
    final_color *= lighting;
    final_color = clamp(final_color, vec3<f32>(0.0), vec3<f32>(1.0));

    final_color = apply_dither(final_color, coord);
    final_color = apply_lut(final_color);

    textureStore(t_output, coord, vec4<f32>(final_color, 1.0));
}

fn apply_dither(color: vec3<f32>, coord: vec2<u32>) -> vec3<f32> {
    let bayer = array<f32, 16>(
        -0.5,       0.0,       -0.375,    0.125,
         0.25,     -0.25,       0.375,   -0.125,
        -0.4375,    0.0625,    -0.46875,  0.03125,
         0.1875,   -0.3125,     0.3125,  -0.1875
    );

    let bayer_ps1 = array<f32, 16>(
        -4.0,  0.0, -3.0,  1.0,
         2.0, -2.0,  3.0, -1.0,
        -3.0,  1.0, -4.0,  0.0,
         3.0, -1.0,  2.0, -2.0
    );
    
    let bayer_index = (coord.y % 4u) * 4u + (coord.x % 4u);
    let offset = bayer[bayer_index];
    let offset_ps1 = bayer_ps1[bayer_index];
    
    let dither_spread = 1.0 / 16.0; 

    return clamp(color + vec3<f32>(mix(offset, offset_ps1, 0.25) * dither_spread), vec3<f32>(0.0), vec3<f32>(1.0));
}

fn apply_lut(color: vec3<f32>) -> vec3<f32> {
    let clamped_color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));
    
    let size = 16.0; 
    let max_color = size - 1.0;
    
    let scaled_rgb = floor(clamped_color * max_color);
    let cell_b = scaled_rgb.z; 
    
    let uv_x = (cell_b * size + scaled_rgb.x + 0.5) / 256.0;
    let uv_y = (max_color - scaled_rgb.y + 0.5) / 16.0;
    
    return textureSampleLevel(t_lut, s_lut, vec2<f32>(uv_x, uv_y), 0.0).rgb;
}