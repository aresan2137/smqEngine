
use wgpu::*;

use crate::Context;

pub struct BindingS<'a> {
    pub entry_layout: BindGroupLayoutEntry,
    pub entry: BindGroupEntry<'a>
}

pub struct BindGroupS {
    pub bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout
}

impl BindGroupS {
    pub fn new(context: &Context, bindings: &[BindingS], label: Option<&str>) -> Self {
        let mut layout_entries = Vec::new();
        let mut entries = Vec::new();

        for (i, binding) in bindings.iter().enumerate() {
            let mut entry_layout = binding.entry_layout;
            entry_layout.binding = i as u32;
            layout_entries.push(entry_layout);

            let mut entry = binding.entry.clone();
            entry.binding = i as u32;
            entries.push(entry);
        }         

        let bind_group_layout = context.device.create_bind_group_layout(&BindGroupLayoutDescriptor { 
            label, 
            entries: layout_entries.as_slice()
        });

        let bind_group = context.device.create_bind_group(&BindGroupDescriptor { 
            label, 
            layout: &bind_group_layout, 
            entries: entries.as_slice()
        });

        return Self { 
            bind_group_layout,
            bind_group 
        };
    }
}