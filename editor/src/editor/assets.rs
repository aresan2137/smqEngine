// use std::{collections::{HashMap, HashSet}, path::PathBuf};

// use crate::CommonFileDescriptor;

// // fixied it a bit but still kida sucks

// pub struct Assets {
//     pub path: PathBuf,
//     pub selected: Option<std::path::PathBuf>,
//     renaming: Option<std::path::PathBuf>,
//     rename_buffer: String,
//     icons: HashMap<String, Option<egui::TextureHandle>>,
// }

// impl Default for Assets {
//     fn default() -> Self {
//         return Self {
//             path: PathBuf::from("smq_proj/assets"),
//             selected: None,
//             renaming: None,
//             rename_buffer: String::new(),
//             icons: HashMap::new(),
//         };
//     }
// }

// impl Assets {
//     const CARD_W: f32 = 84.0;
//     const CARD_H: f32 = 96.0;
//     const ICON: f32 = 48.0;
//     const GAP: f32 = 6.0;
//     const PAD: f32 = 8.0;

//     pub fn egui(&mut self, ui: &mut egui::Ui) {
//         self.draw_top_bar(ui);
        
//         ui.separator();

//         self.draw_center_files(ui);
//     }

//     fn draw_top_bar(&mut self, ui: &mut egui::Ui) {
//         ui.horizontal(|ui| {
//             let rel = self
//                 .path
//                 .display()
//                 .to_string()
//                 .replace('\\', "/")
//                 .replace("smq_proj/", "");

//             let path_splited = rel.split("/");

//             ui.vertical(|ui| {
//                 egui::Frame::default().inner_margin(egui::Margin::symmetric(8, 0)).show(ui, |ui| {
//                     ui.horizontal(|ui| {
//                         ui.style_mut().visuals.button_frame = false;

//                         for (i, pit) in path_splited.clone().enumerate() {
//                             if i != 0 {
//                                 ui.label("/");
//                             }                            

//                             if ui.button(pit).clicked() {
//                                 let mut oute = "smq_proj".to_string();
//                                 for (j, pit) in path_splited.clone().enumerate() {
//                                     if j > i {
//                                         break;
//                                     }

//                                     oute += "/";
                                    
//                                     oute += pit;
//                                 }

//                                 self.path = PathBuf::from(oute);
//                             }
//                         }
//                     });
//                 });
//             });
//         });
//     }

//     fn draw_center_files(&mut self, ui: &mut egui::Ui) {
//         let relative_path = self.path.strip_prefix("smq_proj/assets").unwrap_or(std::path::Path::new(""));
//         let scripts_path = std::path::PathBuf::from("game/src").join(relative_path);

//         let _ = std::fs::create_dir_all(&self.path);
//         let _ = std::fs::create_dir_all(&scripts_path);

//         let mut all_files: Vec<std::path::PathBuf> = Vec::new();
//         if let Ok(rd) = std::fs::read_dir(&self.path) {
//             all_files.extend(rd.filter_map(|e| e.ok()).map(|e| e.path()));
//         }
//         if let Ok(rd) = std::fs::read_dir(&scripts_path) {
//             all_files.extend(rd.filter_map(|e| e.ok()).map(|e| e.path()));
//         }

//         let mut display_files: Vec<std::path::PathBuf> = Vec::new();
//         let mut seen_folders = HashSet::new();

//         for mut path in all_files {
//             let is_dir = path.is_dir();
//             let name = path.file_name().unwrap().to_string_lossy().into_owned();

//             if is_dir && !seen_folders.insert(name.clone()) {
//                 continue;
//             }

//             if !is_dir && path.extension().and_then(|s| s.to_str()) == Some("rs") && path.starts_with("smq_proj/assets") {
//                 let target = scripts_path.join(&name);
//                 if std::fs::rename(&path, &target).is_err() {
//                     if std::fs::copy(&path, &target).is_ok() {
//                         let _ = std::fs::remove_file(&path);
//                     }
//                 }
//                 path = target;
//             }

//             display_files.push(path);
//         }

//         display_files.sort_by_key(|a| {
//             return (
//                 !a.is_dir(),
//                 a.file_name().unwrap().to_string_lossy().to_lowercase(),
//             );
//         });

