use std::{fs, path::{Path, PathBuf}};

use crate::editor::assets::Assets;

use serde::{ Serialize, Deserialize };

mod rendertexture;

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
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

            match ext.as_str() {
                "rendertexture" => rendertexture::draw(&path, ui),
                _ => {
                    ui.heading("file");
                    ui.label(format!("file: {}", path.file_name().unwrap().to_string_lossy()));
                    ui.label(format!("extent: .{}", ext));
                }
            }
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("select file or object");
            });
        }
    }
}


pub fn process_common_file_descriptor(path: &Path) -> (PathBuf, PathBuf) {
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



