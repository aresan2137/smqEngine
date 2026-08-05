use image::GenericImageView;
use wgpu::*;

use crate::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TextureJson {
    pub sample: String,
    pub repeat: String
}

impl Default for TextureJson {
    fn default() -> Self {
        Self { 
            sample: "Nearest".to_string(), 
            repeat: "Repeat".to_string() 
        }
    }
}

#[allow(dead_code)]
pub struct TextureS {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler
}

#[allow(dead_code)]
impl TextureS {
    pub fn from_json(context: &Context, bytes: &[u8], json: &str) -> Self {
        let json_data: TextureJson = serde_json::from_str(json).expect("failed to parse texture json");
        
        let img = image::load_from_memory(bytes).expect("failed to load texture");
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1
        };

        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[]
        });

        context.queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All
            },
            &rgba,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1)
            },
            size
        );

        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = context.device.create_sampler(&json_helper::parse_sampler(&json_data));

        return Self { 
            texture, 
            view, 
            sampler
        };
    }

    #[allow(dead_code)]
    pub fn from_color(context: &Context, color: Color) -> Self {
        let data = [color.r as u8, color.g as u8, color.b as u8, color.a as u8];

        let size = Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };

        let texture = context.device.create_texture(&TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[]
        });

        context.queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All
            },
            &data,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1)
            },
            size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = context.device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        return Self { 
            texture, 
            view, 
            sampler
        };
    }
    
    pub fn get_texture_binding(&self, visibility: ShaderStages) -> BindingS<'_> {
        BindingS {
            entry_layout: BindGroupLayoutEntry {
                binding: 0,
                visibility,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            entry: BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&self.view),
            },
        }
    }

    pub fn get_sampler_binding(&self, visibility: ShaderStages) -> BindingS<'_> {
        BindingS {
            entry_layout: BindGroupLayoutEntry {
                binding: 0,
                visibility,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
            entry: BindGroupEntry {
                binding: 0,
                resource: BindingResource::Sampler(&self.sampler),
            },
        }
    }
}