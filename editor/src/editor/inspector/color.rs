use std::{fs, path::PathBuf};

use egui::{Rgba, color_picker::{Alpha, color_edit_button_rgba}};
use glam::Vec4;
use image::Rgb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct ColorJson {
    pub color: Rgba
}

impl Default for ColorJson {
    fn default() -> Self {
        Self {
            color: Rgba::WHITE
        }
    }
}

pub fn draw(path: &PathBuf, ui: &mut egui::Ui) {
    let json_path = if let Some(file_name) = path.file_name() {
        let mut new_name = file_name.to_os_string();
        new_name.push(".meta");
        path.with_file_name(new_name)
    } else {
        path.clone()
                };

    let mut color_json: ColorJson = if let Ok(contents) = fs::read_to_string(&json_path) {
        if let Ok(file) = serde_json::from_str(&contents) {
            file
        } else {
            ColorJson::default()
        }
    } else {
        ColorJson::default()
    };

    ui.heading("color settings");

    color_edit_button_rgba(ui, &mut color_json.color, Alpha::OnlyBlend);

    if let Some(parent) = json_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let json_string = serde_json::to_string(&color_json).unwrap();
    fs::write(json_path, json_string).expect("failed to save color json");
}
