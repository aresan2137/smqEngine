use std::{path::{Path, PathBuf}, process::Command};

use crate::editor::assets::Assets;

use std::fs;

use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct CommonFileDescriptor {
    pub file_path: String,
    pub json_path: String
}

pub struct Inspector {
    locked: bool,
    path: Option<PathBuf>
}

impl Default for Inspector {
    fn default() -> Self {
        return Self {  
            locked: false,
            path: None
        };
    }
}

impl Inspector {
    pub fn egui(&mut self, ui: &mut egui::Ui, assets: &mut Assets) {

        if !self.locked {
            self.path = assets.selected.clone();
        }

        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let lock_text = if self.locked { "Locked" } else { "Unlocked" };
                ui.toggle_value(&mut self.locked, lock_text);
            });
        });
        ui.separator();

        if let Some(path) = &self.path.clone() {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

            if ext == "png" {
                png(path, ui);
            } else if ext == "exr" {
                png(path, ui);
            } else if ext == "jpg" {
                png(path, ui);
            } else if ext == "renderTexture" {
                render_texture(path, ui);
            } else if ext == "blend" {
                smf(path, ui);
            } else if ext == "ubo" {
                ubo(path, ui);
            } else if ext == "wgsl" {
                wgsl(path, ui);
            } else if ext == "bindGroup" {
                bind_group(path, ui);
            } else {
                ui.heading("file");
                ui.label(format!("file: {}", path.file_name().unwrap().to_string_lossy()));
                ui.label(format!("extent: .{}", ext));
            }
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("select file or object");
            });
        }
    }
}

pub fn process_common_file_descriptor(path: &PathBuf) -> (PathBuf, PathBuf) {
    let is_valid_json = match fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str::<CommonFileDescriptor>(&contents).is_ok(),
        Err(_) => false
    };

    let path_str = path.display().to_string().replace('\\', "/");
    
    let rest = if let Some(idx) = path_str.find("smq_proj/assets/") {
        &path_str[idx + "smq_proj/assets/".len()..]
    } else {
        panic!("error: filepath doesn't start with smq_proj/assets/': {:?}", path);
    };

    let new_file_path = PathBuf::from("smq_proj/data/file").join(rest);
    let new_json_path = PathBuf::from("smq_proj/data/json").join(rest);

    if !is_valid_json {
        if path.exists() {
            if let Some(parent) = new_file_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Err(_) = fs::rename(path, &new_file_path) {
                if fs::copy(path, &new_file_path).is_ok() {
                    let _ = fs::remove_file(path);
                }
            }
        }
    } else {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(descriptor) = serde_json::from_str::<CommonFileDescriptor>(&contents) {
                let new_file_path_str = new_file_path.display().to_string();
                if descriptor.file_path != new_file_path_str && Path::new(&descriptor.file_path).exists() {
                    if let Some(parent) = new_file_path.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    let _ = fs::rename(&descriptor.file_path, &new_file_path);
                }

                let new_json_path_str = new_json_path.display().to_string();
                if descriptor.json_path != new_json_path_str && Path::new(&descriptor.json_path).exists() {
                    if let Some(parent) = new_json_path.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    let _ = fs::rename(&descriptor.json_path, &new_json_path);
                }
            }
        }
    }

    let updated_descriptor = CommonFileDescriptor { 
        file_path: new_file_path.display().to_string(), 
        json_path: new_json_path.display().to_string()
    };    

    let json_string = serde_json::to_string_pretty(&updated_descriptor).unwrap();
    fs::write(path, json_string).expect("failed to save common file descriptor file");

    (new_file_path, new_json_path)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct TextureJson {
    pub sampler: String,
    pub repeater: String,
    pub format: String,
    pub mipmaps: u8
}

impl Default for TextureJson {
    fn default() -> Self {
        Self {
            sampler: "Nearest".to_string(),
            repeater: "Repeat".to_string(),
            format: "Rgba8UnormSrgb".to_string(),
            mipmaps: 1
        }
    }
}

