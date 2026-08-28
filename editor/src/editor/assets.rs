use std::{collections::{HashMap, HashSet}, path::PathBuf};

use egui::{UiBuilder};

use crate::editor::inspector::CommonFileDescriptor;

pub struct Assets {
    pub path: PathBuf,
    pub selected: Option<std::path::PathBuf>,
    renaming: Option<std::path::PathBuf>,
    rename_buffer: String,
    icons: HashMap<String, Option<egui::TextureHandle>>
}

impl Default for Assets {
    fn default() -> Self {
        return Self {  
            path: PathBuf::from("smq_proj/assets"),
            selected: None,
            renaming: None,
            rename_buffer: String::new(),
            icons: HashMap::new()
        };
    }
}

impl Assets {
    pub fn egui(&mut self, ui: &mut egui::Ui) {
        const ASSET_SIZE: f32 = 75.0;
        const CARD_HEIGHT: f32 = 120.0;
        
        let root_path = std::path::PathBuf::from("smq_proj/assets");

        ui.horizontal(|ui| {
            let clicked_back_ui = ui.button("back").clicked();
            let clicked_back_mouse = ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Extra1));

            if (clicked_back_ui || clicked_back_mouse) && self.path != root_path {
                self.path.pop();
            }

            ui.separator();

            let formatted_path = self.path.display().to_string().replace("\\", "/").replace("smq_proj/assets", "");

            let final_label = if formatted_path.is_empty() { "/".to_string() } else { formatted_path };

            ui.label(final_label);
        });

        ui.separator();

        let relative_path = self.path.strip_prefix("smq_proj/assets").unwrap_or(std::path::Path::new(""));
        let scripts_path = std::path::PathBuf::from("game/src").join(relative_path);

        let _ = std::fs::create_dir_all(&self.path);
        let _ = std::fs::create_dir_all(&scripts_path);

        let mut all_files = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&self.path) {
            all_files.extend(entries.filter_map(|e| e.ok()).map(|e| e.path()));
        }
        if let Ok(entries) = std::fs::read_dir(&scripts_path) {
            all_files.extend(entries.filter_map(|e| e.ok()).map(|e| e.path()));
        }

        let mut display_files = Vec::new();
        let mut seen_folders = HashSet::new();

        for mut path in all_files {
            let is_dir = path.is_dir();
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();

            if is_dir {
                if !seen_folders.insert(file_name.clone()) { continue; }
            }

            if !is_dir && path.extension().and_then(|s| s.to_str()) == Some("rs") && path.starts_with("smq_proj/assets") {
                let new_target = scripts_path.join(&file_name);
                
                if std::fs::rename(&path, &new_target).is_err() {
                    if std::fs::copy(&path, &new_target).is_ok() {
                        let _ = std::fs::remove_file(&path);
                    }
                }
                
                path = new_target; 
            }

            display_files.push(path);
        }

        display_files.sort_by_key(|a| (!a.is_dir(), a.file_name().unwrap().to_string_lossy().into_owned()));

        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            let bg_rect = ui.available_rect_before_wrap();
            let bg_response = ui.interact(bg_rect, ui.id().with("bg_menu"), egui::Sense::click());

            if bg_response.clicked() {
                self.selected = None;
            }

            bg_response.context_menu(|ui| {
                if ui.button("new folder").clicked() {
                    let _ = std::fs::create_dir_all(self.path.join("new_folder"));
                    self.rename_buffer = "new_folder".to_string();
                    self.renaming = Some(self.path.join("new_folder"));
                    ui.close_menu();
                }
                if ui.button("new file").clicked() {
                    let _ = std::fs::write(self.path.join("new_file"), "");
                    self.rename_buffer = "new_file".to_string();
                    self.renaming = Some(self.path.join("new_file"));
                    ui.close_menu();
                }
                ui.separator(); 
                if ui.button("import file").clicked() {
                    if let Some(picked_path) = rfd::FileDialog::new().pick_file() {
                        if let Some(file_name) = picked_path.file_name() {
                            let destination = self.path.join(file_name);
                            
                            if let Err(e) = std::fs::copy(&picked_path, &destination) {
                                eprintln!("failed to import file: {}", e);
                            }
                        }
                    }
                    ui.close_menu();
                }
            });

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);

                for path in display_files {
                    let is_dir = path.is_dir();
                    let file_name = path.file_name().unwrap().to_string_lossy().into_owned();

                    let ext = if is_dir {
                        "folder".to_string()
                    } else {
                        path.extension().and_then(|s| s.to_str()).unwrap_or("unknown").to_lowercase()
                    };

                    if !self.icons.contains_key(&ext) {
                        let icon_path = format!("assets/icons/{}.png", ext);
                        let handle = if let Ok(image_bytes) = std::fs::read(&icon_path) {
                            if let Ok(img) = image::load_from_memory(&image_bytes) {
                                let size = [img.width() as _, img.height() as _];
                                let image_buffer = img.to_rgba8();
                                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, image_buffer.as_raw());
                                Some(ui.ctx().load_texture(&ext, color_image, egui::TextureOptions::LINEAR))
                            } else { None }
                        } else { None };
                        self.icons.insert(ext.clone(), handle);
                    }

                    let (card_rect, card_response) = ui.allocate_exact_size(
                        egui::vec2(ASSET_SIZE, CARD_HEIGHT), 
                        egui::Sense::click_and_drag()
                    );

                    card_response.dnd_set_drag_payload(path.clone());
                    let is_selected = self.selected.as_ref() == Some(&path);

                    ui.allocate_new_ui(UiBuilder::default().max_rect(card_rect), |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(4.0);

                            if let Some(Some(texture_handle)) = self.icons.get(&ext) {
                                let mut img = egui::Image::new(texture_handle)
                                    .fit_to_exact_size(egui::vec2(50.0, 50.0));
                                
                                if is_selected {
                                    img = img.tint(egui::Color32::from_rgb(150, 180, 255));
                                }
                                ui.add(img);
                            } else {
                                let bg_color = if is_selected {
                                    egui::Color32::from_rgb(80, 120, 200) 
                                } else if is_dir {
                                    egui::Color32::from_rgb(200, 170, 50)  
                                } else {
                                    egui::Color32::from_rgb(45, 85, 145)   
                                };

                                let (fallback_rect, _) = ui.allocate_exact_size(egui::vec2(50.0, 50.0), egui::Sense::hover());
                                ui.painter().rect_filled(fallback_rect, 4.0, bg_color);
                                if is_selected {
                                    ui.painter().rect_stroke(fallback_rect, 4.0, egui::Stroke::new(2.0, egui::Color32::WHITE), egui::StrokeKind::Inside);
                                }
                            }

                            ui.add_space(4.0);

                            if self.renaming.as_ref() == Some(&path) {
                                let edit_response = ui.add(
                                    egui::TextEdit::singleline(&mut self.rename_buffer).desired_width(ASSET_SIZE)
                                );
                                
                                edit_response.request_focus();

                                if edit_response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    if !self.rename_buffer.is_empty() && self.rename_buffer != file_name {
                                        let new_path = path.parent().unwrap().join(&self.rename_buffer);
                                        if let Err(e) = std::fs::rename(&path, new_path) {
                                            println!("failed to change name: {}", e);
                                        }
                                    }
                                    self.renaming = None;
                                }
                            } else {
                                ui.add(egui::Label::new(&file_name).truncate());
                            }
                        });
                    });

                    card_response.context_menu(|ui| {
                        if ui.button("change name").clicked() {
                            self.renaming = Some(path.clone());
                            self.rename_buffer = file_name.clone();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("delete").clicked() {
                            if trash::delete(&path).is_err() {
                                if is_dir { let _ = std::fs::remove_dir_all(&path); } 
                                else { let _ = std::fs::remove_file(&path); }
                            }
                            self.selected = None;
                            ui.close_menu();
                        }
                    });

                    if card_response.double_clicked() {
                        if is_dir {
                            self.path.push(&file_name);
                            self.selected = None;
                            self.renaming = None;
                        } else {
                            let optioned_descriptor: Option<CommonFileDescriptor> = 
                                if let Ok(contents) = std::fs::read_to_string(&path) { 
                                    if let Ok(file) = serde_json::from_str(&contents) { 
                                        Some(file) 
                                    } else { 
                                        None 
                                    } 
                                } else { 
                                    None 
                                };

                            let path_to_open = if let Some(desc) = optioned_descriptor {
                                std::path::PathBuf::from(desc.file_path) 
                            } else {
                                path.clone()
                            };

                            #[cfg(target_os = "windows")]
                            let _ = std::process::Command::new("cmd").args(["/C", "start", "", path_to_open.to_str().unwrap_or("")]).spawn();
                        }
                    } else if card_response.clicked() {
                        self.selected = Some(path.clone());
                    }
                }
            });
        });    
    }
}