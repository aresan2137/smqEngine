use std::path::Path;

use glam::*;

#[repr(C)]
#[derive(Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BVHNode {
    pub aabb_min: Vec3,
    pub left_first: u32,
    pub aabb_max: Vec3,
    pub tri_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelInstance {
    pub inv_model: Mat4,
    pub bvh_root_node: u32,
    pub id: u32,
    pub _padding: [u32; 2]
}


pub struct LoadedBvhData {
    pub mega_nodes: Vec<BVHNode>,
    pub bvh_roots: Vec<u32>,
}

pub fn save_bvh_data(path: &Path, mega_nodes: &[BVHNode], bvh_roots: &[u32]) -> std::io::Result<()> {
    let mut payload = Vec::new();

    let mega_nodes_count = mega_nodes.len() as u32;
    payload.extend_from_slice(&mega_nodes_count.to_le_bytes());

    let mega_nodes_bytes: &[u8] = bytemuck::cast_slice(mega_nodes);
    payload.extend_from_slice(mega_nodes_bytes);

    let bvh_roots_count = bvh_roots.len() as u32;
    payload.extend_from_slice(&bvh_roots_count.to_le_bytes());

    let bvh_roots_bytes: &[u8] = bytemuck::cast_slice(bvh_roots);
    payload.extend_from_slice(bvh_roots_bytes);

    std::fs::write(path, payload)
}

pub fn load_bvh_data(data: &[u8]) -> std::io::Result<LoadedBvhData> {
    let mut offset = 0;

    let mega_nodes_count = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
    offset += 4;

    let nodes_byte_size = mega_nodes_count * std::mem::size_of::<BVHNode>();
    let nodes_bytes = &data[offset..offset + nodes_byte_size];
    
    let mega_nodes: Vec<BVHNode> = bytemuck::cast_slice(nodes_bytes).to_vec();
    offset += nodes_byte_size;

    let bvh_roots_count = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
    offset += 4;

    let roots_byte_size = bvh_roots_count * std::mem::size_of::<u32>();
    let roots_bytes = &data[offset..offset + roots_byte_size];
    
    let bvh_roots: Vec<u32> = bytemuck::cast_slice(roots_bytes).to_vec();

    return Ok(LoadedBvhData {
        mega_nodes,
        bvh_roots,
    });
}