fn png(path: &PathBuf, ui: &mut egui::Ui) {
    let (_, json_path) = process_common_file_descriptor(path);

    let mut texture_json: TextureJson = if let Ok(contents) = fs::read_to_string(&json_path) { if let Ok(file) = serde_json::from_str(&contents) { file } else { TextureJson::default() } } else { TextureJson::default() };

    ui.heading("texture settings");

    egui::ComboBox::from_label("sampler").selected_text(format!("{}", texture_json.sampler)).show_ui(ui, |ui| {
        ui.selectable_value(&mut texture_json.sampler, "Nearest".to_string(), "Nearest");
        ui.selectable_value(&mut texture_json.sampler, "Bilinear".to_string(), "Bilinear");
        ui.selectable_value(&mut texture_json.sampler, "Trilinear".to_string(), "Trilinear");
    });

    egui::ComboBox::from_label("repeat").selected_text(format!("{}", texture_json.repeater)).show_ui(ui, |ui| {
        ui.selectable_value(&mut texture_json.repeater, "Repeat".to_string(), "Repeat");
        ui.selectable_value(&mut texture_json.repeater, "Clamp".to_string(), "Clamp");
    });

    egui::ComboBox::from_label("format").selected_text(format!("{}", texture_json.format)).show_ui(ui, |ui| {
        ui.selectable_value(&mut texture_json.format, "Rgba8UnormSrgb".to_string(), "Rgba8UnormSrgb");
        ui.selectable_value(&mut texture_json.format, "Rgba8Unorm".to_string(), "Rgba8Unorm");
        ui.selectable_value(&mut texture_json.format, "Rgba16Unorm".to_string(), "Rgba16Unorm");
        ui.selectable_value(&mut texture_json.format, "Rgba16Float".to_string(), "Rgba16Float");
        ui.selectable_value(&mut texture_json.format, "Rgba32Float".to_string(), "Rgba32Float");
    });

    ui.add(egui::Slider::new(&mut texture_json.mipmaps, 1..=4).text("mipmap levels"));

    if let Some(parent) = json_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    
    let json_string = serde_json::to_string(&texture_json).unwrap();
    fs::write(json_path, json_string).expect("failed to save png json");
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RenderTextureAttachmentJson {
    pub format: String,
    pub usages_COPY_DST: bool,
    pub usages_COPY_SRC: bool,
    pub usages_RENDER_ATTACHMENT: bool,
    pub usages_STORAGE_ATOMIC: bool,
    pub usages_STORAGE_BINDING: bool,
    pub usages_TEXTURE_BINDING: bool
}

impl Default for RenderTextureAttachmentJson {
    fn default() -> Self {
        Self {
            format: "Rgba8UnormSrgb".to_string(),
            usages_COPY_DST: false,
            usages_COPY_SRC: false,
            usages_RENDER_ATTACHMENT: false,
            usages_STORAGE_ATOMIC: false,
            usages_STORAGE_BINDING: false,
            usages_TEXTURE_BINDING: false
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
#[allow(non_snake_case)]
pub struct RenderTextureJson {
    pub width: u32,
    pub height: u32,
    pub attachments: Vec<RenderTextureAttachmentJson>,
    pub depth_attachment: Option<String>,
    pub depth_usages_COPY_DST: bool,
    pub depth_usages_COPY_SRC: bool,
    pub depth_usages_RENDER_ATTACHMENT: bool,
    pub depth_usages_STORAGE_ATOMIC: bool,
    pub depth_usages_STORAGE_BINDING: bool,
    pub depth_usages_TEXTURE_BINDING: bool
}

impl Default for RenderTextureJson {
    fn default() -> Self {
        Self {
            depth_attachment: Some("Depth32Float".to_string()),
            height: 720,
            width: 1280,
            attachments: vec![RenderTextureAttachmentJson::default()],
            depth_usages_COPY_DST: false,
            depth_usages_COPY_SRC: false,
            depth_usages_RENDER_ATTACHMENT: false,
            depth_usages_STORAGE_ATOMIC: false,
            depth_usages_STORAGE_BINDING: false,
            depth_usages_TEXTURE_BINDING: false
        }
    }
}

fn render_texture(path: &PathBuf, ui: &mut egui::Ui) {
    let (_, json_path) = process_common_file_descriptor(path);

    let mut texture_json: RenderTextureJson = if let Ok(contents) = fs::read_to_string(&json_path) { if let Ok(file) = serde_json::from_str(&contents) { file } else { RenderTextureJson::default() } } else { RenderTextureJson::default() };

    let w_id = ui.id().with("w_input");
    let h_id = ui.id().with("h_input");
    let s_id = ui.id().with("s_input");
    
    let width_before = texture_json.width;
    let height_before = texture_json.height;

    let mut width_input = ui.data_mut(|d| d.get_temp::<String>(w_id).unwrap_or_else(|| texture_json.width.to_string()));
    let mut height_input = ui.data_mut(|d| d.get_temp::<String>(h_id).unwrap_or_else(|| texture_json.height.to_string()));
    let mut select_id = ui.data_mut(|d| d.get_temp::<usize>(s_id).unwrap_or_else(|| 0));

    ui.heading("texture settings");

    ui.horizontal(|ui| {
        ui.label("Presets:");
        if ui.button("720p").clicked() {
            texture_json.width = 1280; texture_json.height = 720;
        }
        if ui.button("1080p").clicked() {
            texture_json.width = 1920; texture_json.height = 1080;
        }
        if ui.button("4K").clicked() {
            texture_json.width = 3840; texture_json.height = 2160;
        }
        if ui.button("8K").clicked() {
            texture_json.width = 7680; texture_json.height = 4320;
        }
        if ui.button("x24").clicked() {
            texture_json.width = 384; texture_json.height = 216;
        }
    });

    if texture_json.width != width_before || texture_json.height != height_before {
        width_input = texture_json.width.to_string();
        height_input = texture_json.height.to_string();
        
        if let Some(parent) = json_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json_string = serde_json::to_string(&texture_json).unwrap();
        fs::write(&json_path, json_string).expect("failed to save render texture json");
    }

    ui.horizontal(|ui| {
        ui.label("Width:");
        let w_response = ui.add(egui::TextEdit::singleline(&mut width_input).desired_width(100.0));
        
        if w_response.lost_focus() {
            if let Ok(result) = meval::eval_str(&width_input) {
                texture_json.width = result.round() as u32;
                width_input = texture_json.width.to_string();
            } else {
                width_input = texture_json.width.to_string();
            }
        }
    });

    ui.horizontal(|ui| {
        ui.label("Height:");
        let h_response = ui.add(egui::TextEdit::singleline(&mut height_input).desired_width(100.0));
        
        if h_response.lost_focus() {
            if let Ok(result) = meval::eval_str(&height_input) {
                texture_json.height = result.round() as u32;
                height_input = texture_json.height.to_string();
            } else {
                height_input = texture_json.height.to_string();
            }
        }
    });  

    ui.separator();

    let mut text = "None";
    
    if let Some(ref attachment) = texture_json.depth_attachment { 
        text = attachment
    } 

    egui::ComboBox::from_label("depth format").selected_text(format!("{}", text)).show_ui(ui, |ui| {
        let uncompressed_formats = [
            "Depth16Unorm", "Depth24Plus", "Depth24PlusStencil8", 
            "Depth32Float", "Depth32FloatStencil8"
        ];

        for format in uncompressed_formats {
            ui.selectable_value(&mut texture_json.depth_attachment, Some(format.to_string()), format);
        }

        ui.selectable_value(&mut texture_json.depth_attachment, None, "None");
    });  

    ui.checkbox(&mut texture_json.depth_usages_COPY_DST, "depth_COPY_DST");
    ui.checkbox(&mut texture_json.depth_usages_COPY_SRC, "depth_COPY_SRC");
    ui.checkbox(&mut texture_json.depth_usages_RENDER_ATTACHMENT, "depth_RENDER_ATTACHMENT");
    ui.checkbox(&mut texture_json.depth_usages_STORAGE_ATOMIC, "depth_STORAGE_ATOMIC");
    ui.checkbox(&mut texture_json.depth_usages_STORAGE_BINDING, "depth_STORAGE_BINDING");
    ui.checkbox(&mut texture_json.depth_usages_TEXTURE_BINDING, "depth_TEXTURE_BINDING");

    ui.separator();

    ui.horizontal(|ui| {
        if ui.button("add").clicked() {
            texture_json.attachments.push(RenderTextureAttachmentJson::default());
        }
        if ui.button("remove last").clicked() {
            texture_json.attachments.pop();
        }
    });

    egui::ScrollArea::vertical().show(ui, |ui| {
        for (i, attachment) in texture_json.attachments.iter().enumerate() {
            ui.selectable_value(&mut select_id, i, format!("attachment {}, format: {}", i, attachment.format));
        }
    });

    if select_id >= texture_json.attachments.len() {
        select_id = 0;
    }

    ui.separator();

    egui::ComboBox::from_label("format").selected_text(format!("{}", texture_json.attachments[select_id].format)).show_ui(ui, |ui| {
        let uncompressed_formats = [
            "Rgba8Unorm", "Rgba8UnormSrgb", "Rgba8Snorm", "Rgba8Uint", "Rgba8Sint",
            "Rg8Unorm", "Rg8Snorm", "Rg8Uint", "Rg8Sint",
            "R8Unorm", "R8Snorm", "R8Uint", "R8Sint",

            "Rgba16Float", "Rgba16Unorm", "Rgba16Snorm", "Rgba16Uint", "Rgba16Sint",
            "Rg16Float", "Rg16Unorm", "Rg16Snorm", "Rg16Uint", "Rg16Sint",
            "R16Float", "R16Unorm", "R16Snorm", "R16Uint", "R16Sint",

            "Rgba32Float", "Rgba32Uint", "Rgba32Sint",
            "Rg32Float", "Rg32Uint", "Rg32Sint",
            "R32Float", "R32Uint", "R32Sint"
        ];

        for format in uncompressed_formats {
            ui.selectable_value(&mut texture_json.attachments[select_id].format, format.to_string(), format);
        }
    });

    ui.checkbox(&mut texture_json.attachments[select_id].usages_COPY_DST, "COPY_DST");
    ui.checkbox(&mut texture_json.attachments[select_id].usages_COPY_SRC, "COPY_SRC");
    ui.checkbox(&mut texture_json.attachments[select_id].usages_RENDER_ATTACHMENT, "RENDER_ATTACHMENT");
    ui.checkbox(&mut texture_json.attachments[select_id].usages_STORAGE_ATOMIC, "STORAGE_ATOMIC");
    ui.checkbox(&mut texture_json.attachments[select_id].usages_STORAGE_BINDING, "STORAGE_BINDING");
    ui.checkbox(&mut texture_json.attachments[select_id].usages_TEXTURE_BINDING, "TEXTURE_BINDING");

    ui.data_mut(|d| {
        d.insert_temp(w_id, width_input);
        d.insert_temp(h_id, height_input);
        d.insert_temp(s_id, select_id);
    });

    if let Some(parent) = json_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let json_string = serde_json::to_string(&texture_json).unwrap();
    fs::write(json_path, json_string).expect("failed to save render texture json");
}

fn smf(path: &PathBuf, ui: &mut egui::Ui) {
    let (file_path, _) = process_common_file_descriptor(path);
    ui.heading("Blender Model");
    ui.separator();

    let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown");
    ui.label(format!("file: {}", file_name));

    ui.separator();

    if ui.button("Bake").clicked() {
        let relative_path = file_path.strip_prefix("smq_proj/assets/").unwrap_or(&file_path);
        let tmp_gltf_path = std::path::PathBuf::from("smq_proj/tmp_assets/")
            .join(relative_path)
            .with_extension("glb");

        if let Some(parent) = tmp_gltf_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if call_blender_export(&file_path, &tmp_gltf_path) {
            let final_output_dir = file_path.with_extension(""); 
            compile_gltf_to_smf(&tmp_gltf_path, &final_output_dir);
        } else {
            eprintln!("failed to bake");
        }
    }
}

fn call_blender_export(blend_path: &Path, output_gltf_path: &Path) -> bool {
    let python_cmd = format!(
        "import bpy; bpy.ops.export_scene.gltf(filepath='{}', export_animations=False, export_image_format='NONE', export_materials='NONE')",
        output_gltf_path.display().to_string().replace("\\", "/")
    );

    let status = Command::new("blender").arg("-b").arg(blend_path).arg("--python-expr").arg(python_cmd).status();

    match status {
        Ok(s) => s.success(),
        Err(_) => {
            eprintln!("blender not found");
            false
        }
    }
}

fn compile_gltf_to_smf(gltf_path: &Path, output_dir: &Path) {
    let (document, buffers, _) = gltf::import(gltf_path).expect("failed to load glb file");

    let _ = std::fs::create_dir_all(output_dir);

    for mesh in document.meshes() {
        let raw_name = mesh.name().unwrap_or("unnamed_mesh");
        let safe_name = raw_name.replace(|c: char| !c.is_alphanumeric(), "_");

        let mut vertices: Vec<f32> = Vec::new();
        let mut vertex_count: i32 = 0;

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| buffers.get(buffer.index()).map(|b| &b.0[..]));

            let positions = reader.read_positions().map(|p| p.collect::<Vec<_>>());
            let normals = reader.read_normals().map(|n| n.collect::<Vec<_>>());
            let tex_coords = reader.read_tex_coords(0).map(|t| t.into_f32().collect::<Vec<_>>());

            if let Some(pos_vec) = positions {
                let to_y_up = |v: [f32; 3]| -> [f32; 3] {
                    [v[0], v[1], v[2]]
                };

                if let Some(indices_iter) = reader.read_indices() {
                    let indices: Vec<u32> = indices_iter.into_u32().collect();
                    for idx in indices {
                        let i = idx as usize;

                        let transformed_pos = to_y_up(pos_vec[i]);
                        vertices.extend_from_slice(&transformed_pos);

                        if let Some(ref uv_vec) = tex_coords {
                            vertices.extend_from_slice(&uv_vec[i]);
                        } else {
                            vertices.extend_from_slice(&[0.0, 0.0]);
                        }

                        if let Some(ref norm_vec) = normals {
                            let transformed_norm = to_y_up(norm_vec[i]);
                            vertices.extend_from_slice(&transformed_norm);
                        } else {
                            vertices.extend_from_slice(&[0.0, 1.0, 0.0]);
                        }

                        vertex_count += 1;
                    }
                } else {
                    for i in 0..pos_vec.len() {
                        let transformed_pos = to_y_up(pos_vec[i]);
                        vertices.extend_from_slice(&transformed_pos);

                        if let Some(ref uv_vec) = tex_coords {
                            vertices.extend_from_slice(&uv_vec[i]);
                        } else {
                            vertices.extend_from_slice(&[0.0, 0.0]);
                        }

                        if let Some(ref norm_vec) = normals {
                            let transformed_norm = to_y_up(norm_vec[i]);
                            vertices.extend_from_slice(&transformed_norm);
                        } else {
                            vertices.extend_from_slice(&[0.0, 1.0, 0.0]);
                        }

                        vertex_count += 1;
                    }
                }
            }
        }

        if vertex_count == 0 {
            continue; 
        }

        let mut payload = Vec::new();
        payload.push(0b10110000);
        payload.extend_from_slice(&vertex_count.to_le_bytes()); 
        payload.extend_from_slice(&0u32.to_le_bytes()); 

        for float_val in vertices {
            payload.extend_from_slice(&float_val.to_le_bytes());
        }

        let smf_path = output_dir.join(format!("{}.smf", safe_name));
        if let Err(e) = std::fs::write(&smf_path, payload) {
            eprintln!("failed to save smf for {}: {}", safe_name, e);
        } else {
            println!("Exported: {}", smf_path.display());
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct UboVarJson {
    pub type_: String,
    pub name: String
}

impl Default for UboVarJson {
    fn default() -> Self {
        Self {
            type_: "Mat4".to_string(),
            name: "var1".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct UboJson {
    pub stuff: Vec<UboVarJson>,
    pub is_dynamic: bool
}

impl Default for UboJson {
    fn default() -> Self {
        Self {
            stuff: vec![
                UboVarJson::default()
            ],
            is_dynamic: false
        }
    }
}

fn ubo(path: &PathBuf, ui: &mut egui::Ui) {
    let (_, json_path) = process_common_file_descriptor(path);

    let mut ubo_json: UboJson = if let Ok(contents) = fs::read_to_string(&json_path) { if let Ok(file) = serde_json::from_str(&contents) { file } else { UboJson::default() } } else { UboJson::default() };

    ui.heading("Ubo Settings");

    ui.horizontal(|ui| {
        if ui.button("add").clicked() {
            ubo_json.stuff.push(UboVarJson::default());
        }
        if ui.button("remove last").clicked() {
            ubo_json.stuff.pop();
        }
    });

    egui::ScrollArea::vertical().show(ui, |ui| {
        for (i, var) in ubo_json.stuff.iter_mut().enumerate() {
            ui.push_id(i, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("var{}: ", i));
                    let _ = ui.add(egui::TextEdit::singleline(&mut var.name).desired_width(100.0));
                    ui.separator();
                    egui::ComboBox::from_id_salt(0).selected_text(format!("{}", var.type_)).show_ui(ui, |ui| {
                        let uncompressed_formats = [
                            "Mat4", "Mat3", "Mat2", 
                            "i64", "i32", "i16", "i8",
                            "u64", "u32", "u16", "u8",
                            "f64", "f32",

                            "Vec4", "Vec3", "Vec2",
                            "IVec4", "IVec3", "IVec2",
                            "UVec4", "UVec3", "UVec2",

                        ];

                        for format in uncompressed_formats {
                            ui.selectable_value(&mut var.type_, format.to_string(), format);
                        }
                    });            
                });
            });
        }
    });

    ui.checkbox(&mut ubo_json.is_dynamic, "is dynamic");
    
    if let Some(parent) = json_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let json_string = serde_json::to_string(&ubo_json).unwrap();
    fs::write(json_path, json_string).expect("failed to save ubo json");
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct WgslJson {
    pub bindgroups: [Option<BindGroupGroupJson>; 4],
    pub culling: String,
    pub is_full: bool,
    pub write_depth: bool,
    pub depth_compare: String,
    pub blend_mode: String,
    pub topology: String,
    pub is_compute: bool
}

impl Default for WgslJson {
    fn default() -> Self {
        Self {
            bindgroups: [None, None, None, None],
            culling: "None".to_string(),
            is_full: true,
            write_depth: true,
            depth_compare: "Less".to_string(),
            blend_mode: "Opaque".to_string(),
            topology: "TriangleList".to_string(),
            is_compute: false
        }
    }
}

fn wgsl(path: &PathBuf, ui: &mut egui::Ui) {
    let (_, json_path) = process_common_file_descriptor(path);

    let mut wgsl_json: WgslJson = if let Ok(contents) = fs::read_to_string(&json_path) { if let Ok(file) = serde_json::from_str(&contents) { file } else { WgslJson::default() } } else { WgslJson::default() };

    for i in 0..4 {
        ui.label(format!("bind group {}:", i));

        if ui.button("clear").clicked() {
            wgsl_json.bindgroups[i] = None;
        }

        let (drop_rect, drop_response) = ui.allocate_exact_size(
            egui::vec2(200.0, 30.0), 
            egui::Sense::hover()
        );

        let bg_color = if drop_response.dnd_hover_payload::<std::path::PathBuf>().is_some() {
            egui::Color32::from_rgb(100, 150, 200)
        } else {
            egui::Color32::from_rgb(50, 50, 50)
        };
        
        ui.painter().rect_filled(drop_rect, 4.0, bg_color);
        ui.painter().rect_stroke(drop_rect, 4.0, egui::Stroke::new(1.0, egui::Color32::GRAY), egui::StrokeKind::Inside);

        ui.put(drop_rect, egui::Label::new( if let Some(vare) = &wgsl_json.bindgroups[i] {format!("file:{}", vare.potential_path)} else {"drop file here".to_string()}).selectable(false));

        if let Some(dropped_path) = drop_response.dnd_release_payload::<std::path::PathBuf>() {
            wgsl_json.bindgroups[i] = Some(BindGroupGroupJson {
                potential_path: dropped_path.as_ref().display().to_string(), 
                render_texture_id: 0,
                standard_getter_id: 0,
                visibility_compute: false,
                visibility_vertex: false,
                visibility_fragment: false
            });            
        }
    }

    ui.separator();

    ui.push_id(0, |ui| {
        ui.horizontal(|ui| {
            ui.label("culling:");
            egui::ComboBox::from_id_salt(0).selected_text(format!("{}", wgsl_json.culling)).show_ui(ui, |ui| {
                let uncompressed_formats = [
                    "None", "Back", "Front"
                ];

                for format in uncompressed_formats {
                    ui.selectable_value(&mut wgsl_json.culling, format.to_string(), format);
                }
            });            
        });
    });    

    ui.checkbox(&mut wgsl_json.is_full, "full");

    ui.checkbox(&mut wgsl_json.write_depth, "write depth");

    ui.push_id(1, |ui| {
        ui.horizontal(|ui| {
            ui.label("compare:");
            egui::ComboBox::from_id_salt(0).selected_text(format!("{}", wgsl_json.depth_compare)).show_ui(ui, |ui| {
                let uncompressed_formats = [
                    "Less", "Greater", "Equal", "NotEqual",
                    "LessOrEqual", "GreaterOrEqual", 
                    "Always", "Never"
                ];

                for format in uncompressed_formats {
                    ui.selectable_value(&mut wgsl_json.depth_compare, format.to_string(), format);
                }
            });            
        });
    });

    ui.push_id(2, |ui| {
        ui.horizontal(|ui| {
            ui.label("blend mode:");
            egui::ComboBox::from_id_salt(0).selected_text(format!("{}", wgsl_json.blend_mode)).show_ui(ui, |ui| {
                let uncompressed_formats = [
                    "Opaque", "Alpha Blend", "Additive"
                ];

                for format in uncompressed_formats {
                    ui.selectable_value(&mut wgsl_json.blend_mode, format.to_string(), format);
                }
            });            
        });    
    });

    ui.checkbox(&mut wgsl_json.is_compute, "is compute");

    if let Some(parent) = json_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let json_string = serde_json::to_string(&wgsl_json).unwrap();
    fs::write(json_path, json_string).expect("failed to save mat json");    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BindGroupGroupJson {
    pub potential_path: String,
    pub render_texture_id: i32, // -1 = depth 0..inf = attachments
    pub standard_getter_id: u8,
    pub visibility_compute: bool,
    pub visibility_vertex: bool,
    pub visibility_fragment: bool
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct BindGroupJson {
    pub groups: Vec<Option<BindGroupGroupJson>>
}

impl Default for BindGroupJson {
    fn default() -> Self {
        Self {
            groups: vec![None]
        }
    }
}

fn bind_group(path: &PathBuf, ui: &mut egui::Ui) {
    let (_, json_path) = process_common_file_descriptor(path);

    let mut bind_group_json: BindGroupJson = if let Ok(contents) = fs::read_to_string(&json_path) { if let Ok(file) = serde_json::from_str(&contents) { file } else { BindGroupJson::default() } } else { BindGroupJson::default() };

    ui.heading("Bind Group Settings");

    ui.horizontal(|ui| {
        if ui.button("add").clicked() {
            bind_group_json.groups.push(None);
        }
        if ui.button("remove last").clicked() {
            bind_group_json.groups.pop();
        }
    });

    egui::ScrollArea::vertical().show(ui, |ui| {
        for (i, var) in bind_group_json.groups.iter_mut().enumerate() {
            ui.push_id(i, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("binding{}:", i));

                    let (drop_rect, drop_response) = ui.allocate_exact_size(
                        egui::vec2(200.0, 30.0), 
                        egui::Sense::hover()
                    );

                    let bg_color = if drop_response.dnd_hover_payload::<std::path::PathBuf>().is_some() {
                        egui::Color32::from_rgb(100, 150, 200)
                    } else {
                        egui::Color32::from_rgb(50, 50, 50)
                    };
                    
                    ui.painter().rect_filled(drop_rect, 4.0, bg_color);
                    ui.painter().rect_stroke(drop_rect, 4.0, egui::Stroke::new(1.0, egui::Color32::GRAY), egui::StrokeKind::Inside);

                    ui.put(drop_rect, egui::Label::new( if let Some(vare) = var {format!("file:{}", vare.potential_path)} else {"drop file here".to_string()}).selectable(false));

                    if let Some(dropped_path) = drop_response.dnd_release_payload::<std::path::PathBuf>() {
                        *var = Some(BindGroupGroupJson { 
                            potential_path: dropped_path.as_ref().display().to_string(), 
                            render_texture_id: 0,
                            standard_getter_id: 0,
                            visibility_compute: false,
                            visibility_fragment: false,
                            visibility_vertex: false
                        });
                    }
                });

                if let Some(vare) = var {
                    if vare.potential_path.ends_with(".renderTexture") {
                        ui.horizontal(|ui| {
                            ui.label("render texture attachment:");
                            
                            ui.add(egui::DragValue::new(&mut vare.render_texture_id).speed(0.01));
                        }); 
                    }
        
                    ui.horizontal(|ui| {
                        ui.label("visibility:");
                        
                        ui.checkbox(&mut vare.visibility_compute, "compute");
                        ui.checkbox(&mut vare.visibility_vertex, "vertex");
                        ui.checkbox(&mut vare.visibility_fragment, "fragment");
                    }); 
                } 
            });
        }
    });

    let json_string = serde_json::to_string(&bind_group_json).unwrap();
    fs::write(json_path, json_string).expect("failed to save bind group json");
}

