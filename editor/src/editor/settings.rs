use std::fs;

use serde::{Deserialize, Serialize};

use crate::drop_point;

#[derive(Serialize, Deserialize)]
pub struct BakeSettings {
    pub generate_blit_code: bool,
    pub render_texture_path: Option<String>,
    pub render_texture_attachment: String

}

impl Default for BakeSettings {
    fn default() -> Self {
        return Self {  
            generate_blit_code: true,
            render_texture_path: None,
            render_texture_attachment: "0".to_string()
        };
    }
}


#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub selected_setting: String,

    pub bake_settings: BakeSettings

}

impl Default for Settings {
    fn default() -> Self {
        Self { 
            selected_setting: "".to_string(),
            bake_settings: BakeSettings::default()
        }
    }
}

impl Settings {
    pub fn save(&self) {
        let json_string = serde_json::to_string(self).unwrap();
        fs::write("smq_proj/config/settings.json", json_string).expect("failed to save render texture json");
    }

    pub fn load() -> Self {
        return if let Ok(contents) = fs::read_to_string("smq_proj/config/settings.json") { if let Ok(file) = serde_json::from_str(&contents) { file } else { Self::default() } } else { Self::default() };
    }

    pub fn egui(&mut self, ui: &mut egui::Ui) {
        let height = ui.available_height();
                    
        ui.horizontal_top(|ui| {
            ui.push_id(0, |ui| {
                ui.set_width(100.0);
                ui.set_height(height);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.selectable_value(&mut self.selected_setting, "bake_settings".to_string(), "bake settings");

                        egui::CollapsingHeader::new("test2")
                            .show(ui, |ui| {
                                ui.selectable_value(&mut self.selected_setting, "t2sub1".to_string(), "t2sub1");
                                ui.selectable_value(&mut self.selected_setting, "t2sub2".to_string(), "t2sub2");
                            });
                    });
                });
            });                    

            ui.separator();

            ui.push_id(1, |ui| {
                ui.set_height(height);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        match self.selected_setting.as_str() {
                            "bake_settings" => {
                                ui.heading("bake settings");

                                ui.checkbox(&mut self.bake_settings.generate_blit_code, "generate blit code");

                                drop_point(ui, self.bake_settings.render_texture_path.clone(), |ui| {

                                }, |ui, path| {
                                    if let Some(paka) = path {
                                        self.bake_settings.render_texture_path = Some(paka.display().to_string());
                                    }                                    
                                });

                                ui.horizontal(|ui| {
                                    ui.label("attachment:");
                                    let w_response = ui.add(egui::TextEdit::singleline(&mut self.bake_settings.render_texture_attachment).desired_width(100.0));
                                    
                                    if w_response.lost_focus() {
                                        if let Ok(result) = meval::eval_str(&self.bake_settings.render_texture_attachment) {
                                            self.bake_settings.render_texture_attachment = (result.round() as i32).to_string();
                                        }
                                    }
                                });

                            }
                            "t2sub1" => {
                                ui.heading("t2sub1");
                            }
                            "t2sub2" => {
                                ui.heading("t2sub2");
                            }
                            _ => {
                                ui.heading("select a setting");
                            }
                        }
                    });
                });
            });                   
        });
    }
}