//         egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
//             let bg_rect = ui.available_rect_before_wrap();
//             let bg = ui.interact(bg_rect, ui.id().with("asset_bg"), egui::Sense::click());
//             if bg.clicked() {
//                 self.selected = None;
//             }
//             bg.context_menu(|ui| {
//                 if ui.button("new folder").clicked() {
//                     let target = self.path.join("new_folder");
//                     let _ = std::fs::create_dir_all(&target);
//                     self.rename_buffer = "new_folder".to_string();
//                     self.renaming = Some(target);
//                     ui.close();
//                 }
//                 if ui.button("new file").clicked() {
//                     let target = self.path.join("new_file");
//                     let _ = std::fs::write(&target, "");
//                     self.rename_buffer = "new_file".to_string();
//                     self.renaming = Some(target);
//                     ui.close();
//                 }
//                 ui.separator();
//                 if ui.button("import file").clicked() {
//                     if let Some(picked) = rfd::FileDialog::new().pick_file() {
//                         if let Some(fname) = picked.file_name() {
//                             let dst = self.path.join(fname);
//                             if let Err(e) = std::fs::copy(&picked, &dst) {
//                                 eprintln!("import failed: {e}");
//                             }
//                         }
//                     }
//                     ui.close();
//                 }
//             });

//             let avail_w = ui.available_width();
//             let usable = (avail_w - Assets::PAD * 2.0).max(Assets::CARD_W);
//             let columns = ((usable + Assets::GAP) / (Assets::CARD_W + Assets::GAP)).floor().max(1.0) as usize;

//             ui.add_space(Assets::PAD);

//             for chunk in display_files.chunks(columns) {
//                 ui.horizontal(|ui| {
//                     ui.add_space(Assets::PAD);
//                     ui.spacing_mut().item_spacing.x = Assets::GAP;

//                     for path in chunk {
//                         self.draw_file(ui, path, Assets::CARD_W, Assets::CARD_H, Assets::ICON);
//                     }
//                 });
//                 ui.add_space(Assets::GAP);
//             }

//             ui.add_space(Assets::PAD);
//         });
//     }

//     fn draw_file(&mut self, ui: &mut egui::Ui, path: &std::path::Path, card_w: f32, card_h: f32, icon_size: f32) -> egui::Response {
//         let is_dir = path.is_dir();
//         let file_name = path.file_name().unwrap().to_string_lossy().into_owned();

//         let ext = if is_dir {
//             "folder".to_string()
//         } else {
//             path.extension()
//                 .and_then(|s| s.to_str())
//                 .unwrap_or("unknown")
//                 .to_lowercase()
//         };

//         if !self.icons.contains_key(&ext) {
//             let icon_path = format!("assets/icons/{}.png", ext);
//             let handle = std::fs::read(&icon_path)
//                 .ok()
//                 .and_then(|bytes| image::load_from_memory(&bytes).ok())
//                 .map(|img| {
//                     let size = [img.width() as _, img.height() as _];
//                     let rgba = img.to_rgba8();
//                     let color = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
//                     ui.ctx()
//                         .load_texture(&ext, color, egui::TextureOptions::LINEAR)
//                 });
//             self.icons.insert(ext.clone(), handle);
//         }

//         if egui::DragAndDrop::payload::<std::path::PathBuf>(ui.ctx()).as_deref() == Some(&path.to_path_buf()) {
//             if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
//                 egui::Area::new(egui::Id::new("file_drag_preview"))
//                     .fixed_pos(pointer_pos + egui::vec2(12.0, 12.0))
//                     .order(egui::Order::Tooltip)
//                     .interactable(false)
//                     .show(ui.ctx(), |ui| {
//                         egui::Frame::popup(ui.style()).show(ui, |ui| {
//                             ui.horizontal(|ui| {
//                                 let preview_size = egui::vec2(20.0, 20.0);
//                                 if let Some(Some(tex)) = self.icons.get(&ext) {
//                                     ui.add(egui::Image::new(tex).fit_to_exact_size(preview_size));
//                                 } else {
//                                     let fallback = if is_dir {
//                                         egui::Color32::from_rgb(200, 170, 50)
//                                     } else {
//                                         egui::Color32::from_rgb(45, 85, 145)
//                                     };
//                                     let (rect, _) = ui.allocate_exact_size(preview_size, egui::Sense::hover());
//                                     ui.painter().rect_filled(rect, 2.0, fallback);
//                                 }

//                                 ui.label(
//                                     egui::RichText::new(&file_name)
//                                         .size(12.0)
//                                         .color(ui.visuals().text_color()),
//                                 );
//                             });
//                         });
//                     });
//             }
//         }

//         let (card_rect, response) = ui.allocate_exact_size(egui::vec2(card_w, card_h), egui::Sense::click_and_drag());

//         let primary_down = ui.input(|i| i.pointer.primary_down());

