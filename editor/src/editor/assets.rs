use std::{collections::{HashMap, HashSet}, path::PathBuf};

use crate::CommonFileDescriptor;

// this sucks

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
        // ---- tunables ----
        const CARD_W: f32   = 84.0;   // card width in px
        const CARD_H: f32   = 96.0;   // card height in px
        const ICON: f32     = 48.0;   // icon size in px
        const GAP: f32      = 6.0;    // gap between cards
        const PAD: f32      = 8.0;    // padding inside the panel
        // ------------------

        let root_path = std::path::PathBuf::from("smq_proj/assets");

        // ---------- breadcrumb / top bar ----------
        ui.horizontal(|ui| {
            let back_btn = ui.button("⬅ back").clicked();
            let back_mouse = ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Extra1));

            if (back_btn || back_mouse) && self.path != root_path {
                self.path.pop();
                self.selected = None;
            }

            ui.separator();

            let rel = self
                .path
                .display()
                .to_string()
                .replace('\\', "/")
                .replace("smq_proj/assets", "");
            ui.label(if rel.is_empty() { "/".to_string() } else { rel });
        });
        ui.separator();

        // ---------- gather files ----------
        let relative_path = self
            .path
            .strip_prefix("smq_proj/assets")
            .unwrap_or(std::path::Path::new(""));
        let scripts_path = std::path::PathBuf::from("game/src").join(relative_path);

        let _ = std::fs::create_dir_all(&self.path);
        let _ = std::fs::create_dir_all(&scripts_path);

        let mut all_files: Vec<std::path::PathBuf> = Vec::new();
        if let Ok(rd) = std::fs::read_dir(&self.path) {
            all_files.extend(rd.filter_map(|e| e.ok()).map(|e| e.path()));
        }
        if let Ok(rd) = std::fs::read_dir(&scripts_path) {
            all_files.extend(rd.filter_map(|e| e.ok()).map(|e| e.path()));
        }

        // dedupe folders, migrate .rs files, build display list
        let mut display_files: Vec<std::path::PathBuf> = Vec::new();
        let mut seen_folders = HashSet::new();

        for mut p in all_files {
            let is_dir = p.is_dir();
            let name = p.file_name().unwrap().to_string_lossy().into_owned();

            if is_dir && !seen_folders.insert(name.clone()) {
                continue;
            }

            if !is_dir
                && p.extension().and_then(|s| s.to_str()) == Some("rs")
                && p.starts_with("smq_proj/assets")
            {
                let target = scripts_path.join(&name);
                if std::fs::rename(&p, &target).is_err() {
                    if std::fs::copy(&p, &target).is_ok() {
                        let _ = std::fs::remove_file(&p);
                    }
                }
                p = target;
            }

            display_files.push(p);
        }

        // folders first, then files, alphabetical
        display_files.sort_by_key(|a| {
            (!a.is_dir(), a.file_name().unwrap().to_string_lossy().to_lowercase())
        });

        // ---------- scrollable grid ----------
        let scroll = egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // right-click anywhere on the empty background
                let bg_rect = ui.available_rect_before_wrap();
                let bg = ui.interact(bg_rect, ui.id().with("asset_bg"), egui::Sense::click());
                if bg.clicked() {
                    self.selected = None;
                }
                bg.context_menu(|ui| {
                    if ui.button("new folder").clicked() {
                        let target = self.path.join("new_folder");
                        let _ = std::fs::create_dir_all(&target);
                        self.rename_buffer = "new_folder".to_string();
                        self.renaming = Some(target);
                        ui.close();
                    }
                    if ui.button("new file").clicked() {
                        let target = self.path.join("new_file");
                        let _ = std::fs::write(&target, "");
                        self.rename_buffer = "new_file".to_string();
                        self.renaming = Some(target);
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("import file").clicked() {
                        if let Some(picked) = rfd::FileDialog::new().pick_file() {
                            if let Some(fname) = picked.file_name() {
                                let dst = self.path.join(fname);
                                if let Err(e) = std::fs::copy(&picked, &dst) {
                                    eprintln!("import failed: {e}");
                                }
                            }
                        }
                        ui.close();
                    }
                });

                // available width → columns per row
                let avail_w = ui.available_width();
                let usable = (avail_w - PAD * 2.0).max(CARD_W);
                let columns = ((usable + GAP) / (CARD_W + GAP)).floor().max(1.0) as usize;

                ui.add_space(PAD);

                // lay out rows manually — full control over hit rects
                for chunk in display_files.chunks(columns) {
                    ui.horizontal(|ui| {
                        ui.add_space(PAD);
                        ui.spacing_mut().item_spacing.x = GAP;

                        for path in chunk {
                            self.draw_card(ui, path, CARD_W, CARD_H, ICON);
                        }
                    });
                    ui.add_space(GAP);
                }

                ui.add_space(PAD);
            });

        // keep selection scroll-aware (optional): scroll.scroll_to_me can be used elsewhere
        let _ = scroll;
    }

    /// Draws one card and handles its interactions. Returns the response if needed.
    fn draw_card(
        &mut self,
        ui: &mut egui::Ui,
        path: &std::path::Path,
        card_w: f32,
        card_h: f32,
        icon_size: f32,
    ) -> egui::Response {
        let is_dir = path.is_dir();
        let file_name = path.file_name().unwrap().to_string_lossy().into_owned();

        let ext = if is_dir {
            "folder".to_string()
        } else {
            path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_lowercase()
        };

        // lazy icon load
        if !self.icons.contains_key(&ext) {
            let icon_path = format!("assets/icons/{}.png", ext);
            let handle = std::fs::read(&icon_path)
                .ok()
                .and_then(|bytes| image::load_from_memory(&bytes).ok())
                .map(|img| {
                    let size = [img.width() as _, img.height() as _];
                    let rgba = img.to_rgba8();
                    let color = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
                    ui.ctx().load_texture(&ext, color, egui::TextureOptions::LINEAR)
                });
            self.icons.insert(ext.clone(), handle);
        }

        // --- THE key: one allocation for the whole card ---
        let (card_rect, response) = ui.allocate_exact_size(
            egui::vec2(card_w, card_h),
            egui::Sense::click_and_drag(),
        );

        // drag payload — PathBuf, works with existing getters
        if response.drag_started() {
            egui::DragAndDrop::set_payload(ui.ctx(), path.to_path_buf());
        }

        let is_selected = self.selected.as_deref() == Some(path);
        let hovered = response.hovered();

        // --- background ---
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

        // --- content: draw directly into card_rect, no child UI ---
        // icon box (centered horizontally near top)
        let icon_rect = egui::Rect::from_center_size(
            egui::pos2(card_rect.center().x, card_rect.top() + 6.0 + icon_size / 2.0),
            egui::vec2(icon_size, icon_size),
        );

        if let Some(Some(tex)) = self.icons.get(&ext) {
            let mut img = egui::Image::new(tex).fit_to_exact_size(icon_rect.size());
            if is_selected {
                img = img.tint(egui::Color32::from_rgb(150, 180, 255));
            }
            // paint image directly into icon_rect
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

        // label area (below icon)
        let label_rect = egui::Rect::from_min_max(
            egui::pos2(card_rect.left() + 2.0, icon_rect.bottom() + 4.0),
            egui::pos2(card_rect.right() - 2.0, card_rect.bottom() - 2.0),
        );

        if self.renaming.as_deref() == Some(path) {
            // inline rename field
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(label_rect)
                    .layout(egui::Layout::top_down(egui::Align::Center)),
            );
            let edit = child.add(
                egui::TextEdit::singleline(&mut self.rename_buffer)
                    .desired_width(label_rect.width()),
            );
            edit.request_focus();

            if edit.lost_focus() || child.input(|i| i.key_pressed(egui::Key::Enter)) {
                if !self.rename_buffer.is_empty() && self.rename_buffer != file_name {
                    let new_path = path.parent().unwrap().join(&self.rename_buffer);
                    if let Err(e) = std::fs::rename(path, new_path) {
                        eprintln!("rename failed: {e}");
                    }
                }
                self.renaming = None;
            }
        } else {
            // truncated label, centered, with tooltip
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
            ui.painter().galley(text_pos, galley, ui.visuals().text_color());

            // tooltip for full name
            response.clone().on_hover_text(&file_name);
        }

        // --- interactions ---
        if response.clicked() {
            self.selected = Some(path.to_path_buf());
        }
        if response.double_clicked() {
            if is_dir {
                self.path.push(&file_name);
                self.selected = None;
                self.renaming = None;
            } else {
                // if it's a JSON CommonFileDescriptor, open the referenced file
                let target = std::fs::read_to_string(path)
                    .ok()
                    .and_then(|s| serde_json::from_str::<CommonFileDescriptor>(&s).ok())
                    .map(|d| std::path::PathBuf::from(d.file_path))
                    .unwrap_or_else(|| path.to_path_buf());

                #[cfg(target_os = "windows")]
                let _ = std::process::Command::new("cmd")
                    .args(["/C", "start", "", target.to_str().unwrap_or("")])
                    .spawn();
            }
        }

        // right-click context menu per card
        response.context_menu(|ui| {
            if ui.button("change name").clicked() {
                self.renaming = Some(path.to_path_buf());
                self.rename_buffer = file_name.clone();
                ui.close();
            }
            ui.separator();
            if ui.button("delete").clicked() {
                if trash::delete(path).is_err() {
                    if is_dir {
                        let _ = std::fs::remove_dir_all(path);
                    } else {
                        let _ = std::fs::remove_file(path);
                    }
                }
                self.selected = None;
                ui.close();
            }
        });

        response
    }
}