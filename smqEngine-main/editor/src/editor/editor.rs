use bevy_ecs::prelude::*;
use crate::{components::*, editor::{editor_assets, inspector::editor_inspector}};
use smq_engine::*;

const LAYOUT_FILE_PATH: &str = "smq_proj/editor_layout.json";

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EditorTabs {
    Assets,
    Inspector,
    Output
}

#[derive(Resource)]
pub struct EditorState {
    pub tree: egui_dock::DockState<EditorTabs>
}

struct EditorTabViewer<'a> {
    world: &'a mut World
}

impl Default for EditorState {
    fn default() -> Self {
        let mut tree = egui_dock::DockState::new(vec![EditorTabs::Assets]);
        
        let [viewport_node, _hierarchy_node] = tree.main_surface_mut().split_left(
            egui_dock::NodeIndex::root(),
            0.2,
            vec![EditorTabs::Inspector],
        );

        tree.main_surface_mut().split_left(
            viewport_node,
            0.5, 
            vec![EditorTabs::Output]
        );

        Self { tree }
    }
}

impl EditorState {
    pub fn load_or_default() -> Self {
        if let Ok(json_string) = std::fs::read_to_string(LAYOUT_FILE_PATH) {
            if let Ok(saved_tree) = serde_json::from_str::<egui_dock::DockState<EditorTabs>>(&json_string) {
                return Self { tree: saved_tree };
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
            EditorTabs::Assets => "assets".into(),
            EditorTabs::Inspector => "inspector".into(),
            EditorTabs::Output => "output".into()
        }
    }

    fn scroll_bars(&self, _tab: &Self::Tab) -> [bool; 2] {
        return [true, true];
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            EditorTabs::Assets => {
                editor_assets(ui, self.world);
            }
            EditorTabs::Inspector => {
                editor_inspector(ui, self.world);
            }
            EditorTabs::Output => {
              
            }
        }
    }
}

pub fn editor_update(world: &mut World) {

    let delta = world.get_resource::<Delta>().expect("delta not found").delta;

    let mut dock_state = world.remove_resource::<EditorState>().unwrap_or_else(EditorState::load_or_default);

    egui::TopBottomPanel::top("top_menu_bar").show(&ui(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("windows", |ui| {
                
                let tabs_to_toggle = [
                    (EditorTabs::Assets, "assets"),
                    (EditorTabs::Inspector, "inspector"),
                    (EditorTabs::Output, "output")
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
            ui.label(format!("fps: {:.1} delta {}", 1.0/delta, delta ));
        });
    });

    egui::CentralPanel::default().show(&ui(), |ui| {
        let mut tab_viewer = EditorTabViewer { world };
        
        egui_dock::DockArea::new(&mut dock_state.tree)
            .style(egui_dock::Style::from_egui(ui.style()))
            .show_inside(ui, &mut tab_viewer);
    });

    world.insert_resource(dock_state);
}