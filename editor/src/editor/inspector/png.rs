use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::process_common_file_descriptor;


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

pub fn draw(path: &PathBuf, ui: &mut egui::Ui) {
    let (_, json_path) = process_common_file_descriptor(path);

    let mut texture_json: TextureJson = if let Ok(contents) = fs::read_to_string(&json_path) { if let Ok(file) = serde_json::from_str(&contents) { file } else { TextureJson::default() } } else { TextureJson::default() };

    ui.heading("texture settings");

    egui::ComboBox::from_label("sampler").selected_text(format!("{}", texture_json.sampler)).show_ui(ui, |ui| {
        ui.selectable_value(&mut texture_json.sampler, "Nearest".to_string(), "Nearest");
        ui.selectable_value(&mut texture_json.sampler, "Bilinear".to_string(), "Bilinear");
        ui.selectable_value(&mut texture_json.sampler, "Trilinear".to_string(), "Trilinear");
        ui.selectable_value(&mut texture_json.sampler, "NearestMapped".to_string(), "NearestMapped");
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

    ui.add(egui::Slider::new(&mut texture_json.mipmaps, 1..=10).text("mipmap levels"));

    if let Some(parent) = json_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    
    let json_string = serde_json::to_string(&texture_json).unwrap();
    fs::write(json_path, json_string).expect("failed to save png json");
}