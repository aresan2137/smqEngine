use glam::*;

#[repr(C)]
#[derive(Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BVHNode {
    pub aabb_min: Vec3,
    pub left_first: u32,
    pub aabb_max: Vec3,
    pub tri_count: u32,
}

#[derive(Clone, Copy)]
struct Tri {
    centroid: Vec3,
    data: [[f32; 8]; 3], 
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelInstance {
    pub inv_model: Mat4,
    pub bvh_root_node: u32,
    pub id: u32,
    pub _padding: [u32; 2]
}

fn update_node_bounds(node_idx: usize, tris: &[Tri], nodes: &mut [BVHNode]) {
    let node = &mut nodes[node_idx];
    node.aabb_min = vec3(f32::MAX, f32::MAX, f32::MAX);
    node.aabb_max = vec3(f32::MIN, f32::MIN, f32::MIN);

    let first = node.left_first as usize;
    let count = node.tri_count as usize;

    for i in 0..count {
        let tri = &tris[first + i];
        for v in 0..3 {
            let pos = vec3(tri.data[v][0], tri.data[v][1], tri.data[v][2]); 
            node.aabb_min = node.aabb_min.min(pos);
            node.aabb_max = node.aabb_max.max(pos);
        }
    }

    let epsilon = 0.001; 
    if node.aabb_max.x - node.aabb_min.x < epsilon {
        node.aabb_min.x -= epsilon; 
        node.aabb_max.x += epsilon;
    }
    if node.aabb_max.y - node.aabb_min.y < epsilon {
        node.aabb_min.y -= epsilon; 
        node.aabb_max.y += epsilon;
    }
    if node.aabb_max.z - node.aabb_min.z < epsilon {
        node.aabb_min.z -= epsilon; 
        node.aabb_max.z += epsilon;
    }
}

fn subdivide(node_idx: usize, tris: &mut [Tri], nodes: &mut Vec<BVHNode>, nodes_used: &mut usize) {
    let node = nodes[node_idx];
    if node.tri_count <= 2 { return; }

    let extent = node.aabb_max - node.aabb_min;
    
    let mut axis = 0;
    if extent.y > extent.x { axis = 1; }
    if extent.z > if axis == 0 { extent.x } else { extent.y } { axis = 2; }

    let first = node.left_first as usize;
    let count = node.tri_count as usize;

    tris[first..first + count].sort_by(|a, b| {
        let a_val = if axis == 0 { a.centroid.x } else if axis == 1 { a.centroid.y } else { a.centroid.z };
        let b_val = if axis == 0 { b.centroid.x } else if axis == 1 { b.centroid.y } else { b.centroid.z };
        a_val.partial_cmp(&b_val).unwrap()
    });

    let mid = first + count / 2;
    let left_count = mid - first;
    let right_count = count - left_count;

    let left_child_idx = *nodes_used;
    *nodes_used += 2; 

    nodes[node_idx].left_first = left_child_idx as u32;
    nodes[node_idx].tri_count = 0; 

    nodes[left_child_idx].left_first = first as u32;
    nodes[left_child_idx].tri_count = left_count as u32;
    
    nodes[left_child_idx + 1].left_first = mid as u32;
    nodes[left_child_idx + 1].tri_count = right_count as u32;

    update_node_bounds(left_child_idx, tris, nodes);
    update_node_bounds(left_child_idx + 1, tris, nodes);

    subdivide(left_child_idx, tris, nodes, nodes_used);
    subdivide(left_child_idx + 1, tris, nodes, nodes_used);
}

pub struct MegaGeometryData {
    pub mega_vertex_bytes: Vec<u8>,
    pub mega_nodes: Vec<BVHNode>,
    pub vertex_count: u32,
    pub bvh_roots: Vec<u32>
}

pub fn build_mega_geometry(smf_files: &[&[u8]], stride: u64) -> MegaGeometryData {
    let mut mega_vertices= Vec::new();
    let mut mega_nodes = Vec::new();
    let mut bvh_roots = Vec::new();
    let mut total_vertex_count = 0;

    for smf_data in smf_files {
        let mut offset = 1;
        let vertex_count = u32::from_le_bytes(smf_data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 8;
        let verts_data_size = vertex_count * stride as usize;
        let vertex_data = &smf_data[offset..offset + verts_data_size];

        let mut floats = vec![0.0f32; vertex_data.len() / 4];
        for (i, chunk) in vertex_data.chunks_exact(4).enumerate() {
            floats[i] = f32::from_le_bytes(chunk.try_into().unwrap());
        }

        let num_tris = vertex_count / 3;
        let mut tris = Vec::with_capacity(num_tris);
        for i in 0..num_tris {
            let base = i * 3 * 8;
            let mut data = [[0.0; 8]; 3];
            for v in 0..3 {
                for f in 0..8 { data[v][f] = floats[base + v * 8 + f]; }
            }
            let c = (vec3(data[0][0], data[0][1], data[0][2]) + vec3(data[1][0], data[1][1], data[1][2]) + vec3(data[2][0], data[2][1], data[2][2])) / 3.0;
            tris.push(Tri { centroid: c, data });
        }

        let mut local_nodes = vec![BVHNode::default(); num_tris * 2];
        local_nodes[0].left_first = 0;
        local_nodes[0].tri_count = num_tris as u32;
        let mut nodes_used = 1;
        update_node_bounds(0, &tris, &mut local_nodes);
        subdivide(0, &mut tris, &mut local_nodes, &mut nodes_used);
        local_nodes.truncate(nodes_used);

        let current_triangle_offset = (mega_vertices.len() / 24) as u32;
        let current_node_offset = mega_nodes.len() as u32;

        bvh_roots.push(current_node_offset);

        for mut node in local_nodes {
            if node.tri_count > 0 {
                node.left_first += current_triangle_offset;
            } else {
                node.left_first += current_node_offset;
            }
            mega_nodes.push(node);
        }

        for tri in &tris {
            for v in 0..3 {
                for f in 0..8 { mega_vertices.push(tri.data[v][f]); }
            }
        }
        total_vertex_count += vertex_count;
    }

    MegaGeometryData {
        mega_vertex_bytes: bytemuck::cast_slice(&mega_vertices).to_vec(),
        mega_nodes,
        vertex_count: total_vertex_count as u32,
        bvh_roots,
    }
}