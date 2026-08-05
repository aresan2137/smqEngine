use std::path::PathBuf;

use crate::editor::inspector::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default)]
pub struct UboVariable {
    pub name: String,
    pub var_type: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default)]
pub struct UboEdit {
    pub is_dynamic: bool,
    pub max_instances: u32,
    pub variables: Vec<UboVariable>
}

pub fn ubo(ui: &mut egui::Ui, inspector_state: &mut InspectorState, path: &PathBuf) {
    ui.heading("UBO");
    ui.separator();

    if inspector_state.target_json_path.as_ref() != Some(path) {
        if let Ok(json_str) = std::fs::read_to_string(path) {
            if let Ok(loaded_data) = serde_json::from_str::<UboEdit>(&json_str) {
                inspector_state.ubo_edit = loaded_data;
                inspector_state.target_json_path = Some(path.clone());
            }
        } else {
            inspector_state.ubo_edit = UboEdit::default();
            inspector_state.target_json_path = Some(path.clone());
        }
    }

    let mut changed = false;

    ui.horizontal(|ui| {
        if ui.checkbox(&mut inspector_state.ubo_edit.is_dynamic, "Dynamic UBO").changed() {
            changed = true;
        }
    });

    if inspector_state.ubo_edit.is_dynamic {
        ui.horizontal(|ui| {
            ui.label("Instances:");
            if ui.add(egui::DragValue::new(&mut inspector_state.ubo_edit.max_instances).speed(1)).changed() {
                changed = true;
            }
        });
    }

    ui.add_space(10.0);
    ui.heading("layout");
    ui.separator();

    if ui.button("add").clicked() {
        inspector_state.ubo_edit.variables.push(UboVariable {
            name: "nowa_zmienna".to_string(),
            var_type: "Vec4".to_string()
        });
        changed = true;
    }

    ui.add_space(4.0);

    let mut to_remove = None;

    for (idx, var) in inspector_state.ubo_edit.variables.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            if ui.add(egui::TextEdit::singleline(&mut var.name).desired_width(120.0)).changed() {
                changed = true;
            }

            egui::ComboBox::from_id_salt(format!("ubo_var_type_{}", idx))
                .selected_text(&var.var_type)
                .width(80.0)
                .show_ui(ui, |ui| {
                    let types = [
                        "Float", "Int", "UInt", 
                        "Vec2", "Vec3", "Vec4", 
                        "Mat2", "Mat3", "Mat4"
                    ];
                    
                    for t in types {
                        if ui.selectable_value(&mut var.var_type, t.to_string(), t).changed() {
                            changed = true;
                        }
                    }
                });

            if ui.button("remove").clicked() {
                to_remove = Some(idx);
            }
        });
    }

    if let Some(idx_to_delete) = to_remove {
        inspector_state.ubo_edit.variables.remove(idx_to_delete);
        changed = true;
    }

    if changed {
        if let Ok(json_str) = serde_json::to_string_pretty(&inspector_state.ubo_edit) {
            let _ = std::fs::write(path, json_str);
        }
    }
}