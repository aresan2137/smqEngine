use std::error::Error;

use wgpu::*;

use crate::{BindingS, Context};

pub struct Mesh {
    pub vertex_count: u32,
    pub buffer: Buffer
}

impl Mesh {
    pub fn new(context: &Context, data: &[u8], stride: u64, label: Option<&str>, usage: BufferUsages) -> Result<Self, Box<dyn Error>> {
        if data.len() < 9 || data[0] != 0b10110000 {
            return Err("failed to parse smf file: file too short".into());
        }

        let mut offset = 1;

        let vertex_count = u32::from_le_bytes(data[offset..offset + 4].try_into()?) as usize;
        offset += 4;

        let _index_count = u32::from_le_bytes(data[offset..offset + 4].try_into()?) as usize;
        offset += 4;

        let verts_data_size = vertex_count * stride as usize;
        if data.len() < offset + verts_data_size {
            return Err("declared more vertex thata than exists".into());
        }

        let vertex_data = &data[offset..offset + verts_data_size];

        let buffer_size = verts_data_size as u64;

        let buffer = context.device.create_buffer(&BufferDescriptor {
            label,
            mapped_at_creation: false,
            size: buffer_size,
            usage 
        });

        context.queue.write_buffer(&buffer, 0, vertex_data);

        return Ok(Self {
            vertex_count: vertex_count as u32,
            buffer
        });
    }

    pub fn from_raw_data(context: &Context, vertex_count: u32, vertex_data: &[u8], label: Option<&str>, usage: BufferUsages) -> Self {
        let buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            label,
            mapped_at_creation: false,
            size: vertex_data.len() as u64,
            usage
        });

        context.queue.write_buffer(&buffer, 0, vertex_data);

        return Self { 
            vertex_count,
            buffer 
        };
    }

    pub fn get_storage_binding(&self, visibility: ShaderStages, ty: BufferBindingType) -> BindingS<'_> {
        return BindingS {
            entry_layout: BindGroupLayoutEntry { 
                binding: 0, 
                visibility, 
                ty: BindingType::Buffer { 
                    ty,
                    has_dynamic_offset: false, 
                    min_binding_size: None 
                }, 
                count: None
            },
            entry: BindGroupEntry { 
                binding: 0, 
                resource: self.buffer.as_entire_binding() 
            }
        };
    }
}