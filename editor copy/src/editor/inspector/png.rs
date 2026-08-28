use std::path::PathBuf;

use crate::editor::inspector::*;

pub fn png(ui: &mut egui::Ui, inspector_state: &mut InspectorState, path: &PathBuf) {
    ui.heading("Texture");
    ui.separator();

    let json_path = std::path::PathBuf::from(format!("{}.json", path.display()));

    if inspector_state.target_json_path.as_ref() != Some(&json_path) {
        if let Ok(json_str) = std::fs::read_to_string(&json_path) {
            if let Ok(loaded_data) = serde_json::from_str::<TextureJson>(&json_str) {
                inspector_state.texture_edit = loaded_data;
                inspector_state.target_json_path = Some(json_path.clone()); 
            }
        }
    }

    let mut changed = false;

    let sample = &inspector_state.texture_edit.sample;

    egui::ComboBox::from_id_salt("sample_select")
        .selected_text(sample)
        .show_ui(ui, |ui| {
            if ui.selectable_value(&mut inspector_state.texture_edit.sample, "Nearest".to_string(), "Nearest").changed() {
                changed = true;
            }
            if ui.selectable_value(&mut inspector_state.texture_edit.sample, "Bilinear".to_string(), "Bilinear").changed() {
                changed = true;
            }
            if ui.selectable_value(&mut inspector_state.texture_edit.sample, "Trilinear".to_string(), "Trilinear").changed() {
                changed = true;
            }
        });

    let repeat = &inspector_state.texture_edit.repeat;

    egui::ComboBox::from_id_salt("repeat_select")
        .selected_text(repeat)
        .show_ui(ui, |ui| {
            if ui.selectable_value(&mut inspector_state.texture_edit.repeat, "Repeat".to_string(), "Repeat").changed() {
                changed = true;
            }
            if ui.selectable_value(&mut inspector_state.texture_edit.repeat, "Clamp".to_string(), "Clamp").changed() {
                changed = true;
            }
        });

    if changed {
        if let Ok(json_str) = serde_json::to_string_pretty(&inspector_state.texture_edit) {
            let _ = std::fs::write(&json_path, json_str);
        }
    }
}