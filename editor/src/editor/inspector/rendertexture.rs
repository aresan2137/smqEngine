use std::{fs, path::{Path}};

use serde::{Deserialize, Serialize};

use crate::process_common_file_descriptor;

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

pub fn draw(path: &Path, ui: &mut egui::Ui) {
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
