use wgpu::*;

use crate::*;

pub struct RenderTextureAttachment {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler
}

pub struct RenderTexture {
    pub width: u32,
    pub height: u32,
    pub attachments: Vec<RenderTextureAttachment>,
    pub depth_texture: Option<Texture>,
    pub depth_view: Option<TextureView>,
}

#[allow(dead_code)]
impl RenderTexture {
    pub fn from_asset_creator(context: &Context, creator_data: AssetCreatorRenderTexture) -> Self {
        let width = creator_data.width;
        let height = creator_data.height;

        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1
        };

        let mut attachments = Vec::new();

        for attachment in creator_data.attachments.iter() {
            let format = byte_to_texture_format(attachment.format);
            let usage = bools_to_texture_usage(attachment.usages_COPY_DST, attachment.usages_COPY_SRC, attachment.usages_RENDER_ATTACHMENT, attachment.usages_STORAGE_ATOMIC, attachment.usages_STORAGE_BINDING, attachment.usages_TEXTURE_BINDING);

            let texture = context.device.create_texture(&TextureDescriptor {
                label: None,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format,
                usage,
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
                sampler
            });
        }

        let mut depth_texture = None;
        let mut depth_view = None;

        if creator_data.depth_attachment != 0xff {
            let format = byte_to_texture_format(creator_data.depth_attachment);
            
            let tex = context.device.create_texture(&TextureDescriptor {
                label: None,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format,
                usage: bools_to_texture_usage(creator_data.depth_usages_COPY_DST, creator_data.depth_usages_COPY_SRC, creator_data.depth_usages_RENDER_ATTACHMENT, creator_data.depth_usages_STORAGE_ATOMIC, creator_data.depth_usages_STORAGE_BINDING, creator_data.depth_usages_TEXTURE_BINDING),
                view_formats: &[]
            });

            let view = tex.create_view(&TextureViewDescriptor::default());

            depth_texture = Some(tex);
            depth_view = Some(view);
        }

        Self {
            width,
            height,
            attachments,
            depth_texture,
            depth_view
        }
    }

    pub fn get_depth_binding(&self) -> BindingS<'_>  {
        BindingS {
            entry_layout: BindGroupLayoutEntry { 
                binding: 0,
                visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE, 
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Depth,
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false
                }, 
                count: None
            },
            entry: BindGroupEntry { 
                binding: 0,
                resource: BindingResource::TextureView(self.depth_view.as_ref().expect("trying to get depth binding when depth doesn't exist")) 
            }
        }
    }
}

impl RenderTextureAttachment {
    pub fn get_texture_binding(&self) -> BindingS<'_> {
        BindingS {
            entry_layout: BindGroupLayoutEntry { 
                binding: 0,
                visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE, 
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
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

    pub fn get_writible_texture_binding(&self) -> BindingS<'_> {
        BindingS {
            entry_layout: BindGroupLayoutEntry { 
                binding: 0,                
                visibility: ShaderStages::COMPUTE,                
                ty: BindingType::StorageTexture {
                    access: StorageTextureAccess::WriteOnly,
                    format: TextureFormat::Rgba8Unorm,
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

    pub fn get_sampler_binding(&self) -> BindingS<'_> {
        BindingS {
            entry_layout: BindGroupLayoutEntry { 
                binding: 0,
                visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
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