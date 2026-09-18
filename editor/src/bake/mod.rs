
use smq_engine::{other::bvh::save_bvh_data, save_custom_ssf};
use walkdir::WalkDir;
use wgpu::naga::valid::{Validator, Capabilities, ValidationFlags};

use std::{error::Error, fmt::format, fs, path::{Path, PathBuf}};

mod wgsl_anaizer;
use wgsl_anaizer::*;

use crate::{editor::{EditorState, RenderTextureJson, TextureJson, write_smf_file}, process_common_file_descriptor};

mod bvh;
use bvh::*;

pub fn bake(state: &EditorState) -> Result<(), Box<dyn Error>> {
    let mut files: Vec<PathBuf> = Vec::new();

    files.extend(bake_assets(state)?);

    let mut smffs: Vec<Vec<u8>> = Vec::with_capacity(files.len());

    for smf in &files {
        if smf.extension().is_some_and(|ext| ext == "smf") {
            let data = fs::read(smf)?;
            smffs.push(data);
        }        
    }

    let smf_slices: Vec<&[u8]> = smffs.iter().map(|v| v.as_slice()).collect();

    let bvh_gen = build_mega_geometry(&smf_slices, 32);

    let bvh_path = PathBuf::from("smq_proj/build/bvh.smf");
    write_smf_file(&bvh_path, bvh_gen.vertex_count, &bvh_gen.mega_vertex_bytes)?;
    files.push(bvh_path);

    let bvh_path_2 = PathBuf::from("smq_proj/build/bvh.bin");
    save_bvh_data(&bvh_path_2, &bvh_gen.mega_nodes, &bvh_gen.bvh_roots)?;
    files.push(bvh_path_2);

    let mut comp: Vec<(PathBuf, String, u8)> = Vec::new();
    comp.reserve(files.len());

    for file in files {
        comp.push((file.clone(), file.display().to_string().replace("\\", "/").replace("smq_proj/data/", "").replace("/", "_").replace(".", "_"), 0x00));
    }

    save_custom_ssf("game", comp);

    log::info!("bake sucess");

    return Ok(());
}

