use wgpu::*;

use crate::{BindingS, Context};

pub struct RenderTextureAttachment {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    pub format: TextureFormat
}

pub struct RenderTexture {
    pub width: u32,
    pub height: u32,
    pub attachments: Vec<RenderTextureAttachment>,
    pub depth_texture: Option<(Texture, TextureView, TextureFormat)>
}

#[allow(dead_code)]
impl RenderTexture {
    pub fn new(context: &Context, width: u32, height: u32, attachments_data: &[(TextureFormat, TextureUsages, Option<&str>)], depth_data: Option<(TextureFormat, TextureUsages, Option<&str>)>) -> Self {
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1
        };

        let mut attachments = Vec::new();

        for attachment in attachments_data.iter() {
            let texture = context.device.create_texture(&TextureDescriptor {
                label: attachment.2,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: attachment.0,
                usage: attachment.1,
                view_formats: &[]
            });

            let view = texture.create_view(&TextureViewDescriptor::default());
            
            let sampler = context.device.create_sampler(&SamplerDescriptor {
                label: None,
                address_mode_u: AddressMode::ClampToEdge,
                address_mode_v: AddressMode::ClampToEdge,
                address_mode_w: AddressMode::ClampToEdge,
                mag_filter: FilterMode::Nearest,
                min_filter: FilterMode::Nearest,
                mipmap_filter: FilterMode::Nearest,
                ..Default::default()
            });

            attachments.push(RenderTextureAttachment {
                texture,
                view,
                sampler,
                format: attachment.0
            });
        }

        let mut depth_texture = None;

        if let Some(depth) = depth_data {            
            let tex = context.device.create_texture(&TextureDescriptor {
                label: depth.2,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: depth.0,
                usage: depth.1,
                view_formats: &[]
            });

            let view = tex.create_view(&TextureViewDescriptor::default());

            depth_texture = Some((tex, view, depth.0));
        }

        Self {
            width,
            height,
            attachments,
            depth_texture,
        }
    }

    pub fn get_depth_binding(&self, visibility: ShaderStages) -> BindingS<'_>  {
        BindingS {
            entry_layout: BindGroupLayoutEntry { 
                binding: 0,
                visibility, 
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Depth,
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false
                }, 
                count: None
            },
            entry: BindGroupEntry { 
                binding: 0,
                resource: BindingResource::TextureView(&self.depth_texture.as_ref().expect("trying to get depth binding when depth doesn't exist").1) 
            }
        }
    }

    pub fn create_egui_texture_id(&self, context: &mut Context, attachment: i32, filter: FilterMode) -> egui::TextureId {
        let view = if attachment == -1 {
            &self.depth_texture.as_ref().expect("tried to get egui TextureID for depth when depth doesn't exist").1
        } else {
            &self.attachments[attachment as usize].view
        };

        return context.egui_renderer.register_native_texture(&context.device, view, filter);
    }
}

impl RenderTextureAttachment {
    pub fn get_texture_binding(&self, visibility: ShaderStages) -> BindingS<'_> {
        BindingS {
            entry_layout: BindGroupLayoutEntry { 
                binding: 0,
                visibility, 
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { 
                        filterable: true 
                    },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false
                }, 
                count: None
            },
            entry: BindGroupEntry { 
                binding: 0,
                resource: BindingResource::TextureView(&self.view) 
            }
        }
    }

    pub fn get_writible_texture_binding(&self, visibility: ShaderStages, access: StorageTextureAccess) -> BindingS<'_> {
        BindingS {
            entry_layout: BindGroupLayoutEntry { 
                binding: 0,                
                visibility,                
                ty: BindingType::StorageTexture {
                    access,
                    format: self.format,
                    view_dimension: TextureViewDimension::D2,
                }, 
                count: None
            },
            entry: BindGroupEntry { 
                binding: 0, 
                resource: BindingResource::TextureView(&self.view) 
            }
        }
    }

    pub fn get_sampler_binding(&self, visibility: ShaderStages) -> BindingS<'_> {
        BindingS {
            entry_layout: BindGroupLayoutEntry { 
                binding: 0,
                visibility,
                ty: BindingType::Sampler(SamplerBindingType::Filtering), 
                count: None
            },
            entry: BindGroupEntry { 
                binding: 0,
                resource: BindingResource::Sampler(&self.sampler) 
            }
        }
    }
}