//         if !primary_down {
//             let _ = egui::DragAndDrop::take_payload::<std::path::PathBuf>(ui.ctx());
//         }

//         if response.drag_started() {
//             egui::DragAndDrop::set_payload(ui.ctx(), path.to_path_buf());
//         }

//         let is_selected = self.selected.as_deref() == Some(path);
//         let hovered = response.hovered();

//         let bg = if is_selected {
//             ui.visuals().selection.bg_fill
//         } else if hovered {
//             ui.visuals().widgets.hovered.weak_bg_fill
//         } else {
//             egui::Color32::TRANSPARENT
//         };
//         if bg != egui::Color32::TRANSPARENT {
//             ui.painter().rect_filled(card_rect, 4.0, bg);
//         }

//         let icon_rect = egui::Rect::from_center_size(
//             egui::pos2(
//                 card_rect.center().x,
//                 card_rect.top() + 6.0 + icon_size / 2.0,
//             ),
//             egui::vec2(icon_size, icon_size),
//         );

//         if let Some(Some(tex)) = self.icons.get(&ext) {
//             let mut img = egui::Image::new(tex).fit_to_exact_size(icon_rect.size());
//             if is_selected {
//                 img = img.tint(egui::Color32::from_rgb(150, 180, 255));
//             }
//             egui::Image::paint_at(&img, ui, icon_rect);
//         } else {
//             let fallback = if is_dir {
//                 egui::Color32::from_rgb(200, 170, 50)
//             } else {
//                 egui::Color32::from_rgb(45, 85, 145)
//             };
//             ui.painter().rect_filled(icon_rect, 4.0, fallback);
//             if is_selected {
//                 ui.painter().rect_stroke(
//                     icon_rect,
//                     4.0,
//                     egui::Stroke::new(2.0, egui::Color32::WHITE),
//                     egui::StrokeKind::Inside,
//                 );
//             }
//         }

//         let label_rect = egui::Rect::from_min_max(
//             egui::pos2(card_rect.left() + 2.0, icon_rect.bottom() + 4.0),
//             egui::pos2(card_rect.right() - 2.0, card_rect.bottom() - 2.0),
//         );

//         if self.renaming.as_deref() == Some(path) {
//             let mut child = ui.new_child(
//                 egui::UiBuilder::new()
//                     .max_rect(label_rect)
//                     .layout(egui::Layout::top_down(egui::Align::Center)),
//             );
//             let edit = child.add(
//                 egui::TextEdit::singleline(&mut self.rename_buffer)
//                     .desired_width(label_rect.width()),
//             );
//             edit.request_focus();

//             if edit.lost_focus() || child.input(|i| i.key_pressed(egui::Key::Enter)) {
//                 if !self.rename_buffer.is_empty() && self.rename_buffer != file_name {
//                     let new_path = path.parent().unwrap().join(&self.rename_buffer);
//                     if let Err(e) = std::fs::rename(path, new_path) {
//                         log::error!("rename failed: {e}");
//                     }
//                 }
//                 self.renaming = None;
//             }
//         } else {
//             let galley = ui.painter().layout(
//                 file_name.clone(),
//                 egui::FontId::proportional(11.0),
//                 ui.visuals().text_color(),
//                 label_rect.width(),
//             );
//             let text_pos = egui::pos2(
//                 label_rect.center().x - galley.size().x / 2.0,
//                 label_rect.top(),
//             );
//             ui.painter()
//                 .galley(text_pos, galley, ui.visuals().text_color());

//             response.clone().on_hover_text(&file_name);
//         }

//         if response.clicked() {
//             self.selected = Some(path.to_path_buf());
//         }
//         if response.double_clicked() {
//             if is_dir {
//                 self.path.push(&file_name);
//                 self.selected = None;
//                 self.renaming = None;
//             } else {
//                 let target = std::fs::read_to_string(path)
//                     .ok()
//                     .and_then(|s| serde_json::from_str::<CommonFileDescriptor>(&s).ok())
//                     .map(|d| std::path::PathBuf::from(d.file_path))
//                     .unwrap_or_else(|| path.to_path_buf());

//                 #[cfg(target_os = "windows")]
//                 let _ = std::process::Command::new("cmd")
//                     .args(["/C", "start", "", target.to_str().unwrap_or("")])
//                     .spawn();
//             }
//         }

