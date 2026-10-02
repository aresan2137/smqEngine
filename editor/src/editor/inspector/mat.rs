use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct MatJson {
}

impl Default for MatJson {
    fn default() -> Self {
        Self {
        }
    }
}

pub fn draw(path: &PathBuf, ui: &mut egui::Ui) {
    let mut texture_json: MatJson = if let Ok(contents) = fs::read_to_string(&path) {
        if let Ok(file) = serde_json::from_str(&contents) {
            file
        } else {
            MatJson::default()
        }
    } else {
        MatJson::default()
    };

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let json_string = serde_json::to_string(&texture_json).unwrap();
    fs::write(path, json_string).expect("failed to save png json");
}
