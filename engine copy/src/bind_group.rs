use wgpu::*;

use crate::*;

pub struct BindingS<'a> {
    pub entry_layout: BindGroupLayoutEntry,
    pub entry: BindGroupEntry<'a>
}

pub struct BindGroupS {
    pub bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup
}

impl BindGroupS {
    pub fn from_bindings(context: &Context, bindings: Vec<BindingS>) -> Self {
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
            label: None, 
            entries: layout_entries.as_slice()
        });

        let bind_group = context.device.create_bind_group(&BindGroupDescriptor { 
            label: None, 
            layout: &bind_group_layout, 
            entries: entries.as_slice()
        });

        return Self { 
            bind_group_layout,
            bind_group 
        };
    }
}