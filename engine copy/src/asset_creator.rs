use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AssetCreatorTexture {
    pub sampler: u8,
    pub repeater: u8,
    pub format: u8,
    pub mipmaps: u8,
    pub id: i32
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AssetCreatorRenderTextureAttachment {
    pub format: u8,
    pub usages_COPY_DST: bool,
    pub usages_COPY_SRC: bool,
    pub usages_RENDER_ATTACHMENT: bool,
    pub usages_STORAGE_ATOMIC: bool,
    pub usages_STORAGE_BINDING: bool,
    pub usages_TEXTURE_BINDING: bool
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AssetCreatorRenderTexture {
    pub attachments: Vec<AssetCreatorRenderTextureAttachment>,
    pub width: u32,
    pub height: u32,
    pub depth_attachment: u8,
    pub depth_usages_COPY_DST: bool,
    pub depth_usages_COPY_SRC: bool,
    pub depth_usages_RENDER_ATTACHMENT: bool,
    pub depth_usages_STORAGE_ATOMIC: bool,
    pub depth_usages_STORAGE_BINDING: bool,
    pub depth_usages_TEXTURE_BINDING: bool
}

#[derive(Serialize, Deserialize)]
pub struct AssetCreatorUbo {
    pub size: u32,
    pub is_dynamic: u32 // 0 = is_dynamic false. !0 = is_dynamic true & = dynamic count  
}

#[derive(Serialize, Deserialize)]
pub struct AssetCreatorRenderWgsl {
    pub bindgroup0: i32, // -1 = none. = assetID
    pub bindgroup1: i32, // -1 = none. = assetID
    pub bindgroup2: i32, // -1 = none. = assetID
    pub bindgroup3: i32, // -1 = none. = assetID
    pub render_texture: i32,
    pub culling: u8,
    pub is_full: bool,
    pub write_depth: bool,
    pub depth_compare: u8,
    pub blend_mode: u8,
    pub topology: u8,
    pub id: i32
}

#[derive(Serialize, Deserialize)]
pub struct AssetCreatorComputeWgsl {
    pub bindgroup0: i32, // -1 = none. = assetID
    pub bindgroup1: i32, // -1 = none. = assetID
    pub bindgroup2: i32, // -1 = none. = assetID
    pub bindgroup3: i32, // -1 = none. = assetID
    pub id: i32
}

#[derive(Serialize, Deserialize)]
pub struct AssetCreatorBindGroupGroup {
    pub from: FileFrom,
    pub id: i32,
    pub render_texture_id: i32, // -1 = depth. = attachments
    pub visibility_compute: bool,
    pub visibility_vertex: bool,
    pub visibility_fragment: bool
}

#[derive(Serialize, Deserialize)]
pub struct AssetCreatorBindGroup {
    pub groups: Vec<AssetCreatorBindGroupGroup>
}

#[derive(Serialize, Deserialize)]
pub struct AssetCreatorSMF {
    pub id: i32
}

#[derive(Serialize, Deserialize)]
pub struct AssetCreatorData {
    pub render_textures: Vec<AssetCreatorRenderTexture>,
    pub textures: Vec<AssetCreatorTexture>,
    pub ubo: Vec<AssetCreatorUbo>,
    pub render_wgsl: Vec<AssetCreatorRenderWgsl>,
    pub compute_wgsl: Vec<AssetCreatorComputeWgsl>,
    pub bind_groups: Vec<AssetCreatorBindGroup>,
    pub smfs: Vec<AssetCreatorSMF>
}


impl Default for AssetCreatorData {
    fn default() -> Self {
        return Self { 
            render_textures: Vec::new(), 
            textures: Vec::new(), 
            ubo: Vec::new(), 
            render_wgsl: Vec::new(), 
            compute_wgsl: Vec::new(), 
            bind_groups: Vec::new(),
            smfs: Vec::new()
        };
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Serialize, Deserialize)]
enum FileFrom {
    Png = 0,
    RenderTexture = 1,
    Ubo = 2,
    BindGroup = 3
}

pub fn culling_str_to_byte(val: &str) -> u8 {
    match val { 
        "None" => 0x00, 
        "Back" => 0x01, 
        "Front" => 0x02, 
        _ => 0x00 
    }
}

pub fn compare_str_to_byte(val: &str) -> u8 {
    match val {
        "Less" => 0x00, 
        "Greater" => 0x01,
        "Equal" => 0x02,
        "NotEqual" => 0x03,
        "LessOrEqual" => 0x04, 
        "GreaterOrEqual" => 0x05, 
        "Always" => 0x06, 
        "Never" => 0x07,
        _ => panic!("unknown compare type: {}", val)
    }
}

pub fn blend_str_to_byte(val: &str) -> u8 {
    match val { 
        "Opaque" => 0x00, 
        "Alpha Blend" => 0x01, 
        "Additive" => 0x02, 
        _ => panic!("unknown blend type: {}", val) 
    }
}

pub fn topology_str_to_byte(val: &str) -> u8 {
    match val { 
        "TriangleList" => 0x00, 
        "LineList" => 0x01, 
        "PointList" => 0x02, 
        _ => panic!("unknown topology type: {}", val) 
    }
}

pub fn sample_str_to_byte(sample: &str) -> u8 {
    match sample {
        "Nearest" => 0x00,
        "Bilinear" => 0x01,
        "Trilinear" => 0x02,
        _ => panic!("unknown sample type")
    }
}

pub fn repeat_str_to_byte(repeat: &str) -> u8 {
    match repeat {
        "Repeat" => 0x00,
        "Clamp" => 0x01,
        _ => panic!("unknown repeat type")
    }
}

pub fn optioned_format_str_to_byte(format: Option<String>) -> u8 {
    if let Some(str) = format {
        return format_str_to_byte(&str);
    } else {
        return 0xff;
    }
}

pub fn format_str_to_byte(format: &str) -> u8 {
    match format {
        "R8Unorm" => 0x00,
        "R8Snorm" => 0x01,
        "R8Uint" => 0x02,
        "R8Sint" => 0x03,
        "R16Uint" => 0x04,
        "R16Sint" => 0x05,
        "R16Unorm" => 0x06,
        "R16Snorm" => 0x07,
        "R16Float" => 0x08,
        "Rg8Unorm" => 0x09,
        "Rg8Snorm" => 0x0A,
        "Rg8Uint" => 0x0B,
        "Rg8Sint" => 0x0C,
        "R32Uint" => 0x0D,
        "R32Sint" => 0x0E,
        "R32Float" => 0x0F,
        "Rg16Uint" => 0x10,
        "Rg16Sint" => 0x11,
        "Rg16Unorm" => 0x12,
        "Rg16Snorm" => 0x13,
        "Rg16Float" => 0x14,
        "Rgba8Unorm" => 0x15,
        "Rgba8UnormSrgb" => 0x16,
        "Rgba8Snorm" => 0x17,
        "Rgba8Uint" => 0x18,
        "Rgba8Sint" => 0x19,
        "Bgra8Unorm" => 0x1A,
        "Bgra8UnormSrgb" => 0x1B,
        "Rgb10a2Unorm" => 0x1C,
        "Rg11b10Float" => 0x1D,
        "Rg32Uint" => 0x1E,
        "Rg32Sint" => 0x1F,
        "Rg32Float" => 0x20,
        "Rgba16Uint" => 0x21,
        "Rgba16Sint" => 0x22,
        "Rgba16Unorm" => 0x23,
        "Rgba16Snorm" => 0x24,
        "Rgba16Float" => 0x25,
        "Rgba32Uint" => 0x26,
        "Rgba32Sint" => 0x27,
        "Rgba32Float" => 0x28,
        "Stencil8" => 0x29,
        "Depth16Unorm" => 0x2A,
        "Depth24Plus" => 0x2B,
        "Depth24PlusStencil8" => 0x2C,
        "Depth32Float" => 0x2D,
        "Depth32FloatStencil8" => 0x2E,
        "Bc1RgbaUnorm" => 0x2F,
        "Bc1RgbaUnormSrgb" => 0x30,
        "Bc2RgbaUnorm" => 0x31,
        "Bc2RgbaUnormSrgb" => 0x32,
        "Bc3RgbaUnorm" => 0x33,
        "Bc3RgbaUnormSrgb" => 0x34,
        "Bc4RUnorm" => 0x35,
        "Bc4RSnorm" => 0x36,
        "Bc5RgUnorm" => 0x37,
        "Bc5RgSnorm" => 0x38,
        "Bc6hRgbUfloat" => 0x39,
        "Bc6hRgbFloat" => 0x3A,
        "Bc7RgbaUnorm" => 0x3B,
        "Bc7RgbaUnormSrgb" => 0x3C,
        "Rgb9e5Ufloat" => 0x3D,
        "Depth24UnormStencil8" => 0x3E,
        _ => panic!("unknown format: {}", format)
    }
}

pub fn byte_to_cull_mode(val: u8) -> Option<wgpu::Face> {
    match val {
        0x00 => None,
        0x01 => Some(wgpu::Face::Back),
        0x02 => Some(wgpu::Face::Front),
        _ => None,
    }
}

pub fn byte_to_compare_function(val: u8) -> wgpu::CompareFunction {
    match val {
        0x00 => wgpu::CompareFunction::Less,
        0x01 => wgpu::CompareFunction::Greater,
        0x02 => wgpu::CompareFunction::Equal,
        0x03 => wgpu::CompareFunction::NotEqual,
        0x04 => wgpu::CompareFunction::LessEqual,
        0x05 => wgpu::CompareFunction::GreaterEqual,
        0x06 => wgpu::CompareFunction::Always,
        0x07 => wgpu::CompareFunction::Never,
        _ => panic!("unknown compare type: {}", val),
    }
}

pub fn byte_to_blend_state(val: u8) -> Option<wgpu::BlendState> {
    match val {
        0x00 => Some(wgpu::BlendState::REPLACE),
        0x01 => Some(wgpu::BlendState::ALPHA_BLENDING),
        0x02 => Some(wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent::OVER,
        }),
        _ => panic!("unknown blend type: {}", val),
    }
}

pub fn byte_to_topology(val: u8) -> wgpu::PrimitiveTopology {
    match val {
        0x00 => wgpu::PrimitiveTopology::TriangleList,
        0x01 => wgpu::PrimitiveTopology::LineList,
        0x02 => wgpu::PrimitiveTopology::PointList,
        _ => panic!("unknown topology type: {}", val),
    }
}

pub fn byte_to_filter_mode(val: u8) -> (wgpu::FilterMode, wgpu::FilterMode) {
    match val {
        0x00 => (wgpu::FilterMode::Nearest, wgpu::FilterMode::Nearest), // Nearest
        0x01 => (wgpu::FilterMode::Linear, wgpu::FilterMode::Nearest),  // Bilinear
        0x02 => (wgpu::FilterMode::Linear, wgpu::FilterMode::Linear),   // Trilinear
        _ => panic!("unknown filter mode type: {}", val),
    }
}

pub fn byte_to_address_mode(val: u8) -> wgpu::AddressMode {
    match val {
        0x00 => wgpu::AddressMode::Repeat,
        0x01 => wgpu::AddressMode::ClampToEdge,
        _ => panic!("unknown repeat type: {}", val),
    }
}

pub fn byte_to_texture_format(val: u8) -> wgpu::TextureFormat {
    match val {
        0x00 => wgpu::TextureFormat::R8Unorm,
        0x01 => wgpu::TextureFormat::R8Snorm,
        0x02 => wgpu::TextureFormat::R8Uint,
        0x03 => wgpu::TextureFormat::R8Sint,
        0x04 => wgpu::TextureFormat::R16Uint,
        0x05 => wgpu::TextureFormat::R16Sint,
        0x06 => wgpu::TextureFormat::R16Unorm,
        0x07 => wgpu::TextureFormat::R16Snorm,
        0x08 => wgpu::TextureFormat::R16Float,
        0x09 => wgpu::TextureFormat::Rg8Unorm,
        0x0A => wgpu::TextureFormat::Rg8Snorm,
        0x0B => wgpu::TextureFormat::Rg8Uint,
        0x0C => wgpu::TextureFormat::Rg8Sint,
        0x0D => wgpu::TextureFormat::R32Uint,
        0x0E => wgpu::TextureFormat::R32Sint,
        0x0F => wgpu::TextureFormat::R32Float,
        0x10 => wgpu::TextureFormat::Rg16Uint,
        0x11 => wgpu::TextureFormat::Rg16Sint,
        0x12 => wgpu::TextureFormat::Rg16Unorm,
        0x13 => wgpu::TextureFormat::Rg16Snorm,
        0x14 => wgpu::TextureFormat::Rg16Float,
        0x15 => wgpu::TextureFormat::Rgba8Unorm,
        0x16 => wgpu::TextureFormat::Rgba8UnormSrgb,
        0x17 => wgpu::TextureFormat::Rgba8Snorm,
        0x18 => wgpu::TextureFormat::Rgba8Uint,
        0x19 => wgpu::TextureFormat::Rgba8Sint,
        0x1A => wgpu::TextureFormat::Bgra8Unorm,
        0x1B => wgpu::TextureFormat::Bgra8UnormSrgb,
        0x1C => wgpu::TextureFormat::Rgb10a2Unorm,
        0x1E => wgpu::TextureFormat::Rg32Uint,
        0x1F => wgpu::TextureFormat::Rg32Sint,
        0x20 => wgpu::TextureFormat::Rg32Float,
        0x21 => wgpu::TextureFormat::Rgba16Uint,
        0x22 => wgpu::TextureFormat::Rgba16Sint,
        0x23 => wgpu::TextureFormat::Rgba16Unorm,
        0x24 => wgpu::TextureFormat::Rgba16Snorm,
        0x25 => wgpu::TextureFormat::Rgba16Float,
        0x26 => wgpu::TextureFormat::Rgba32Uint,
        0x27 => wgpu::TextureFormat::Rgba32Sint,
        0x28 => wgpu::TextureFormat::Rgba32Float,
        0x29 => wgpu::TextureFormat::Stencil8,
        0x2A => wgpu::TextureFormat::Depth16Unorm,
        0x2B => wgpu::TextureFormat::Depth24Plus,
        0x2C => wgpu::TextureFormat::Depth24PlusStencil8,
        0x2D => wgpu::TextureFormat::Depth32Float,
        0x2E => wgpu::TextureFormat::Depth32FloatStencil8,
        0x2F => wgpu::TextureFormat::Bc1RgbaUnorm,
        0x30 => wgpu::TextureFormat::Bc1RgbaUnormSrgb,
        0x31 => wgpu::TextureFormat::Bc2RgbaUnorm,
        0x32 => wgpu::TextureFormat::Bc2RgbaUnormSrgb,
        0x33 => wgpu::TextureFormat::Bc3RgbaUnorm,
        0x34 => wgpu::TextureFormat::Bc3RgbaUnormSrgb,
        0x35 => wgpu::TextureFormat::Bc4RUnorm,
        0x36 => wgpu::TextureFormat::Bc4RSnorm,
        0x37 => wgpu::TextureFormat::Bc5RgUnorm,
        0x38 => wgpu::TextureFormat::Bc5RgSnorm,
        0x39 => wgpu::TextureFormat::Bc6hRgbUfloat,
        0x3A => wgpu::TextureFormat::Bc6hRgbFloat,
        0x3B => wgpu::TextureFormat::Bc7RgbaUnorm,
        0x3C => wgpu::TextureFormat::Bc7RgbaUnormSrgb,
        0x3D => wgpu::TextureFormat::Rgb9e5Ufloat,
        0xFF => panic!("found none as texture format"),
        _ => panic!("unknown format type: {}", val),
    }
}

pub fn bools_to_texture_usage(copy_dst: bool, copy_src: bool, render_attachment: bool, storage_atomic: bool, storage_binding: bool, texture_binding: bool) -> wgpu::TextureUsages {
    let mut usage = wgpu::TextureUsages::empty();

    if copy_dst {
        usage |= wgpu::TextureUsages::COPY_DST;
    }
    if copy_src {
        usage |= wgpu::TextureUsages::COPY_SRC;
    }
    if render_attachment {
        usage |= wgpu::TextureUsages::RENDER_ATTACHMENT;
    }
    if storage_atomic {
        usage |= wgpu::TextureUsages::STORAGE_ATOMIC;
    }
    if storage_binding {
        usage |= wgpu::TextureUsages::STORAGE_BINDING;
    }
    if texture_binding {
        usage |= wgpu::TextureUsages::TEXTURE_BINDING;
    }
    
    return usage;
}