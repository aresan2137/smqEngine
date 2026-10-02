use std::path::Path;

pub fn path_to_var(path: &Path) -> String {
    return path
        .display()
        .to_string()
        .replace("\\", "/")
        .replace("smq_proj/assets/", "")
        .replace("smq_proj/data/", "")
        .replace("/", "_")
        .replace(".", "_");
}

pub fn sampler_str_to_code(strng: &str) -> String {
    return match strng {
        "Nearest" => "smq_engine::SamplingMode::Nearest",
        "Bilinear" => "smq_engine::SamplingMode::Bilinear",
        "Trilinear" => "smq_engine::SamplingMode::Trilinear",
        "NearestMapped" => "smq_engine::SamplingMode::NearestMapped",
        _ => panic!("unknown sampler str: {strng}"),
    }
    .to_string();
}

pub fn adresat_str_to_code(strng: &str) -> String {
    return match strng {
        "Clamp" => "AddressMode::ClampToEdge",
        "Repeat" => "AddressMode::Repeat",
        _ => panic!("unknown adresat str: {strng}"),
    }
    .to_string();
}

pub fn texture_format_str_to_code(strng: &str) -> String {
    return match strng {
        "R8Unorm" => "TextureFormat::R8Unorm",
        "R8Snorm" => "TextureFormat::R8Snorm",
        "R8Uint" => "TextureFormat::R8Uint",
        "R8Sint" => "TextureFormat::R8Sint",

        "R16Uint" => "TextureFormat::R16Uint",
        "R16Sint" => "TextureFormat::R16Sint",
        "R16Float" => "TextureFormat::R16Float",
        "Rg8Unorm" => "TextureFormat::Rg8Unorm",
        "Rg8Snorm" => "TextureFormat::Rg8Snorm",
        "Rg8Uint" => "TextureFormat::Rg8Uint",
        "Rg8Sint" => "TextureFormat::Rg8Sint",

        "R32Uint" => "TextureFormat::R32Uint",
        "R32Sint" => "TextureFormat::R32Sint",
        "R32Float" => "TextureFormat::R32Float",
        "Rg16Uint" => "TextureFormat::Rg16Uint",
        "Rg16Sint" => "TextureFormat::Rg16Sint",
        "Rg16Float" => "TextureFormat::Rg16Float",
        "Rgba8Unorm" => "TextureFormat::Rgba8Unorm",
        "Rgba8UnormSrgb" => "TextureFormat::Rgba8UnormSrgb",
        "Rgba8Snorm" => "TextureFormat::Rgba8Snorm",
        "Rgba8Uint" => "TextureFormat::Rgba8Uint",
        "Rgba8Sint" => "TextureFormat::Rgba8Sint",
        "Bgra8Unorm" => "TextureFormat::Bgra8Unorm",
        "Bgra8UnormSrgb" => "TextureFormat::Bgra8UnormSrgb",

        "Rgb9e5Ufloat" => "TextureFormat::Rgb9e5Ufloat",
        "Rgb10a2Unorm" => "TextureFormat::Rgb10a2Unorm",
        "Rg11b10Float" => "TextureFormat::Rg11b10Float",

        "Rg32Uint" => "TextureFormat::Rg32Uint",
        "Rg32Sint" => "TextureFormat::Rg32Sint",
        "Rg32Float" => "TextureFormat::Rg32Float",
        "Rgba16Uint" => "TextureFormat::Rgba16Uint",
        "Rgba16Sint" => "TextureFormat::Rgba16Sint",
        "Rgba16Float" => "TextureFormat::Rgba16Float",

        "Rgba32Uint" => "TextureFormat::Rgba32Uint",
        "Rgba32Sint" => "TextureFormat::Rgba32Sint",
        "Rgba32Float" => "TextureFormat::Rgba32Float",

        "Stencil8" => "TextureFormat::Stencil8",
        "Depth16Unorm" => "TextureFormat::Depth16Unorm",
        "Depth24Plus" => "TextureFormat::Depth24Plus",
        "Depth24PlusStencil8" => "TextureFormat::Depth24PlusStencil8",
        "Depth32Float" => "TextureFormat::Depth32Float",
        "Depth32FloatStencil8" => "TextureFormat::Depth32FloatStencil8",

        "Bc1RgbaUnorm" => "TextureFormat::Bc1RgbaUnorm",
        "Bc1RgbaUnormSrgb" => "TextureFormat::Bc1RgbaUnormSrgb",
        "Bc2RgbaUnorm" => "TextureFormat::Bc2RgbaUnorm",
        "Bc2RgbaUnormSrgb" => "TextureFormat::Bc2RgbaUnormSrgb",
        "Bc3RgbaUnorm" => "TextureFormat::Bc3RgbaUnorm",
        "Bc3RgbaUnormSrgb" => "TextureFormat::Bc3RgbaUnormSrgb",
        "Bc4RUnorm" => "TextureFormat::Bc4RUnorm",
        "Bc4RSnorm" => "TextureFormat::Bc4RSnorm",
        "Bc5RgUnorm" => "TextureFormat::Bc5RgUnorm",
        "Bc5RgSnorm" => "TextureFormat::Bc5RgSnorm",
        "Bc6hRgbUfloat" => "TextureFormat::Bc6hRgbUfloat",
        "Bc6hRgbFloat" => "TextureFormat::Bc6hRgbFloat",
        "Bc7RgbaUnorm" => "TextureFormat::Bc7RgbaUnorm",
        "Bc7RgbaUnormSrgb" => "TextureFormat::Bc7RgbaUnormSrgb",

        "Etc2Rgb8Unorm" => "TextureFormat::Etc2Rgb8Unorm",
        "Etc2Rgb8UnormSrgb" => "TextureFormat::Etc2Rgb8UnormSrgb",
        "Etc2Rgb8A1Unorm" => "TextureFormat::Etc2Rgb8A1Unorm",
        "Etc2Rgb8A1UnormSrgb" => "TextureFormat::Etc2Rgb8A1UnormSrgb",
        "Etc2Rgba8Unorm" => "TextureFormat::Etc2Rgba8Unorm",
        "Etc2Rgba8UnormSrgb" => "TextureFormat::Etc2Rgba8UnormSrgb",
        "EacR11Unorm" => "TextureFormat::EacR11Unorm",
        "EacR11Snorm" => "TextureFormat::EacR11Snorm",
        "EacRg11Unorm" => "TextureFormat::EacRg11Unorm",
        "EacRg11Snorm" => "TextureFormat::EacRg11Snorm",

        _ => panic!("unknown texture format type: {strng}"),
    }
    .to_string();
}
