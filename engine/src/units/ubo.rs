use wgpu::*;
use bytemuck::Pod;

use crate::{BindingS, Context};

pub struct DynamicUbo<T> {
    pub data: Vec<T>,
    pub buffer: Buffer,
    pub size: u64
}

impl<T: Pod> DynamicUbo<T> {
    pub fn new(context: &Context, data: Vec<T>) -> Self {
        let size = std::mem::size_of::<T>() as u64;

        assert!( // it would be nice for it to be compile time
            size % 256 == 0, 
            "UBO type must be multiple of 256 its: {}", 
            size
        );

        let buffer_size = size * data.len() as u64;

        let buffer = context.device.create_buffer(&BufferDescriptor { 
            label: None, 
            size: buffer_size, 
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM, 
            mapped_at_creation: false 
        });

        return Self { 
            data,
            buffer,
            size
        };
    }

    pub fn upload_data(&self, context: &Context) {
        context.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.data));
    }

    pub fn get_binding(&self, visibility: ShaderStages) -> BindingS<'_> {
        return BindingS {
            entry_layout: BindGroupLayoutEntry {
                binding: 0,
                visibility: visibility,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: None
                },
                count: None
            }, 
            entry: BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &self.buffer,
                    offset: 0,
                    size: BufferSize::new(self.size.clone())
                })
            } 
        };
    }
}

pub struct Ubo<T> {
    pub data: T,
    pub buffer: Buffer,
    pub size: u64
}

impl<T: Pod> Ubo<T> {
    pub fn new(context: &Context, data: T) -> Self {
        let size = std::mem::size_of::<T>() as u64;

        let buffer = context.device.create_buffer(&BufferDescriptor { 
            label: None, 
            size: size, 
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            mapped_at_creation: false 
        });

        return Self { 
            data,
            buffer,
            size
        };
    }

    pub fn upload_data(&self, context: &Context) {
        context.queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&self.data));
    }

    pub fn get_binding(&self, visibility: ShaderStages) -> BindingS<'_> {
        return BindingS { 
            entry_layout: BindGroupLayoutEntry {
                binding: 0,
                visibility: visibility,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }, 
            entry: BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &self.buffer,
                    offset: 0,
                    size: BufferSize::new(self.size.clone())
                })
            }
        };
    }
}