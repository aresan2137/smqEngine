use wgpu::*;

use crate::TextureJson;

pub fn str_to_face_culling(s: &str) -> Option<Face> {
    return match s {
        "Back" => Some(Face::Back),
        "Front" => Some(Face::Front),
        _ => None
    };
}

pub fn parse_sampler<'a>(json: &TextureJson) -> SamplerDescriptor<'a> {
    let (mag_filter, min_filter, mipmap_filter) = match json.sample.to_lowercase().as_str() {
        "bilinear" => {
            (FilterMode::Linear, FilterMode::Linear, FilterMode::Nearest)
        }
        "trilinear" => {
            (FilterMode::Linear, FilterMode::Linear, FilterMode::Linear)
        }
        _ => {
            (FilterMode::Nearest, FilterMode::Nearest, FilterMode::Nearest)
        }
    };

    let address_mode = match json.repeat.to_lowercase().as_str() {
        "clamp" => AddressMode::ClampToEdge,
        "mirror" => AddressMode::MirrorRepeat,
        _ => AddressMode::Repeat
    };

    SamplerDescriptor {
        label: None,
        address_mode_u: address_mode,
        address_mode_v: address_mode,
        address_mode_w: address_mode,
        mag_filter,
        min_filter,
        mipmap_filter,
        ..Default::default()
    }
}
pub fn parse_texture_usages(usages: &[String]) -> TextureUsages {
    let mut result = TextureUsages::empty();
    
    for u in usages {
        result |= match u.as_str() {
            "COPY_SRC" => TextureUsages::COPY_SRC,
            "COPY_DST" => TextureUsages::COPY_DST,
            "TEXTURE_BINDING" => TextureUsages::TEXTURE_BINDING,
            "STORAGE_BINDING" => TextureUsages::STORAGE_BINDING,
            "RENDER_ATTACHMENT" => TextureUsages::RENDER_ATTACHMENT,
            _ => TextureUsages::empty(),
        };
    }

    if result.is_empty() {
        result = TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING;
    }

    return result;
}

