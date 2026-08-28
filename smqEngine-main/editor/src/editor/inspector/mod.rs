use bevy_ecs::prelude::*;
use egui::UiBuilder;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};

use smq_engine::*;

use crate::editor::EditorAssets;

mod png;
mod rendertexture;
mod smf;
mod ubo;
mod wgsl;

#[derive(Resource, Default)]
pub struct InspectorState {
    pub inspecting_path: Option<PathBuf>,
    pub is_locked: bool,
    pub target_json_path: Option<PathBuf>,
    
    pub mat_edit: MaterialJson,
    pub render_texture_edit: RenderTextureEdit,
    pub texture_edit: TextureJson,
    pub ubo_edit: ubo::UboEdit,
    
    pub width_buffer: String,
    pub height_buffer: String,
    
    pub is_editing_width: bool,
    pub is_editing_height: bool
}

pub fn editor_inspector(ui: &mut egui::Ui, world: &mut World) {
    let selected_path = world
        .get_resource::<EditorAssets>()
        .and_then(|state| state.selected_item.clone());

    let mut inspector_state = world.remove_resource::<InspectorState>().unwrap_or_default();

    if !inspector_state.is_locked {
        inspector_state.inspecting_path = selected_path.clone();
    }

    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let lock_text = if inspector_state.is_locked { "Locked" } else { "Unlocked" };
            ui.toggle_value(&mut inspector_state.is_locked, lock_text);
        });
    });
    ui.separator();

    if let Some(path) = &inspector_state.inspecting_path.clone() {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        if ext == "png" {
            png::png(ui, &mut inspector_state, path);
        } else if ext == "renderTexture" {
            rendertexture::render_texture(ui, &mut inspector_state, path);
        } else if ext == "blend" {
            smf::blend(ui, &mut inspector_state, path);
        } else if ext == "ubo" {
            ubo::ubo(ui, &mut inspector_state, path);
        } else if ext == "wgsl" {
            wgsl::wgsl(ui, &mut inspector_state, path);
        } else {
            ui.heading("file");
            ui.label(format!("file: {}", path.file_name().unwrap().to_string_lossy()));
            ui.label(format!("extent: .{}", ext));
        }
    } else {
        ui.centered_and_justified(|ui| {
            ui.label("select file or object");
        });
    }

    world.insert_resource(inspector_state);
}



