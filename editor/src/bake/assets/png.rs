use std::{fs, path::PathBuf};

use crate::{
    bake::assets::converters::{
        adresat_str_to_code, path_to_var, sampler_str_to_code, texture_format_str_to_code,
    },
    editor::TextureJson,
};

use std::error::Error;

pub fn create_png_bake(
    iterer: &Vec<(PathBuf, String)>,
    files: &mut Vec<PathBuf>,
) -> Result<(i32, String, String), Box<dyn Error>> {
    let mut png_files = 0;
    let mut png_enum = "".to_string();
    let mut png_gen = "".to_string();

    for (path, ext) in iterer {
        match ext.as_str() {
            "png" => {
                let var_name = path_to_var(&path);

                let json_path = if let Some(file_name) = path.file_name() {
                    let mut new_name = file_name.to_os_string();
                    new_name.push(".meta");
                    path.with_file_name(new_name)
                } else {
                    path.clone()
                };

                let json_content = fs::read_to_string(&json_path)?;
                let texture_json: TextureJson = serde_json::from_str(&json_content)?;

                png_gen += &format!(
                    "\nTextureS::new(context, ssf_data[{}].as_ref(), {}, {}, {}, {}, Some(\"{}\")),",
                    files.len() as i32,
                    sampler_str_to_code(&texture_json.sampler),
                    texture_json.mipmaps,
                    texture_format_str_to_code(&texture_json.format),
                    adresat_str_to_code(&texture_json.repeater),
                    var_name
                );

                png_enum += &format!("{} = {},", var_name, png_files);

                png_files += 1;

                files.push(path.clone());
            }
            _ => {}
        }
    }

    return Ok((png_files, png_enum, png_gen));
}
