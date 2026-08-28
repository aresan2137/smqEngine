use image::GenericImageView;
use wgpu::*;

use crate::*;

pub struct TextureS {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler
}

#[derive(PartialEq)]
pub enum SamplingMode {
    Nearest,
    Bilinear,
    Trilinear
}

impl TextureS {
    pub fn new(context: &Context, bytes: &[u8], sampling: SamplingMode, mitmap_level: u32, format: TextureFormat, repeater: AddressMode, label: Option<&str>) -> Self {
        let img = image::load_from_memory(bytes).expect("failed to load texture");
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1
        };

        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: mitmap_level,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
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
        let sampler = context.device.create_sampler(&SamplerDescriptor { 
            label: None, 
            address_mode_u: repeater, 
            address_mode_v: repeater, 
            address_mode_w: repeater, 
            mag_filter: if sampling == SamplingMode::Nearest { FilterMode::Nearest } else { FilterMode::Linear }, 
            min_filter: if sampling == SamplingMode::Nearest { FilterMode::Nearest } else { FilterMode::Linear }, 
            mipmap_filter: if sampling == SamplingMode::Trilinear { FilterMode::Linear } else { FilterMode::Nearest }, 
            ..Default::default()
        });

        return Self { 
            texture, 
            view, 
            sampler
        };
    }

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
                    multisampled: false
                },
                count: None
            },
            entry: BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&self.view)
            },
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
            },
        }
    }
}