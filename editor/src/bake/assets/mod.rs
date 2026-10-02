use walkdir::WalkDir;
use wgpu::naga::valid::{Capabilities, ValidationFlags, Validator};

use std::{error::Error, fs, path::PathBuf, process::Command};

use crate::{
    bake::wgsl_anaizer::{analize_wgsl_file, create_rust_ubo_structs, wgsl_analisys_to_ubos},
    editor::{EditorState, RenderTextureJson},
};

mod converters;
use converters::*;

mod smf;
use smf::*;

mod png;
use png::*;

pub fn bake_assets(state: &EditorState) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files: Vec<PathBuf> = Vec::new();

    let mut struct_code = "".to_string();
    let mut fn_code = "".to_string();
    let mut ret_code = "".to_string();
    let mut ret_struct_code = "".to_string();

    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());

    let mut analisys = (Vec::new(), Vec::new());

    let mut iterer = Vec::new();

    for entry in WalkDir::new("smq_proj/assets")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            let ext = entry
                .path()
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("");

            iterer.push((entry.path().to_path_buf(), ext.to_string()));
        }
    }

    let mut wgsl_analisys_will_fail = false;
    for (path, ext) in &iterer {
        if ext == "wgsl" {
            let analisyse = analize_wgsl_file(&path, &mut validator);

            if let Err(e) = &analisyse {
                wgsl_analisys_will_fail = true;
                log::error!(
                    "wgsl compile error in: {} error:\n {}",
                    path.display(),
                    e
                );
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

    create_rust_ubo_structs(
        &ubos,
        &mut struct_code,
        &mut fn_code,
        &mut ret_code,
        &mut ret_struct_code,
    )?;

    let (smf_files, smf_enum, smf_gen) = create_smf_bake(&iterer, &mut files);
    let (png_files, png_enum, png_gen) = create_png_bake(&iterer, &mut files)?;

    for (path, ext) in &iterer {
        let var_name = path_to_var(&path);

        match ext.as_str() {
            "renderTexture" => {
                let json_content = fs::read_to_string(&path)?;
                let render_texture_json: RenderTextureJson = serde_json::from_str(&json_content)?;

                let mut attachments_data = "".to_string();

                for (i, attachment) in render_texture_json.attachments.iter().enumerate() {
                    let mut usages = "".to_string();

                    if attachment.usages_COPY_DST {
                        usages += &"TextureUsages::COPY_DST | ";
                    }
                    if attachment.usages_COPY_SRC {
                        usages += &"TextureUsages::COPY_SRC | ";
                    }
                    if attachment.usages_RENDER_ATTACHMENT {
                        usages += &"TextureUsages::RENDER_ATTACHMENT | ";
                    }
                    if attachment.usages_STORAGE_ATOMIC {
                        usages += &"TextureUsages::STORAGE_ATOMIC | ";
                    }
                    if attachment.usages_STORAGE_BINDING {
                        usages += &"TextureUsages::STORAGE_BINDING | ";
                    }
                    if attachment.usages_TEXTURE_BINDING {
                        usages += &"TextureUsages::TEXTURE_BINDING | ";
                    }

                    usages += "TextureUsages::empty()";

                    attachments_data += &format!(
                        "\n({}, {}, Some(\"{}_{}\")),",
                        texture_format_str_to_code(&attachment.format),
                        usages,
                        var_name,
                        i
                    );
                }

                let depth_data = if let Some(depth) = render_texture_json.depth_attachment {
                    let mut usages = "".to_string();

                    if render_texture_json.depth_usages_COPY_DST {
                        usages += &"TextureUsages::COPY_DST | ";
                    }
                    if render_texture_json.depth_usages_COPY_SRC {
                        usages += &"TextureUsages::COPY_SRC | ";
                    }
                    if render_texture_json.depth_usages_RENDER_ATTACHMENT {
                        usages += &"TextureUsages::RENDER_ATTACHMENT | ";
                    }
                    if render_texture_json.depth_usages_STORAGE_ATOMIC {
                        usages += &"TextureUsages::STORAGE_ATOMIC | ";
                    }
                    if render_texture_json.depth_usages_STORAGE_BINDING {
                        usages += &"TextureUsages::STORAGE_BINDING | ";
                    }
                    if render_texture_json.depth_usages_TEXTURE_BINDING {
                        usages += &"TextureUsages::TEXTURE_BINDING | ";
                    }

                    usages += "TextureUsages::empty()";

                    format!(
                        "Some(({}, {}, Some(\"{}_depth\")))",
                        texture_format_str_to_code(&depth),
                        usages,
                        var_name
                    )
                } else {
                    "None".to_string()
                };

                fn_code += &format!(
                    "\nlet {} = RenderTexture::new(context, {}, {}, &[{}], {});",
                    var_name,
                    render_texture_json.width,
                    render_texture_json.height,
                    attachments_data,
                    depth_data
                );

                ret_code += &format!("\n{var_name},");
                ret_struct_code += &format!("\npub {var_name}: RenderTexture,");
            }
            _ => {}
        }
    }

    // for i in 0..analisys.0.len() {
    //     //let analiz = &analisys.0[i];
    //     //let path = &analisys.1[i];

    //     //let (file_path, json_path) = process_common_file_descriptor(path);

    //     //let json_content = fs::read_to_string(&json_path)?;
    //     //let wgsl_json: WgslJson = serde_json::from_str(&json_content)?;

    //     //let var_name = path_to_var(&path);

    //     //RenderMaterial::new(context, culling, label, render_texture, module, vert_layout, depth_compare, layout)

    //     // fn_code += &format!("let {} = RenderMaterial::new(context, {}, Some({}), {}, {}, {}, {}, {});",
    //     //     var_name,
    //     // );
    // }

    let mut buttom = fs::read_to_string("smq_proj/config/build/inject/buttom.txt")?;
    let outside = fs::read_to_string("smq_proj/config/build/inject/outside.txt")?;
    let mut ret_struct = fs::read_to_string("smq_proj/config/build/inject/ret_struct.txt")?;
    let mut ret = fs::read_to_string("smq_proj/config/build/inject/ret.txt")?;
    let top = fs::read_to_string("smq_proj/config/build/inject/top.txt")?;

    if state.settings.bake_settings.generate_blit_code {
        let attachment_name = path_to_var(&PathBuf::from(
            state
                .settings
                .bake_settings
                .render_texture_path
                .clone()
                .ok_or("using blit gen wydouth setting renderTexture to blit")?,
        ));
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

    let code = format!(
        "
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

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Textures {{
{png_enum}
}}


{struct_code}

{outside}

pub struct GenAssets {{
    {ret_struct_code}
    {ret_struct}
    pub meshes: [Mesh; {smf_files}],
    pub textures: [TextureS; {png_files}]
}}

impl GenAssets {{
    pub fn init_gen_assets<D>(context: &Context<D>, ssf_data: &[SSFAsset]) -> Self {{
        let meshes = [
            {smf_gen}
        ];
        let textures = [
            {png_gen}
        ];
        {top}
        {fn_code}
        {buttom}
        return Self {{
            {ret_code}
            {ret}
            meshes,
            textures
        }};
    }}
}}
"
    );

    fs::write("game/src/gen_bake.rs", code)?;

    Command::new("rustfmt")
        .arg("game/src/gen_bake.rs")
        .status()
        .unwrap();

    return Ok(files);
}
