use std::path::PathBuf;

use crate::editor::assets::Assets;

mod rendertexture;
pub use rendertexture::*;

mod blend;
pub use blend::*;

mod png;
pub use png::*;

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
                "rendertexture" => rendertexture::draw(path, ui),
                "blend" => blend::draw(path, ui),
                "png" => png::draw(path, ui),
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