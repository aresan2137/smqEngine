use wgpu::*;

use crate::*;
pub struct Mesh {
    pub vertex_count: u32,
    pub buffer: Buffer
}

impl Mesh {
    pub fn new(context: &Context, data: &[u8], stride: u64) -> Self {
        if data.len() < 9 || data[0] != 0b10110000 {
            panic!("failed to parse smf file invalid header");
        }

        let mut offset = 1;

        let vertex_count = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;

        let _index_count = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;

        let verts_data_size = vertex_count * stride as usize;
        if data.len() < offset + verts_data_size {
            panic!("smf file corrupted");
        }

        let vertex_data = &data[offset..offset + verts_data_size];

        let buffer_size = verts_data_size as u64;

        let buffer = context.device.create_buffer(&BufferDescriptor {
            label: None,
            mapped_at_creation: false,
            size: buffer_size,
            usage: BufferUsages::COPY_DST | BufferUsages::VERTEX | BufferUsages::STORAGE 
        });

        context.queue.write_buffer(&buffer, 0, vertex_data);

        Self {  
            vertex_count: vertex_count as u32,
            buffer
        }
    }

    pub fn from_raw_bytes(context: &Context, vertex_count: u32, vertex_data: &[u8]) -> Self {
        let buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Mesh Raw Buffer"),
            mapped_at_creation: false,
            size: vertex_data.len() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
        });
        context.queue.write_buffer(&buffer, 0, vertex_data);
        Self { vertex_count, buffer }
    }

    pub fn get_storage_binding(&self) -> BindingS<'_> {
        BindingS {
            entry_layout: wgpu::BindGroupLayoutEntry { 
                binding: 0, 
                visibility: wgpu::ShaderStages::COMPUTE, 
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None
            },
            entry: wgpu::BindGroupEntry { binding: 0, resource: self.buffer.as_entire_binding() }
        }
    }
}