use std::process::Command;

use bevy_ecs::prelude::*;
use egui::TextureId;
use crate::{bake::bake, components::*};
use smq_engine::*;

mod settings;
mod assets;
pub mod inspector;

const LAYOUT_FILE_PATH: &str = "smq_proj/editor_layout.json";

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EditorTabs {
    Viewport,
    Inspector,
    Assets,
    SceneView,
    Settings
}

#[derive(Resource)]
pub struct EditorState {
    pub tree: egui_dock::DockState<EditorTabs>,

    settings: settings::Settings,
    assets: assets::Assets,
    inspector: inspector::Inspector,

    viewportid: Option<TextureId>
}

#[allow(unused)]
struct EditorTabViewer<'a> {
    world: &'a mut World,

    settings: &'a mut settings::Settings,
    assets: &'a mut assets::Assets,
    inspector: &'a mut inspector::Inspector,

    viewportid: Option<TextureId>
}

impl Default for EditorState {
    fn default() -> Self {
        let mut tree = egui_dock::DockState::new(vec![EditorTabs::Viewport]);
        
        let [viewport_node, _hierarchy_node] = tree.main_surface_mut().split_left(
            egui_dock::NodeIndex::root(),
            0.2,
            vec![EditorTabs::Inspector],
        );

        tree.main_surface_mut().split_below(
            viewport_node,
            0.5, 
            vec![EditorTabs::Assets]
        );

        Self { 
            tree, 
            settings: settings::Settings::default(),
            assets: assets::Assets::default(),
            inspector: inspector::Inspector::default(),
            viewportid: None
        }
    }
}

impl EditorState {
    pub fn load_or_default() -> Self {
        if let Ok(json_string) = std::fs::read_to_string(LAYOUT_FILE_PATH) {
            if let Ok(saved_tree) = serde_json::from_str::<egui_dock::DockState<EditorTabs>>(&json_string) {
                return Self { 
                    tree: saved_tree, 
                    settings: settings::Settings::default(),
                    assets: assets::Assets::default(),
                    inspector: inspector::Inspector::default(),
                    viewportid: None
                };
            }
        }
        
        Self::default()
    }
}

pub fn save_editor_layout(world: &World) {
    if let Some(dock_state) = world.get_resource::<EditorState>() {
        match serde_json::to_string_pretty(&dock_state.tree) {
            Ok(json_string) => {
                if let Err(e) = std::fs::write(LAYOUT_FILE_PATH, json_string) {
                    eprintln!("failed to save layout: {}", e);
                }
            }
            Err(e) => eprintln!("failed to parce layout: {}", e),
        }
    }
}

impl<'a> egui_dock::TabViewer for EditorTabViewer<'a> {
    type Tab = EditorTabs;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            EditorTabs::Viewport => "viewport".into(),
            EditorTabs::Inspector => "inspector".into(),
            EditorTabs::Assets => "assets".into(),
            EditorTabs::SceneView => "sceneview".into(),
            EditorTabs::Settings => "settings".into()
        }
    }

    fn scroll_bars(&self, _tab: &Self::Tab) -> [bool; 2] {
        return [true, true];
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            EditorTabs::Viewport => {
                
                if let Some(texture_id) = self.viewportid {
                    let available_size = ui.available_size();
                    let target_aspect = 16.0 / 9.0;

                    let mut render_size = available_size;
                    if available_size.x / available_size.y > target_aspect {
                        render_size.x = available_size.y * target_aspect;
                    } else {
                        render_size.y = available_size.x / target_aspect;
                    }

                    let (rect, _response) = ui.allocate_exact_size(available_size, egui::Sense::click_and_drag());

                    ui.painter().rect_filled(rect, 0.0, egui::Color32::from_gray(40));

                    let image_rect = egui::Rect::from_center_size(rect.center(), render_size);

                    let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
                    ui.painter().image(texture_id, image_rect, uv, egui::Color32::WHITE);
                } else {
                    ui.label("viewportId = None");
                }

            }
            EditorTabs::Inspector => {
                self.inspector.egui(ui, &mut self.assets);
            }
            EditorTabs::Assets => {
                self.assets.egui(ui);
            }
            EditorTabs::SceneView => {
                ui.label("testiog");
            }
            EditorTabs::Settings => {
                self.settings.egui(ui);
            }
        }
    }
}


pub fn editor_set_viewport_texture_id(world: &mut World, id: TextureId) {
    let mut dock_state = world.remove_resource::<EditorState>().unwrap_or_else(EditorState::load_or_default);

    dock_state.viewportid = Some(id);

    world.insert_resource(dock_state);
}

pub fn editor_update(world: &mut World) {

    #[allow(unused)]
    let delta = world.get_resource::<Delta>().expect("delta not found").delta;

    let mut dock_state = world.remove_resource::<EditorState>().unwrap_or_else(EditorState::load_or_default);

    egui::TopBottomPanel::top("top_menu_bar").show(&ui(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("windows", |ui| {
                
                let tabs_to_toggle = [
                    (EditorTabs::Viewport, "viewport"),
                    (EditorTabs::Inspector, "inspector"),
                    (EditorTabs::Assets, "assets"),
                    (EditorTabs::SceneView, "sceneview"),
                    (EditorTabs::Settings, "settings")
                ];

                for (tab_variant, label) in tabs_to_toggle {
                    let mut is_open = dock_state.tree.find_tab(&tab_variant).is_some();
                    
                    if ui.checkbox(&mut is_open, label).clicked() {
                        if is_open {
                            dock_state.tree.main_surface_mut().push_to_focused_leaf(tab_variant.clone());
                        } else {
                            if let Some(tab_location) = dock_state.tree.find_tab(&tab_variant) {
                                dock_state.tree.remove_tab(tab_location);
                            }
                        }
                        ui.close_menu();
                    }
                }
            });

            ui.horizontal(|ui| {
                if ui.button("play").clicked() {
                    bake().unwrap();
                    let _ = Command::new("cargo").arg("run").arg("-p").arg("smq_game").spawn().expect("failed to run game");
                }
                if ui.button("play no bake").clicked() {
                    let _ = Command::new("cargo").arg("run").arg("-p").arg("smq_game").spawn().expect("failed to run game");
                }
                if ui.button("bake").clicked() {
                    bake().unwrap();
                }
            });      
        });
    });

    egui::CentralPanel::default().show(&ui(), |ui| {
        let mut tab_viewer = EditorTabViewer { 
            world, 
            settings: &mut dock_state.settings,
            assets: &mut dock_state.assets,
            inspector: &mut dock_state.inspector,

            viewportid: dock_state.viewportid
        };
        
        egui_dock::DockArea::new(&mut dock_state.tree).style(egui_dock::Style::from_egui(ui.style())).show_inside(ui, &mut tab_viewer);
    });

    world.insert_resource(dock_state);
}