fn bake_assets(state: &EditorState) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files: Vec<PathBuf> = Vec::new();

    let mut struct_code= "".to_string();
    let mut fn_code= "".to_string();
    let mut ret_code= "".to_string();
    let mut ret_struct_code= "".to_string();

    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());

    let mut analisys = (Vec::new(), Vec::new());

    let mut iterer = Vec::new();

    for entry in WalkDir::new("smq_proj/assets").into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let ext = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("");
            
            iterer.push((entry.path().to_path_buf(), ext.to_string()));
        }
    }

    let mut wgsl_analisys_will_fail = false;
    for (path, ext) in &iterer {
        if ext == "wgsl" {
            let (file_path, _) = process_common_file_descriptor(&path);
            
            let analisyse = analize_wgsl_file(&file_path, &mut validator);

            if let Err(e) = &analisyse {
                wgsl_analisys_will_fail = true;
                log::error!("wgsl compile error in: {} error:\n {}", file_path.display(), e);
            } 

            if !wgsl_analisys_will_fail {
                if let Ok(analizen) = analisyse {
                    analisys.0.push(analizen);
                    analisys.1.push(path.clone());
                } 
            }            
        }
    }

    if wgsl_analisys_will_fail {
        return Err("errors while compiling wgsl".into());
    }

    let ubos = wgsl_analisys_to_ubos(&analisys.0)?;

    create_rust_ubo_structs(&ubos, &mut struct_code, &mut fn_code, &mut ret_code, &mut ret_struct_code)?;

    let mut smf_files = 0;
    let mut smf_enum = "".to_string();
    let mut smf_gen = "".to_string();

    for (path, ext) in &iterer {
        let var_name = path_to_var(&path);

        match ext.as_str() {
            "png" => {
                let (file_path, json_path) = process_common_file_descriptor(&path);

                let json_content = fs::read_to_string(&json_path)?;
                let texture_json: TextureJson = serde_json::from_str(&json_content)?;

                fn_code += &format!("\nlet {} = TextureS::new(context, ssf_data[{}].as_ref(), {}, {}, {}, {}, Some(\"{}\"));", 
                    var_name, files.len() as i32, sampler_str_to_code(&texture_json.sampler), texture_json.mipmaps, texture_format_str_to_code(&texture_json.format),
                    adresat_str_to_code(&texture_json.repeater), var_name
                );

                ret_code += &format!("\n{var_name},");
                ret_struct_code += &format!("\npub {var_name}: TextureS,");

                files.push(file_path);
            },
            "renderTexture" => {
                let (_, json_path) = process_common_file_descriptor(&path);

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
            "blend" => {
                let (file_path, _) = process_common_file_descriptor(&path);
                
                for entry in WalkDir::new(file_path.with_extension("")).into_iter().filter_map(|e| e.ok()) {
                    if entry.path().is_file() {
                        
                        let smf_var_name = path_to_var(entry.path());

                        smf_gen += &format!("\nMesh::new(context, ssf_data[{}].as_ref(), 32, Some(\"{}\")).unwrap(),", files.len() as i32, smf_var_name);

                        smf_enum += &format!("{} = {},", smf_var_name, smf_files);

                        smf_files += 1;

                        files.push(entry.path().to_path_buf());
                    }
                }                    
            },
            _ => {}
        
        }
    }

    for i in 0..analisys.0.len() {
        //let analiz = &analisys.0[i];
        //let path = &analisys.1[i];

        //let (file_path, json_path) = process_common_file_descriptor(path);
            
        //let json_content = fs::read_to_string(&json_path)?;
        //let wgsl_json: WgslJson = serde_json::from_str(&json_content)?;

        //let var_name = path_to_var(&path);

        //RenderMaterial::new(context, culling, label, render_texture, module, vert_layout, depth_compare, layout)

        

        // fn_code += &format!("let {} = RenderMaterial::new(context, {}, Some({}), {}, {}, {}, {}, {});",
        //     var_name, 
        // );
    }

    let mut buttom = fs::read_to_string("smq_proj/config/build/inject/buttom.txt")?;
    let outside = fs::read_to_string("smq_proj/config/build/inject/outside.txt")?;
    let mut ret_struct = fs::read_to_string("smq_proj/config/build/inject/ret_struct.txt")?;
    let mut ret = fs::read_to_string("smq_proj/config/build/inject/ret.txt")?;
    let top = fs::read_to_string("smq_proj/config/build/inject/top.txt")?;

    if state.settings.bake_settings.generate_blit_code {
        let attachment_name = path_to_var(&PathBuf::from(state.settings.bake_settings.render_texture_path.clone().ok_or("using blit gen wydouth setting renderTexture to blit")?));
        let attachment = &state.settings.bake_settings.render_texture_attachment;

        buttom += &format!("
            let blit_group = BindGroupS::new(context, &[
                {attachment_name}.attachments[{attachment}].get_texture_binding(ShaderStages::FRAGMENT, true),
                {attachment_name}.attachments[{attachment}].get_sampler_binding(ShaderStages::FRAGMENT)
            ], None);

            let blitinfo = context.create_blit_pipeline(&blit_group);
        ");

        ret += "blitinfo,
            blit_group,";
        ret_struct += "pub blitinfo: BlitInfo,
            pub blit_group: BindGroupS,";
    }

    let code = format!("
// this code is generated by code
// if you change this file your changes will be deleted

#![allow(non_snake_case, unused, non_camel_case_types)]

use glam::*;
use smq_engine::*;
use wgpu::*;

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Meshes {{
{smf_enum}
}}

{struct_code}

{outside}

pub struct GenAssets {{
    {ret_struct_code}
    {ret_struct}
    pub meshes: [Mesh; {smf_files}]
}}

impl GenAssets {{
    pub fn init_gen_assets<D>(context: &Context<D>, ssf_data: &[SSFAsset]) -> Self {{
        let meshes = [
            {smf_gen}
        ];
        {top}
        {fn_code}
        {buttom}
        return Self {{
            {ret_code}
            {ret}
            meshes
        }};
    }}
}}
");

    fs::write("game/src/gen_bake.rs", code)?;

    std::process::Command::new("rustfmt").arg("game/src/gen_bake.rs").status().unwrap();

    return Ok(files);
}

fn path_to_var(path: &Path) -> String {
    return path.display().to_string().replace("\\", "/").replace("smq_proj/assets/", "").replace("smq_proj/data/", "").replace("/", "_").replace(".", "_");
}

fn sampler_str_to_code(strng: &str) -> String {
    return match strng {
        "Nearest" => "smq_engine::SamplingMode::Nearest",
        "Bilinear" => "smq_engine::SamplingMode::Bilinear",
        "Trilinear" => "smq_engine::SamplingMode::Trilinear",
        "NearestMapped" => "smq_engine::SamplingMode::NearestMapped",
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