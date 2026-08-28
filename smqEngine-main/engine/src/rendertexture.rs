use wgpu::*;

use crate::*;

#[derive(serde::Serialize, serde::Deserialize, Default, Clone, PartialEq)]
pub struct RenderTextureTextureEdit {
    pub format: String,
    pub usage: Vec<String>
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone, PartialEq)]
pub struct RenderTextureEdit {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub color_attachments: Vec<RenderTextureTextureEdit>,
    pub depth_format: Option<String>
}

pub struct RenderTextureAttachment {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler
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

pub struct RenderTexture {
    pub width: u32,
    pub height: u32,
    pub attachments: Vec<RenderTextureAttachment>,
    pub depth_texture: Option<Texture>,
    pub depth_view: Option<TextureView>,
}

#[allow(dead_code)]
impl RenderTexture {
    pub fn from_json(context: &Context, json_str: &str) -> Self {
        let edit_data: RenderTextureEdit = serde_json::from_str(json_str).expect("failed to parse renderTexture json");

        let width = edit_data.width;
        let height = edit_data.height;

        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1
        };

        let mut attachments = Vec::new();

        for (_i, attachment_edit) in edit_data.color_attachments.iter().enumerate() {
            let format = json_helper::str_to_texture_format(&attachment_edit.format);
            let usage = json_helper::parse_texture_usages(&attachment_edit.usage);

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

        if let Some(depth_format_str) = &edit_data.depth_format {
            let format = json_helper::str_to_texture_format(depth_format_str);
            
            let tex = context.device.create_texture(&TextureDescriptor {
                label: None,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
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