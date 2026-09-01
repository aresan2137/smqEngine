use std::{fs, path::{Path, PathBuf}};

use walkdir::WalkDir;
use wgpu::naga::valid::{Validator, Capabilities, ValidationFlags};

use std::error::Error;

mod wgsl_anaizer;
use wgsl_anaizer::*;

use crate::editor::inspector::{RenderTextureJson, process_common_file_descriptor};

pub fn bake() -> Result<(), Box<dyn Error>> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut struct_code= "".to_string();
    let mut fn_code= "".to_string();
    let mut ret_code= "".to_string();
    let mut ret_struct_code= "".to_string();

    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());

    let mut analisys = Vec::new();

    for entry in WalkDir::new("smq_proj/assets").into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let ext = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("");
            
            match ext {
                "wgsl" => {
                    let (file_path, _) = process_common_file_descriptor(&entry.path().to_path_buf());
                    
                    analisys.push(analize_wgsl_file(&file_path, &mut validator)?);
                },
                _ => {}
            }
        }
    }

    bake_auto_ubos(&analisys, &mut struct_code, &mut fn_code, &mut ret_code, &mut ret_struct_code)?;

    for entry in WalkDir::new("smq_proj/assets").into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let path = entry.path();
            let var_name = path_to_var(path);
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

            match ext {
                // "png" => {
                //     let (file_path, json_path) = process_common_file_descriptor(&entry.path().to_path_buf());

                //     let json_content = fs::read_to_string(&json_path)?;
                //     let texture_json: TextureJson = serde_json::from_str(&json_content).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

                //     fn_code += &format!("\n    let {} = TextureS::new(context, ssf_data[{}], {}, {}, {}, {}, Some(\"{}\"));", 
                //         var_name, files.len() as i32, sampler_str_to_code(&texture_json.sampler), texture_json.mipmaps, texture_format_str_to_code(&texture_json.format),
                //         adresat_str_to_code(&texture_json.repeater), var_name
                //     );

                //     ret_code += &format!("\n    {var_name},");
                //     ret_struct_code += &format!("\n    pub {var_name}: TextureS,");

                //     files.push(file_path);
                // },
                "renderTexture" => {
                    let (_, json_path) = process_common_file_descriptor(&entry.path().to_path_buf());

                    let json_content = fs::read_to_string(&json_path)?;
                    let render_texture_json: RenderTextureJson = serde_json::from_str(&json_content)?;

                    let mut attachments_data = "".to_string();

                    for (i, attachment) in render_texture_json.attachments.iter().enumerate() {
                        let mut usages = "".to_string();

                        if attachment.usages_COPY_DST { usages += &"TextureUsages::COPY_DST | "; } 
                        if attachment.usages_COPY_SRC { usages += &"TextureUsages::COPY_SRC | "; } 
                        if attachment.usages_RENDER_ATTACHMENT { usages += &"TextureUsages::RENDER_ATTACHMENT | "; } 
                        if attachment.usages_STORAGE_ATOMIC { usages += &"TextureUsages::STORAGE_ATOMIC | "; } 
                        if attachment.usages_STORAGE_BINDING { usages += &"TextureUsages::STORAGE_BINDING | "; } 
                        if attachment.usages_TEXTURE_BINDING{ usages += &"TextureUsages::TEXTURE_BINDING | "; }

                        usages += "TextureUsages::empty()";

                        attachments_data += &format!("\n({}, {}, Some(\"{}_{}\")),", texture_format_str_to_code(&attachment.format), usages, var_name, i);
                    }

                    let depth_data = if let Some(depth) = render_texture_json.depth_attachment {
                        let mut usages = "".to_string();

                        if render_texture_json.depth_usages_COPY_DST { usages += &"TextureUsages::COPY_DST | "; } 
                        if render_texture_json.depth_usages_COPY_SRC { usages += &"TextureUsages::COPY_SRC | "; } 
                        if render_texture_json.depth_usages_RENDER_ATTACHMENT { usages += &"TextureUsages::RENDER_ATTACHMENT | "; } 
                        if render_texture_json.depth_usages_STORAGE_ATOMIC { usages += &"TextureUsages::STORAGE_ATOMIC | "; } 
                        if render_texture_json.depth_usages_STORAGE_BINDING { usages += &"TextureUsages::STORAGE_BINDING | "; } 
                        if render_texture_json.depth_usages_TEXTURE_BINDING{ usages += &"TextureUsages::TEXTURE_BINDING | "; }

                        usages += "TextureUsages::empty()";

                        format!("Some(({}, {}, Some(\"{}_depth\")))", texture_format_str_to_code(&depth), usages, var_name)
                    } else {
                        "None".to_string()
                    };

                    fn_code += &format!("\nlet {} = RenderTexture::new(context, {}, {}, &[{}], {});", var_name, render_texture_json.width, render_texture_json.height, attachments_data, depth_data);

                    ret_code += &format!("\n{var_name},");
                    ret_struct_code += &format!("\npub {var_name}: RenderTexture,");

                },
//                 "ubo" => {
//                     let (_, json_path) = process_common_file_descriptor(&entry.path().to_path_buf());

//                     let json_content = fs::read_to_string(&json_path)?;
//                     let ubo_json: UboJson = serde_json::from_str(&json_content).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

//                     let mut offset: u32 = 0;
//                     let mut max_align: u32 = 0;
//                     let mut pad_counter: u32 = 0;

//                     let raw_name = entry.path().display().to_string().replace('\\', "/").replace("smq_proj/assets/", "").replace('/', "_").replace('.', "_");

//                     let struct_name: String = raw_name.split('_').map(|word| {
//                             let mut chars = word.chars();
//                             match chars.next() {
//                                 None => String::new(),
//                                 Some(first_char) => {
//                                     first_char.to_uppercase().collect::<String>() + chars.as_str()
//                                 }
//                             }
//                         }).collect();

//                     struct_code += &format!("\n#[repr(C)]\n#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]\npub struct {} {{", struct_name);

//                     let mut builder = format!("
// impl Default for {} {{
//     fn default() -> Self {{
//         Self {{", struct_name);

//                     for var in &ubo_json.stuff {
//                         let (var_size, var_align) = match var.type_.as_str() {
//                             "i32" | "u32" | "f32" => (4, 4),
//                             "Vec2" | "IVec2" | "UVec2" => (8, 8),
//                             "Vec3" | "IVec3" | "UVec3" => (12, 16),
//                             "Vec4" | "IVec4" | "UVec4" => (16, 16),
//                             "Mat3" => (48, 16), 
//                             "Mat4" => (64, 16),
//                             _ => panic!("unknown ubo format: {}", var.type_),
//                         };

//                         if var_align > max_align { max_align = var_align; }

//                         let aligned_offset = (offset + var_align - 1) & !(var_align - 1);
                        
//                         let padding_needed = aligned_offset - offset;
//                         if padding_needed > 0 {
//                             struct_code += &format!("\n    pub _pad{}: [u8; {}],", pad_counter, padding_needed);
//                             builder += &format!("\n     _pad{}: [0; {}],", pad_counter, padding_needed);
//                             pad_counter += 1;
//                         }

//                         offset = aligned_offset + var_size;

//                         struct_code += &format!("\n    pub {}: {},", var.name, var.type_);

//                         let default_value_str: String = match var.type_.as_str() {
//                             "i32" | "u32" => "0".to_string(),
//                             "f32" => "0.0".to_string(),
                            
//                             "Vec2" | "IVec2" | "UVec2" | 
//                             "Vec3" | "IVec3" | "UVec3" | 
//                             "Vec4" | "IVec4" | "UVec4" => format!("{}::ZERO", var.type_),
                            
//                             "Mat3" | "Mat4" => format!("{}::IDENTITY", var.type_),
                            
//                             _ => panic!("unknown ubo format: {}", var.type_),
//                         };
//                         builder += &format!("\n    {}: {},", var.name, default_value_str);
//                     }

//                     let final_size = if max_align > 0 { (offset + max_align - 1) & !(max_align - 1) } else { 0 };
//                     let end_padding = final_size - offset;
                    
//                     if end_padding > 0 {
//                         struct_code += &format!("\n    pub _pad{}: [u8; {}],", pad_counter, end_padding);
//                         builder += &format!("\n    _pad{}: [0; {}],", pad_counter, end_padding);
//                     }

//                     struct_code += &format!("\n}}\n{builder}\n        }}\n    }}\n}}\n\n");

//                     fn_code += &format!("\n      let mut {} = Ubo::new(context, {}::default());", var_name, struct_name);

//                     ret_code += &format!("\n    {},", var_name);

//                     ret_struct_code += &format!("\n    pub {}: Ubo<{}>,", var_name, struct_name);
//                 },
                // "blend" => {
                //     let (file_path, _) = process_common_file_descriptor(&entry.path().to_path_buf());

                //     for entry in WalkDir::new(file_path.with_extension("")).into_iter().filter_map(|e| e.ok()) {
                //         if entry.path().is_file() {
                //             assets.smfs.push(AssetCreatorSMF { 
                //                 id: files.len() as i32
                //             });

                //             files.push(entry.path().to_path_buf());
                //         }
                //     }                    
                // },
                _ => {}
            }
        }
    }

    // for entry in WalkDir::new("smq_proj/assets").into_iter().filter_map(|e| e.ok()) {
    //     if entry.path().is_file() {
    //         let ext = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("");
            
    //         match ext {
    //             "bindgroup" => {
    //                 let (_, json_path) = process_common_file_descriptor(&entry.path().to_path_buf());

    //                 let json_content = fs::read_to_string(&json_path)?;
    //                 let bindgroup_json: BindGroupJson = serde_json::from_str(&json_content).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    //                 let mut groups = Vec::new();
                    
    //                 for group in bindgroup_json.groups {
    //                     let gupa = group.expect("for a bindgroup some binds are set as none");
    //                     let (from, id) = &asset_mapper[&gupa.potential_path.replace('\\', "/")];

    //                     groups.push(AssetCreatorBindGroupGroup { 
    //                         from: *from, 
    //                         id: *id, 
    //                         render_texture_id: gupa.render_texture_id, 
    //                         visibility_compute: gupa.visibility_compute, 
    //                         visibility_vertex: gupa.visibility_vertex, 
    //                         visibility_fragment: gupa.visibility_fragment 
    //                     });
    //                 }

    //                 assets.bind_groups.push(AssetCreatorBindGroup { 
    //                     groups
    //                 });

    //                 asset_mapper.insert(entry.path().display().to_string().replace('\\', "/"), (FileFrom::BindGroup, assets.bind_groups.len() as i32 - 1));
    //             },
    //             _ => {}
    //         }
    //     }
    // }

    // for entry in WalkDir::new("smq_proj/assets").into_iter().filter_map(|e| e.ok()) {
    //     if entry.path().is_file() {
    //         let ext = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("");
            
    //         match ext {
    //             "wgsl" => {
    //                 let (file_path, json_path) = process_common_file_descriptor(&entry.path().to_path_buf());

    //                 let json_content = fs::read_to_string(&json_path)?;
    //                 let wgsl_json: WgslJson = serde_json::from_str(&json_content).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    //                 let bindgroup0: i32 = if let Some(bindgroup) = &wgsl_json.bindgroups[0] { asset_mapper[&bindgroup.potential_path.replace('\\', "/")].1 } else { -1 };
    //                 let bindgroup1: i32 = if let Some(bindgroup) = &wgsl_json.bindgroups[1] { asset_mapper[&bindgroup.potential_path.replace('\\', "/")].1 } else { -1 };
    //                 let bindgroup2: i32 = if let Some(bindgroup) = &wgsl_json.bindgroups[2] { asset_mapper[&bindgroup.potential_path.replace('\\', "/")].1 } else { -1 };
    //                 let bindgroup3: i32 = if let Some(bindgroup) = &wgsl_json.bindgroups[3] { asset_mapper[&bindgroup.potential_path.replace('\\', "/")].1 } else { -1 };

    //                 if !wgsl_json.is_compute {
    //                     assets.render_wgsl.push(AssetCreatorRenderWgsl { 
    //                         bindgroup0, 
    //                         bindgroup1, 
    //                         bindgroup2,
    //                         bindgroup3, 
    //                         culling: culling_str_to_byte(&wgsl_json.culling), 
    //                         is_full: wgsl_json.is_full, 
    //                         write_depth: wgsl_json.write_depth, 
    //                         depth_compare: compare_str_to_byte(&wgsl_json.depth_compare), 
    //                         blend_mode: blend_str_to_byte(&wgsl_json.blend_mode), 
    //                         topology: topology_str_to_byte(&wgsl_json.topology),
    //                         id: files.len() as i32
    //                     });
    //                 } else {
    //                     assets.compute_wgsl.push(AssetCreatorComputeWgsl { 
    //                         bindgroup0, 
    //                         bindgroup1, 
    //                         bindgroup2, 
    //                         bindgroup3,
    //                         id: files.len() as i32
    //                     });
    //                 }

    //                 files.push(file_path);
    //             },
    //             _ => {}
    //         }
    //     }
    // }

    // let mut comp: Vec<(PathBuf, String, u8)> = Vec::new();
    // comp.reserve(files.len());

    // for file in files {
    //     comp.push((file.clone(), file.display().to_string().replace("\\", "/").replace("smq_proj/data/", "").replace("/", "_").replace(".", "_"), 0x00));
    // }

    // save_custom_ssf("game", comp);


    let buttom = fs::read_to_string("smq_proj/config/build/inject/buttom.txt")?;
    let outside = fs::read_to_string("smq_proj/config/build/inject/outside.txt")?;
    let ret_struct = fs::read_to_string("smq_proj/config/build/inject/ret_struct.txt")?;
    let ret = fs::read_to_string("smq_proj/config/build/inject/ret.txt")?;
    let top = fs::read_to_string("smq_proj/config/build/inject/top.txt")?;


