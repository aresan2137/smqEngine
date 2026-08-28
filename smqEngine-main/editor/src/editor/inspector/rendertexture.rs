use std::path::PathBuf;

use crate::editor::inspector::*;

pub fn render_texture(ui: &mut egui::Ui, inspector_state: &mut InspectorState, path: &PathBuf) {
    ui.heading("Render Texture");
    ui.separator();

    if inspector_state.target_json_path.as_ref() != Some(path) {
        if let Ok(json_str) = std::fs::read_to_string(path) {
            if let Ok(loaded_data) = serde_json::from_str::<RenderTextureEdit>(&json_str) {
                inspector_state.render_texture_edit = loaded_data;
                inspector_state.target_json_path = Some(path.clone());
            }
        }
    }

    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("width:");
        if !inspector_state.is_editing_width { 
            inspector_state.width_buffer = inspector_state.render_texture_edit.width.to_string(); 
        }
        let edit_w = ui.add(egui::TextEdit::singleline(&mut inspector_state.width_buffer).desired_width(60.0));
        if edit_w.gained_focus() { 
            inspector_state.is_editing_width = true; 
            inspector_state.width_buffer.clear(); 
        }
        if edit_w.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            inspector_state.is_editing_width = false;
            if let Ok(val) = inspector_state.width_buffer.parse::<u32>() {
                if inspector_state.render_texture_edit.width != val { 
                    inspector_state.render_texture_edit.width = val; 
                    changed = true; 
                }
            } else { 
                inspector_state.width_buffer = inspector_state.render_texture_edit.width.to_string(); 
            }
        }

        ui.add_space(20.0);

        ui.label("height:");
        if !inspector_state.is_editing_height { 
            inspector_state.height_buffer = inspector_state.render_texture_edit.height.to_string(); 
        }
        let edit_h = ui.add(egui::TextEdit::singleline(&mut inspector_state.height_buffer).desired_width(60.0));
        if edit_h.gained_focus() { 
            inspector_state.is_editing_height = true; 
            inspector_state.height_buffer.clear(); 
        }
        if edit_h.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            inspector_state.is_editing_height = false;
            if let Ok(val) = inspector_state.height_buffer.parse::<u32>() {
                if inspector_state.render_texture_edit.height != val { 
                    inspector_state.render_texture_edit.height = val; 
                    changed = true; 
                }
            } else { 
                inspector_state.height_buffer = inspector_state.render_texture_edit.height.to_string(); 
            }
        }
    });

    ui.separator();

    ui.horizontal(|ui| {
        ui.label("color atachments");
        if ui.button("add atachment").clicked() {
            inspector_state.render_texture_edit.color_attachments.push(RenderTextureTextureEdit { format: "Rgba8Unorm".to_string(), usage: vec!["RENDER_ATTACHMENT".to_string(), "TEXTURE_BINDING".to_string()] } );
            changed = true;
        }
    });

    ui.add_space(4.0);

    let mut to_remove = None;
    for (idx, format) in inspector_state.render_texture_edit.color_attachments.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.label(format!("[{}]", idx));
            
            egui::ComboBox::from_id_salt(format!("color_fmt_{}", idx))
                .selected_text(&*format.format)
                .show_ui(ui, |ui| {
                    ui.strong("8 bit:");
                    if ui.selectable_value(&mut format.format, "Rgba8Unorm".to_string(), "Rgba8Unorm").changed() { changed = true; }
                    if ui.selectable_value(&mut format.format, "Rgba8UnormSrgb".to_string(), "Rgba8UnormSrgb").changed() { changed = true; }
                    if ui.selectable_value(&mut format.format, "Bgra8Unorm".to_string(), "Bgra8Unorm").changed() { changed = true; }
                    
                    ui.separator();
                    ui.strong("16 bit");
                    if ui.selectable_value(&mut format.format, "R16Float".to_string(), "R16Float").changed() { changed = true; }
                    if ui.selectable_value(&mut format.format, "Rg16Float".to_string(), "Rg16Float").changed() { changed = true; }
                    if ui.selectable_value(&mut format.format, "Rgba16Float".to_string(), "Rgba16Float").changed() { changed = true; }
                    
                    ui.separator();
                    ui.strong("32 bit");
                    if ui.selectable_value(&mut format.format, "R32Float".to_string(), "R32Float").changed() { changed = true; }
                    if ui.selectable_value(&mut format.format, "Rg32Float".to_string(), "Rg32Float").changed() { changed = true; }
                    if ui.selectable_value(&mut format.format, "Rgba32Float".to_string(), "Rgba32Float").changed() { changed = true; }

                    ui.separator();
                    ui.strong("32 bit int");
                    if ui.selectable_value(&mut format.format, "R32Uint".to_string(), "R32Uint").changed() { changed = true; }
                });

            if ui.button("remove").clicked() {
                to_remove = Some(idx);
            }
        });
    }

    if let Some(idx_to_delete) = to_remove {
        inspector_state.render_texture_edit.color_attachments.remove(idx_to_delete);
        changed = true;
    }

    ui.separator();

    ui.horizontal(|ui| {
        ui.label("depth buffer:");
        
        let current_depth_text = match &inspector_state.render_texture_edit.depth_format {
            Some(fmt) => fmt.clone(),
            None => "None".to_string(),
        };

        egui::ComboBox::from_id_salt("depth_format_select")
            .selected_text(current_depth_text)
            .show_ui(ui, |ui| {
                if ui.selectable_value(&mut inspector_state.render_texture_edit.depth_format, None, "None").changed() {
                    changed = true;
                }
                if ui.selectable_value(&mut inspector_state.render_texture_edit.depth_format, Some("Depth32Float".to_string()), "Depth32Float").changed() {
                    changed = true;
                }
                if ui.selectable_value(&mut inspector_state.render_texture_edit.depth_format, Some("Depth24Plus".to_string()), "Depth24Plus").changed() {
                    changed = true;
                }
                if ui.selectable_value(&mut inspector_state.render_texture_edit.depth_format, Some("Depth24PlusStencil8".to_string()), "Depth24PlusStencil8").changed() {
                    changed = true;
                }
                if ui.selectable_value(&mut inspector_state.render_texture_edit.depth_format, Some("Depth16Unorm".to_string()), "Depth16Unorm").changed() {
                    changed = true;
                }
            });
    });

    if changed {
        if let Ok(json_str) = serde_json::to_string_pretty(&inspector_state.render_texture_edit) {
            let _ = std::fs::write(path, json_str);
        }
    }
}