//         response.context_menu(|ui| {
//             if ui.button("change name").clicked() {
//                 self.renaming = Some(path.to_path_buf());
//                 self.rename_buffer = file_name.clone();
//                 ui.close();
//             }
//             ui.separator();
//             if ui.button("delete").clicked() {
//                 if trash::delete(path).is_err() {
//                     if is_dir {
//                         let _ = std::fs::remove_dir_all(path);
//                     } else {
//                         let _ = std::fs::remove_file(path);
//                     }
//                 }
//                 self.selected = None;
//                 ui.close();
//             }
//         });

//         return response;
//     }
// }

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

pub struct Assets {
    pub path: PathBuf,
    pub selected: Option<PathBuf>,

    renaming: Option<PathBuf>,
    rename_buffer: String,

    icons: HashMap<String, Option<egui::TextureHandle>>,

    /// Recty wszystkich celów dropu w bieżącej ramce.
    /// (rect, ścieżka folderu)
    drop_targets: Vec<(egui::Rect, PathBuf)>,
}

impl Default for Assets {
    fn default() -> Self {
        Self {
            path: PathBuf::from("smq_proj/assets"),
            selected: None,
            renaming: None,
            rename_buffer: String::new(),
            icons: HashMap::new(),
            drop_targets: Vec::new(),
        }
    }
}

impl Assets {
    const CARD_W: f32 = 84.0;
    const CARD_H: f32 = 96.0;
    const ICON: f32 = 48.0;
    const GAP: f32 = 6.0;
    const PAD: f32 = 8.0;