let code = format!("
// this code is generated by code
// if you change this file your changes will be deleted

#![allow(non_snake_case, unused)]

use glam::*;
use smq_engine::*;
use wgpu::*;

{struct_code}

{outside}

pub struct GenAssets {{
    {ret_struct_code}
    {ret_struct}
}}

impl GenAssets {{
    pub fn init_gen_assets(context: &Context) -> Self {{
        {top}
        {fn_code}
        {buttom}
        return Self {{
            {ret_code}
            {ret}
        }};
    }}
}}
");

    fs::write("game/src/gen_bake.rs", code)?;

    std::process::Command::new("rustfmt").arg("game/src/gen_bake.rs").status().unwrap();

    return Ok(());
}

fn bake_auto_ubos(analisys: &[(Vec<WgslBinding>, Vec<WgslStruct>)], struct_code: &mut String, fn_code: &mut String, ret_code: &mut String, ret_struct_code: &mut String) -> Result<(), Box<dyn Error>> {
    let ubos = wgsl_analisys_to_ubos(analisys)?;

    for ubo in ubos.iter() {
        let struct_name = &ubo.name;
        let var_name = struct_name.to_lowercase(); 

        let mut current_offset: u32 = 0;
        let mut max_align: u32 = 0;
        let mut pad_counter: u32 = 0;

        *struct_code += &format!("\n#[repr(C)]\n#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]\npub struct {} {{", struct_name);

        let mut builder = format!("
            impl Default for {} {{
                fn default() -> Self {{
                    Self {{", struct_name
        );

        for var in &ubo.vars {
            let (var_size, var_align) = match var.type_.as_str() {
                "i32" | "u32" | "f32" => (4, 4),
                "Vec2" | "IVec2" | "UVec2" => (8, 8),
                "Vec3" | "IVec3" | "UVec3" => (12, 16),
                "Vec4" | "IVec4" | "UVec4" => (16, 16),
                "Mat3" => (48, 16), 
                "Mat4" => (64, 16),
                _ => return Err(format!("unknown ubo format: {}", var.type_).into())
            };

            if var_align > max_align { max_align = var_align; }

            if var.offset > current_offset {
                let padding_needed = var.offset - current_offset;
                
                *struct_code += &format!("\npub _pad{}: [u8; {}],", pad_counter, padding_needed);
                builder += &format!("\n_pad{}: [0; {}],", pad_counter, padding_needed);
                
                pad_counter += 1;
                current_offset = var.offset;
            } else if var.offset < current_offset {
                return Err(format!("overlap in: {} at: {}", struct_name, var.name).into());
            }

            *struct_code += &format!("\npub {}: {},", var.name, var.type_);

            let default_value_str: String = match var.type_.as_str() {
                "i32" | "u32" => "0".to_string(),
                "f32" => "0.0".to_string(),
                "Vec2" | "IVec2" | "UVec2" | 
                "Vec3" | "IVec3" | "UVec3" | 
                "Vec4" | "IVec4" | "UVec4" => format!("{}::ZERO", var.type_),
                "Mat3" | "Mat4" => format!("{}::IDENTITY", var.type_),
                _ => return Err(format!("unknown ubo format: {}", var.type_).into()),
            };
            
            builder += &format!("\n{}: {},", var.name, default_value_str);

            current_offset += var_size;
        }

        let final_size = if max_align > 0 { (current_offset + max_align - 1) & !(max_align - 1) } else { 0 };
        
        let end_padding = final_size - current_offset;
        
        if end_padding > 0 {
            *struct_code += &format!("\npub _pad{}: [u8; {}],", pad_counter, end_padding);
            builder += &format!("\n_pad{}: [0; {}],", pad_counter, end_padding);
        }

        *struct_code += &format!("\n}}\n{builder}\n}}\n}}\n}}\n\n");

        *fn_code += &format!("\nlet mut {} = Ubo::new(context, {}::default());", var_name, struct_name);
        *ret_code += &format!("\n{},", var_name);
        *ret_struct_code += &format!("\npub {}: Ubo<{}>,", var_name, struct_name);
    }

    return Ok(());
}

fn path_to_var(path: &Path) -> String {
    return path.display().to_string().replace("\\", "/").replace("smq_proj/assets/", "").replace("/", "_").replace(".", "_");
}

fn sampler_str_to_code(strng: &str) -> String {
    return match strng {
        "Nearest" => "smq_engine::SamplingMode::Nearest",
        "Bilinear" => "smq_engine::SamplingMode::Bilinear",
        "Trilinear" => "smq_engine::SamplingMode::Trilinear",
        _ => panic!("unknown sampler str: {strng}")
    }.to_string();
}

fn adresat_str_to_code(strng: &str) -> String {
    return match strng {
        "Clamp" => "AddressMode::ClampToEdge",
        "Repeat" => "AddressMode::Repeat",
        _ => panic!("unknown adresat str: {strng}")
    }.to_string();
}

fn texture_format_str_to_code(strng: &str) -> String {
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
    }.to_string();
}