pub fn str_to_texture_format(s: &str) -> TextureFormat {
    // Dynamiczny parser dla formatów ASTC (żeby uniknąć pisania kilkudziesięciu kombinacji)
    if s.starts_with("Astc") {
        let remainder = &s[4..];
        let (block, chan_str) = if remainder.starts_with("4x4") { (AstcBlock::B4x4, &remainder[3..]) }
        else if remainder.starts_with("5x4") { (AstcBlock::B5x4, &remainder[3..]) }
        else if remainder.starts_with("5x5") { (AstcBlock::B5x5, &remainder[3..]) }
        else if remainder.starts_with("6x5") { (AstcBlock::B6x5, &remainder[3..]) }
        else if remainder.starts_with("6x6") { (AstcBlock::B6x6, &remainder[3..]) }
        else if remainder.starts_with("8x5") { (AstcBlock::B8x5, &remainder[3..]) }
        else if remainder.starts_with("8x6") { (AstcBlock::B8x6, &remainder[3..]) }
        else if remainder.starts_with("8x8") { (AstcBlock::B8x8, &remainder[3..]) }
        else if remainder.starts_with("10x5") { (AstcBlock::B10x5, &remainder[4..]) }
        else if remainder.starts_with("10x6") { (AstcBlock::B10x6, &remainder[4..]) }
        else if remainder.starts_with("10x8") { (AstcBlock::B10x8, &remainder[4..]) }
        else if remainder.starts_with("10x10") { (AstcBlock::B10x10, &remainder[5..]) }
        else if remainder.starts_with("12x10") { (AstcBlock::B12x10, &remainder[5..]) }
        else if remainder.starts_with("12x12") { (AstcBlock::B12x12, &remainder[5..]) }
        else { return TextureFormat::Bgra8UnormSrgb; };

        let channel = match chan_str {
            "RgbaUnorm" => AstcChannel::Unorm,
            "RgbaUnormSrgb" => AstcChannel::UnormSrgb,
            "RgbaHdr" => AstcChannel::Hdr,
            _ => return TextureFormat::Bgra8UnormSrgb,
        };

        return TextureFormat::Astc { block, channel };
    }

    match s {
        // --- Formaty 8-bitowe (Jednokanałowe) ---
        "R8Unorm" => TextureFormat::R8Unorm,
        "R8Snorm" => TextureFormat::R8Snorm,
        "R8Uint" => TextureFormat::R8Uint,
        "R8Sint" => TextureFormat::R8Sint,

        // --- Formaty 16-bitowe ---
        "R16Uint" => TextureFormat::R16Uint,
        "R16Sint" => TextureFormat::R16Sint,
        "R16Unorm" => TextureFormat::R16Unorm,
        "R16Snorm" => TextureFormat::R16Snorm,
        "R16Float" => TextureFormat::R16Float,
        "Rg8Unorm" => TextureFormat::Rg8Unorm,
        "Rg8Snorm" => TextureFormat::Rg8Snorm,
        "Rg8Uint" => TextureFormat::Rg8Uint,
        "Rg8Sint" => TextureFormat::Rg8Sint,

        // --- Formaty 32-bitowe ---
        "R32Uint" => TextureFormat::R32Uint,
        "R32Sint" => TextureFormat::R32Sint,
        "R32Float" => TextureFormat::R32Float,
        "Rg16Uint" => TextureFormat::Rg16Uint,
        "Rg16Sint" => TextureFormat::Rg16Sint,
        "Rg16Unorm" => TextureFormat::Rg16Unorm,
        "Rg16Snorm" => TextureFormat::Rg16Snorm,
        "Rg16Float" => TextureFormat::Rg16Float, // Twoja zguba!
        "Rgba8Unorm" => TextureFormat::Rgba8Unorm,
        "Rgba8UnormSrgb" => TextureFormat::Rgba8UnormSrgb,
        "Rgba8Snorm" => TextureFormat::Rgba8Snorm,
        "Rgba8Uint" => TextureFormat::Rgba8Uint,
        "Rgba8Sint" => TextureFormat::Rgba8Sint,
        "Bgra8Unorm" => TextureFormat::Bgra8Unorm,
        "Bgra8UnormSrgb" => TextureFormat::Bgra8UnormSrgb,
        "Rgb10a2Unorm" => TextureFormat::Rgb10a2Unorm,
        "Rgb10a2Uint" => TextureFormat::Rgb10a2Uint,

        // --- Formaty 64-bitowe ---
        "Rg32Uint" => TextureFormat::Rg32Uint,
        "Rg32Sint" => TextureFormat::Rg32Sint,
        "Rg32Float" => TextureFormat::Rg32Float,
        "Rgba16Uint" => TextureFormat::Rgba16Uint,
        "Rgba16Sint" => TextureFormat::Rgba16Sint,
        "Rgba16Unorm" => TextureFormat::Rgba16Unorm,
        "Rgba16Snorm" => TextureFormat::Rgba16Snorm,
        "Rgba16Float" => TextureFormat::Rgba16Float,

        // --- Formaty 128-bitowe ---
        "Rgba32Uint" => TextureFormat::Rgba32Uint,
        "Rgba32Sint" => TextureFormat::Rgba32Sint,
        "Rgba32Float" => TextureFormat::Rgba32Float,

        // --- Formaty Depth (Głębokość) i Stencil (Szablon) ---
        "Depth16Unorm" => TextureFormat::Depth16Unorm,
        "Depth24Plus" => TextureFormat::Depth24Plus,
        "Depth24PlusStencil8" => TextureFormat::Depth24PlusStencil8,
        "Depth32Float" => TextureFormat::Depth32Float,
        "Depth32FloatStencil8" => TextureFormat::Depth32FloatStencil8,
        "Stencil8" => TextureFormat::Stencil8,

        // --- Kompresja BC (Block Compression - S3TC / DirectX) ---
        "Bc1RgbaUnorm" => TextureFormat::Bc1RgbaUnorm,
        "Bc1RgbaUnormSrgb" => TextureFormat::Bc1RgbaUnormSrgb,
        "Bc2RgbaUnorm" => TextureFormat::Bc2RgbaUnorm,
        "Bc2RgbaUnormSrgb" => TextureFormat::Bc2RgbaUnormSrgb,
        "Bc3RgbaUnorm" => TextureFormat::Bc3RgbaUnorm,
        "Bc3RgbaUnormSrgb" => TextureFormat::Bc3RgbaUnormSrgb,
        "Bc4RUnorm" => TextureFormat::Bc4RUnorm,
        "Bc4RSnorm" => TextureFormat::Bc4RSnorm,
        "Bc5RgUnorm" => TextureFormat::Bc5RgUnorm,
        "Bc5RgSnorm" => TextureFormat::Bc5RgSnorm,
        "Bc6hRgbUfloat" => TextureFormat::Bc6hRgbUfloat,
        "Bc7RgbaUnorm" => TextureFormat::Bc7RgbaUnorm,
        "Bc7RgbaUnormSrgb" => TextureFormat::Bc7RgbaUnormSrgb,

        // --- Kompresja ETC2 / EAC (Mobile) ---
        "Etc2Rgb8Unorm" => TextureFormat::Etc2Rgb8Unorm,
        "Etc2Rgb8UnormSrgb" => TextureFormat::Etc2Rgb8UnormSrgb,
        "Etc2Rgb8A1Unorm" => TextureFormat::Etc2Rgb8A1Unorm,
        "Etc2Rgb8A1UnormSrgb" => TextureFormat::Etc2Rgb8A1UnormSrgb,
        "Etc2Rgba8Unorm" => TextureFormat::Etc2Rgba8Unorm,
        "Etc2Rgba8UnormSrgb" => TextureFormat::Etc2Rgba8UnormSrgb,
        "EacR11Unorm" => TextureFormat::EacR11Unorm,
        "EacR11Snorm" => TextureFormat::EacR11Snorm,
        "EacRg11Unorm" => TextureFormat::EacRg11Unorm,
        "EacRg11Snorm" => TextureFormat::EacRg11Snorm,

        // --- Fallback (w razie literówki w pliku konfiguracyjnym) ---
        _ => TextureFormat::Bgra8UnormSrgb,
    }
}