    pub fn egui(&mut self, ui: &mut egui::Ui) {
        self.drop_targets.clear();

        // ------------------------------------------------------------
        // Zewnętrzny drag & drop plików do edytora
        // ------------------------------------------------------------

        ui.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    let path = file.path();
                    if let Some(file_name) = path.file_name() {
                        let dest = self.path.join(file_name);

                        if let Err(e) = Self::copy_with_meta(path, &dest) {
                            log::error!(
                                "Failed to import dropped file {:?}: {}",
                                path,
                                e
                            );
                        }
                    }
                }
            }
        });

        // ------------------------------------------------------------
        // Lewy panel - drzewo folderów
        // ------------------------------------------------------------

        egui::Panel::left("assets_left_panel")
            .resizable(true)
            .default_size(150.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let root_path = PathBuf::from("smq_proj/assets");

                    if !root_path.exists() {
                        let _ = std::fs::create_dir_all(&root_path);
                    }

                    self.draw_folder_tree(ui, &root_path);
                });
            });

        // ------------------------------------------------------------
        // Główny panel
        // ------------------------------------------------------------

        egui::CentralPanel::default().show(ui, |ui| {
            self.draw_top_bar(ui);
            ui.separator();
            self.draw_center_files(ui);
        });

        // ------------------------------------------------------------
        // Drag preview + drop (na samym końcu, jeden globalny handler)
        // ------------------------------------------------------------

        self.draw_drag_preview(ui.ctx());
        self.process_drop(ui.ctx());
    }

    // ================================================================
    // META
    // ================================================================

    fn meta_path(path: &Path) -> PathBuf {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        path.with_file_name(format!("{}.meta", file_name))
    }

    fn is_meta(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("meta"))
    }

    fn ensure_meta(path: &Path) {
        if !path.exists() {
            return;
        }

        if Self::is_meta(path) {
            return;
        }

        let meta = Self::meta_path(path);

        if !meta.exists() {
            if let Err(e) = std::fs::write(&meta, "") {
                log::error!(
                    "Failed to create meta file {:?}: {}",
                    meta,
                    e
                );
            }
        }
    }

    fn move_with_meta(
        source: &Path,
        destination: &Path,
    ) -> std::io::Result<()> {
        if Self::is_meta(source) {
            return std::fs::rename(source, destination);
        }

        Self::ensure_meta(source);

        let source_meta = Self::meta_path(source);
        let destination_meta = Self::meta_path(destination);

        std::fs::rename(source, destination)?;

        if source_meta.exists() {
            if let Err(e) = std::fs::rename(
                &source_meta,
                &destination_meta,
            ) {
                log::error!(
                    "Failed to move meta {:?} -> {:?}: {}",
                    source_meta,
                    destination_meta,
                    e
                );

                Self::ensure_meta(destination);
            }
        } else {
            Self::ensure_meta(destination);
        }

        Ok(())
    }

    fn copy_with_meta(
        source: &Path,
        destination: &Path,
    ) -> std::io::Result<()> {
        if source.is_dir() {
            std::fs::create_dir_all(destination)?;

            let source_meta = Self::meta_path(source);
            let destination_meta = Self::meta_path(destination);

            if source_meta.exists() {
                std::fs::copy(&source_meta, &destination_meta)?;
            } else {
                Self::ensure_meta(destination);
            }

            return Ok(());
        }

        std::fs::copy(source, destination)?;

        let source_meta = Self::meta_path(source);
        let destination_meta = Self::meta_path(destination);

        if source_meta.exists() {
            std::fs::copy(&source_meta, &destination_meta)?;
        } else {
            Self::ensure_meta(destination);
        }

        Ok(())
    }

    fn delete_with_meta(path: &Path) -> std::io::Result<()> {
        if Self::is_meta(path) {
            return trash::delete(path)
                .map_err(std::io::Error::other);
        }

        let meta = Self::meta_path(path);

        if path.exists() {
            if let Err(e) = trash::delete(path) {
                log::debug!(
                    "Failed to move {:?} to trash: {}",
                    path,
                    e
                );

                if path.is_dir() {
                    std::fs::remove_dir_all(path)?;
                } else {
                    std::fs::remove_file(path)?;
                }
            }
        }

        if meta.exists() {
            if let Err(e) = trash::delete(&meta) {
                log::debug!(
                    "Failed to move {:?} to trash: {}",
                    meta,
                    e
                );

                if meta.is_dir() {
                    std::fs::remove_dir_all(&meta)?;
                } else {
                    std::fs::remove_file(&meta)?;
                }
            }
        }

        Ok(())
    }

    // ================================================================
    // FOLDER TREE
    // ================================================================

    fn draw_folder_tree(
        &mut self,
        ui: &mut egui::Ui,
        current_path: &Path,
    ) {
        let mut folder_name = current_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        let mut sub_dirs = Vec::new();

        if let Ok(rd) = std::fs::read_dir(current_path) {
            for entry in rd.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    sub_dirs.push(path);
                }
            }
        }

        sub_dirs.sort();

        let is_selected = self.path == current_path;

        let id = ui.id().with(current_path);

        let state =
            egui::collapsing_header::CollapsingState::
                load_with_default_open(
                    ui.ctx(),
                    id,
                    true,
                );

        state
            .show_header(ui, |ui| {
                let response = ui.selectable_label(
                    is_selected,
                    &folder_name,
                );

                if response.clicked() {
                    self.path = current_path.to_path_buf();
                    self.selected = None;
                }

                self.drop_targets.push((
                    response.rect,
                    current_path.to_path_buf(),
                ));
            })
            .body(|ui| {
                for dir in sub_dirs {
                    self.draw_folder_tree(ui, &dir);
                }
            });
    }

    // ================================================================
    // TOP BAR
    // ================================================================

    fn draw_top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let rel = self
                .path
                .display()
                .to_string()
                .replace('\\', "/")
                .replace("smq_proj/", "");

            let path_split: Vec<&str> = rel.split('/').collect();

            egui::Frame::default()
                .inner_margin(egui::Margin::symmetric(8, 0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.style_mut().visuals.button_frame = false;

                        for (i, part) in path_split.iter().enumerate() {
                            if i != 0 {
                                ui.label(">");
                            }

                            let response = ui.button(*part);

                            let mut build_path = "smq_proj".to_string();

                            for (j, p) in path_split.iter().enumerate() {
                                if j > i {
                                    break;
                                }

                                build_path.push('/');
                                build_path.push_str(p);
                            }

                            let target_path = PathBuf::from(&build_path);

                            if response.clicked() {
                                self.path = target_path.clone();
                                self.selected = None;
                            }

                            self.drop_targets
                                .push((response.rect, target_path));
                        }
                    });
                });
        });
    }

    // ================================================================
    // FILE GRID
    // ================================================================

    fn draw_center_files(&mut self, ui: &mut egui::Ui) {
        let relative_path = self
            .path
            .strip_prefix("smq_proj/assets")
            .unwrap_or(Path::new(""));

        let scripts_path = PathBuf::from("game/src").join(relative_path);

        let _ = std::fs::create_dir_all(&self.path);
        let _ = std::fs::create_dir_all(&scripts_path);

        let mut all_files = Vec::<PathBuf>::new();

        if let Ok(rd) = std::fs::read_dir(&self.path) {
            all_files.extend(
                rd.filter_map(|e| e.ok()).map(|e| e.path()),
            );
        }

        if let Ok(rd) = std::fs::read_dir(&scripts_path) {
            all_files.extend(
                rd.filter_map(|e| e.ok()).map(|e| e.path()),
            );
        }

        let mut display_files = Vec::<PathBuf>::new();
        let mut seen_folders = HashSet::new();

        for mut path in all_files {
            let is_dir = path.is_dir();

            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();

            if !is_dir && Self::is_meta(&path) {
                continue;
            }

            Self::ensure_meta(&path);

            if is_dir && !seen_folders.insert(name.clone()) {
                continue;
            }

            if !is_dir
                && path.extension().and_then(|s| s.to_str()) == Some("rs")
                && path.starts_with("smq_proj/assets")
            {
                let target = scripts_path.join(&name);

                if let Err(e) = Self::move_with_meta(&path, &target) {
                    log::error!(
                        "Failed to move Rust file {:?} -> {:?}: {}",
                        path,
                        target,
                        e
                    );
                }

                path = target;
            }

            display_files.push(path);
        }

        display_files.sort_by_key(|a| {
            (
                !a.is_dir(),
                a.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase(),
            )
        });

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let bg_rect = ui.available_rect_before_wrap();

                let bg = ui.interact(
                    bg_rect,
                    ui.id().with("asset_bg"),
                    egui::Sense::click(),
                );

                if bg.clicked() {
                    self.selected = None;
                }

                // Bieżący folder jako fallback drop target.
                // Wciskamy go z małym priorytetem — zostanie wybrany
                // tylko jeśli żaden inny target nie pasuje.
                // (Nie dodajemy tutaj — process_drop i tak użyje
                // self.path jako fallback.)

                // ----------------------------------------------------
                // PPM na pustym miejscu
                // ----------------------------------------------------

                bg.context_menu(|ui| {
                    if ui.button("new folder").clicked() {
                        let target = self.path.join("new_folder");

                        if let Err(e) =
                            std::fs::create_dir_all(&target)
                        {
                            log::error!(
                                "Failed to create folder: {}",
                                e
                            );
                        } else {
                            Self::ensure_meta(&target);

                            self.rename_buffer =
                                "new_folder".to_string();
                            self.renaming = Some(target);
                        }

                        ui.close();
                    }

                    if ui.button("new file").clicked() {
                        let target = self.path.join("new_file");

                        if let Err(e) =
                            std::fs::write(&target, "")
                        {
                            log::error!(
                                "Failed to create file: {}",
                                e
                            );
                        } else {
                            Self::ensure_meta(&target);

                            self.rename_buffer =
                                "new_file".to_string();
                            self.renaming = Some(target);
                        }

                        ui.close();
                    }

                    ui.separator();

                    if ui.button("import file").clicked() {
                        if let Some(picked) =
                            rfd::FileDialog::new().pick_file()
                        {
                            if let Some(fname) = picked.file_name() {
                                let dst = self.path.join(fname);

                                if let Err(e) = Self::copy_with_meta(
                                    &picked,
                                    &dst,
                                ) {
                                    log::error!(
                                        "import failed: {}",
                                        e
                                    );
                                }
                            }
                        }

                        ui.close();
                    }
                });

                let avail_w = ui.available_width();

                let usable = (avail_w - Assets::PAD * 2.0)
                    .max(Assets::CARD_W);

                let columns = ((usable + Assets::GAP)
                    / (Assets::CARD_W + Assets::GAP))
                    .floor()
                    .max(1.0) as usize;

                ui.add_space(Assets::PAD);

                for chunk in display_files.chunks(columns) {
                    ui.horizontal(|ui| {
                        ui.add_space(Assets::PAD);

                        ui.spacing_mut().item_spacing.x = Assets::GAP;

                        for path in chunk {
                            self.draw_file(
                                ui,
                                path,
                                Assets::CARD_W,
                                Assets::CARD_H,
                                Assets::ICON,
                            );
                        }
                    });

                    ui.add_space(Assets::GAP);
                }

                ui.add_space(Assets::PAD);
            });
    }

    // ================================================================
    // SINGLE FILE
    // ================================================================

    fn draw_file(
        &mut self,
        ui: &mut egui::Ui,
        path: &Path,
        card_w: f32,
        card_h: f32,
        icon_size: f32,
    ) -> egui::Response {
        let is_dir = path.is_dir();

        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        let ext = if is_dir {
            "folder".to_string()
        } else {
            path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_lowercase()
        };

        // ------------------------------------------------------------
        // Icon
        // ------------------------------------------------------------

        if !self.icons.contains_key(&ext) {
            let icon_path = format!("assets/icons/{}.png", ext);

            let handle = std::fs::read(&icon_path)
                .ok()
                .and_then(|bytes| image::load_from_memory(&bytes).ok())
                .map(|img| {
                    let size =
                        [img.width() as _, img.height() as _];

                    let rgba = img.to_rgba8();

                    let color =
                        egui::ColorImage::from_rgba_unmultiplied(
                            size,
                            rgba.as_raw(),
                        );

                    ui.ctx().load_texture(
                        &ext,
                        color,
                        egui::TextureOptions::LINEAR,
                    )
                });

            self.icons.insert(ext.clone(), handle);
        }

        // ------------------------------------------------------------
        // Interaction
        // ------------------------------------------------------------

        let (card_rect, response) = ui.allocate_exact_size(
            egui::vec2(card_w, card_h),
            egui::Sense::click_and_drag(),
        );

        if response.drag_started() {
            egui::DragAndDrop::set_payload(
                ui.ctx(),
                path.to_path_buf(),
            );
        }

        // ------------------------------------------------------------
        // Folder jako target dropu — tylko rejestrujemy rect
        // ------------------------------------------------------------

        if is_dir {
            self.drop_targets
                .push((card_rect, path.to_path_buf()));
        }

        // ------------------------------------------------------------
        // Selection
        // ------------------------------------------------------------

        let is_selected = self.selected.as_deref() == Some(path);
        let hovered = response.hovered();

        let bg = if is_selected {
            ui.visuals().selection.bg_fill
        } else if hovered {
            ui.visuals().widgets.hovered.weak_bg_fill
        } else {
            egui::Color32::TRANSPARENT
        };

        if bg != egui::Color32::TRANSPARENT {
            ui.painter().rect_filled(card_rect, 4.0, bg);
        }

        // ------------------------------------------------------------
        // Icon
        // ------------------------------------------------------------

        let icon_rect = egui::Rect::from_center_size(
            egui::pos2(
                card_rect.center().x,
                card_rect.top() + 6.0 + icon_size / 2.0,
            ),
            egui::vec2(icon_size, icon_size),
        );

        if let Some(Some(tex)) = self.icons.get(&ext) {
            let mut img = egui::Image::new(tex)
                .fit_to_exact_size(icon_rect.size());

            if is_selected {
                img = img.tint(egui::Color32::from_rgb(
                    150, 180, 255,
                ));
            }

            egui::Image::paint_at(&img, ui, icon_rect);
        } else {
            let fallback = if is_dir {
                egui::Color32::from_rgb(200, 170, 50)
            } else {
                egui::Color32::from_rgb(45, 85, 145)
            };

            ui.painter().rect_filled(icon_rect, 4.0, fallback);

            if is_selected {
                ui.painter().rect_stroke(
                    icon_rect,
                    4.0,
                    egui::Stroke::new(2.0, egui::Color32::WHITE),
                    egui::StrokeKind::Inside,
                );
            }
        }

        // ------------------------------------------------------------
        // Label
        // ------------------------------------------------------------

        let label_rect = egui::Rect::from_min_max(
            egui::pos2(
                card_rect.left() + 2.0,
                icon_rect.bottom() + 4.0,
            ),
            egui::pos2(
                card_rect.right() - 2.0,
                card_rect.bottom() - 2.0,
            ),
        );

        if self.renaming.as_deref() == Some(path) {
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(label_rect)
                    .layout(egui::Layout::top_down(
                        egui::Align::Center,
                    )),
            );

            let edit = child.add(
                egui::TextEdit::singleline(&mut self.rename_buffer)
                    .desired_width(label_rect.width()),
            );

            edit.request_focus();

            if edit.lost_focus()
                || child.input(|i| i.key_pressed(egui::Key::Enter))
            {
                if !self.rename_buffer.is_empty()
                    && self.rename_buffer != file_name
                {
                    let new_path = path
                        .parent()
                        .unwrap_or_else(|| Path::new("."))
                        .join(&self.rename_buffer);

                    if let Err(e) =
                        Self::move_with_meta(path, &new_path)
                    {
                        log::error!("rename failed: {}", e);
                    }
                }

                self.renaming = None;
            }
        } else {
            let galley = ui.painter().layout(
                file_name.clone(),
                egui::FontId::proportional(11.0),
                ui.visuals().text_color(),
                label_rect.width(),
            );

            let text_pos = egui::pos2(
                label_rect.center().x - galley.size().x / 2.0,
                label_rect.top(),
            );

            ui.painter().galley(
                text_pos,
                galley,
                ui.visuals().text_color(),
            );

            response.clone().on_hover_text(&file_name);
        }

        // ------------------------------------------------------------
        // Click
        // ------------------------------------------------------------

        if response.clicked() {
            self.selected = Some(path.to_path_buf());
        }

        // ------------------------------------------------------------
        // Double click
        // ------------------------------------------------------------

        if response.double_clicked() {
            if is_dir {
                self.path.push(&file_name);
                self.selected = None;
                self.renaming = None;
            } else {
                #[cfg(target_os = "windows")]
                {
                    let _ = std::process::Command::new("cmd")
                        .args([
                            "/C",
                            "start",
                            "",
                            path.to_str().unwrap_or(""),
                        ])
                        .spawn();
                }
            }
        }

        // ------------------------------------------------------------
        // Context menu
        // ------------------------------------------------------------

        response.context_menu(|ui| {
            if ui.button("change name").clicked() {
                self.renaming = Some(path.to_path_buf());
                self.rename_buffer = file_name.clone();
                ui.close();
            }

            ui.separator();

            if ui.button("delete").clicked() {
                if let Err(e) = Self::delete_with_meta(path) {
                    log::error!(
                        "Failed to delete {:?}: {}",
                        path,
                        e
                    );
                }

                self.selected = None;
                self.renaming = None;

                ui.close();
            }
        });

        response
    }

    // ================================================================
    // DRAG PREVIEW (tooltip pod kursorem)
    // ================================================================

    fn draw_drag_preview(&self, ctx: &egui::Context) {
        let Some(payload_path) =
            egui::DragAndDrop::payload::<PathBuf>(ctx)
        else {
            return;
        };

        if !ctx.input(|i| i.pointer.primary_down()) {
            return;
        }

        let Some(pointer_pos) = ctx.pointer_interact_pos() else {
            return;
        };

        let file_name = payload_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        let is_dir = payload_path.is_dir();
        let ext = if is_dir {
            "folder".to_string()
        } else {
            payload_path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_lowercase()
        };

        egui::Area::new(egui::Id::new("file_drag_preview"))
            .fixed_pos(pointer_pos + egui::vec2(14.0, 14.0))
            .order(egui::Order::Tooltip)
            .interactable(false)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let preview_size = egui::vec2(20.0, 20.0);

                        if let Some(Some(tex)) =
                            self.icons.get(&ext)
                        {
                            ui.add(
                                egui::Image::new(tex)
                                    .fit_to_exact_size(preview_size),
                            );
                        } else {
                            let fallback = if is_dir {
                                egui::Color32::from_rgb(200, 170, 50)
                            } else {
                                egui::Color32::from_rgb(45, 85, 145)
                            };

                            let (rect, _) = ui.allocate_exact_size(
                                preview_size,
                                egui::Sense::hover(),
                            );

                            ui.painter().rect_filled(rect, 2.0, fallback);
                        }

                        ui.label(
                            egui::RichText::new(&file_name)
                                .size(12.0)
                                .color(ui.visuals().text_color()),
                        );
                    });
                });
            });
    }

    // ================================================================
    // GLOBAL DROP
    // ================================================================

    fn process_drop(&mut self, ctx: &egui::Context) {
        let Some(payload_path) =
            egui::DragAndDrop::payload::<PathBuf>(ctx)
        else {
            return;
        };

        let Some(pointer_pos) = ctx.pointer_interact_pos() else {
            return;
        };

        // Znajdź najgłębszy (najmniejszy) target pod kursorem.
        let mut best: Option<(f32, PathBuf)> = None;
        for (rect, path) in &self.drop_targets {
            if rect.contains(pointer_pos) {
                let area = rect.width() * rect.height();
                if best.as_ref().is_none_or(|(a, _)| area < *a) {
                    best = Some((area, path.clone()));
                }
            }
        }

        let target = best
            .map(|(_, p)| p)
            .unwrap_or_else(|| self.path.clone());

        // Walidacja: nie można przenieść do samego siebie
        // ani do swojego dziecka.
        let src = payload_path.as_path();
let valid = src != target.as_path() && !target.starts_with(src);

        if !valid {
            return;
        }

        // Drop dopiero w momencie puszczenia przycisku.
        if ctx.input(|i| i.pointer.primary_released()) {
            let Some(payload_path) =
                egui::DragAndDrop::take_payload::<PathBuf>(ctx)
            else {
                return;
            };

            if let Some(file_name) = payload_path.file_name() {
                let new_path = target.join(file_name);

                if new_path != *payload_path {
                    if let Err(e) =
                        Self::move_with_meta(&payload_path, &new_path)
                    {
                        log::error!(
                            "Failed to move asset {:?} -> {:?}: {}",
                            payload_path,
                            new_path,
                            e
                        );
                    } else {
                        self.selected = Some(new_path);
                    }
                }
            }
        }
    }
}