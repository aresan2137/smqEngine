use std::path::PathBuf;

use smq_engine::*;

use crate::editor::inspector::InspectorState;

pub fn wgsl(ui: &mut egui::Ui, inspector_state: &mut InspectorState, path: &PathBuf) {
    ui.heading("Material");
    ui.separator();

    let json_path = std::path::PathBuf::from(format!("{}.json", path.display()));

    if inspector_state.target_json_path.as_ref() != Some(&json_path) {
        if let Ok(json_str) = std::fs::read_to_string(&json_path) {
            if let Ok(loaded_data) = serde_json::from_str::<MaterialJson>(&json_str) {
                inspector_state.mat_edit = loaded_data;
                inspector_state.target_json_path = Some(json_path.clone());
            }
        } else {
            inspector_state.mat_edit = MaterialJson::default();
            inspector_state.target_json_path = Some(json_path.clone());
        }
    }

    let mut changed = false;

    ui.add_space(10.0);
    ui.label("drop to update");

    ui.horizontal(|ui| {
        ui.label("Render Texture:");
        
        let drop_rect = ui.available_rect_before_wrap();
        let (rect, response) = ui.allocate_exact_size(egui::vec2(drop_rect.width(), 30.0), egui::Sense::hover());
        
        let stroke_color = if response.hovered() { egui::Color32::GREEN } else { egui::Color32::from_gray(80) };
        ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.0, stroke_color), egui::StrokeKind::Inside);
        ui.put(rect, egui::Label::new("Upuść RenderTexture tutaj").selectable(false));

        if let Some(payload) = response.dnd_release_payload::<PathBuf>() {
            if let Ok(json_str) = std::fs::read_to_string(payload.as_ref()) {
                if let Ok(rt_meta) = serde_json::from_str::<RenderTextureEdit>(&json_str) {
                    inspector_state.mat_edit.atachments.clear();
                    
                    for color_attach in rt_meta.color_attachments {
                        inspector_state.mat_edit.atachments.push(color_attach.format);
                    }
                    
                    changed = true;
                }
            }
        }
    });
    ui.add_space(20.0);

    let culling = &inspector_state.mat_edit.culling;

    egui::ComboBox::from_id_salt("culling_select")
        .selected_text(culling)
        .show_ui(ui, |ui| {
            if ui.selectable_value(&mut inspector_state.mat_edit.culling, "None".to_string(), "None").changed() {
                changed = true;
            }
            if ui.selectable_value(&mut inspector_state.mat_edit.culling, "Back".to_string(), "Back").changed() {
                changed = true;
            }
            if ui.selectable_value(&mut inspector_state.mat_edit.culling, "Front".to_string(), "Front").changed() {
                changed = true;
            }            
        });

    ui.label(format!("attachments: {:?}", inspector_state.mat_edit.atachments));

    if changed {
        if let Ok(json_str) = serde_json::to_string_pretty(&inspector_state.mat_edit) {
            let _ = std::fs::write(&json_path, json_str);
        }
    }
}