use std::{fs, path::{Path, PathBuf}};

use bevy_ecs::prelude::*;
use egui_dock::{DockState, NodeIndex, TabViewer};
use egui::{TextureId, Ui, WidgetText};
use serde::{Deserialize, Serialize};
use smq_engine::{drop_point, ui};

use crate::editor::settings::Settings;

mod assets;
mod settings;


#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum EditorTab {
    Assets,
    Inspector,
    Viewport,
    Scene,
    Settings
}

impl EditorTab {
    pub fn name(&self) -> &'static str {
        match self {
            EditorTab::Assets => "assets",
            EditorTab::Inspector => "inspector",
            EditorTab::Viewport => "viewport",
            EditorTab::Scene => "scene",
            EditorTab::Settings => "settings"
        }
    }

    pub fn all() -> [EditorTab; 5] {
        [EditorTab::Assets, EditorTab::Inspector, EditorTab::Viewport, EditorTab::Scene, EditorTab::Settings]
    }
}

pub fn save_layout(dock_state: &EditorState) {
    if let Ok(serialized) = serde_json::to_string_pretty(&dock_state.state) {
        let _ = fs::write("smq_proj/layout.json", serialized);
    }
}

pub fn load_layout() -> EditorState {
    if let Ok(contents) = fs::read_to_string("smq_proj/layout.json") {
        if let Ok(deserialized) = serde_json::from_str(&contents) {
            return EditorState { 
                state: deserialized,
                ..Default::default()
            };
        }
    }
    
    return EditorState::default();
}

pub struct EditorViewer<'a> {
    pub world: &'a mut World,

    viewportid: Option<TextureId>,

    assets: &'a mut assets::Assets,
    settings: &'a mut settings::Settings
}

impl<'a> TabViewer for EditorViewer<'a> {
    type Tab = EditorTab;

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.name().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            EditorTab::Assets => { 
                self.assets.egui(ui);
            }
            EditorTab::Inspector => { 
                ui.label("lakaka");
            }
            EditorTab::Viewport => { 
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
            EditorTab::Scene => { 
                ui.label("lakaka"); 
            }
            EditorTab::Settings => { 
                self.settings.egui(ui);
            }
        }
    }
}

#[derive(Resource)]
pub struct EditorState {
    pub state: DockState<EditorTab>,

    viewportid: Option<TextureId>,
    hovered_menu: Option<&'static str>,
    menu_popup_rect: Option<egui::Rect>,

    pub assets: assets::Assets,
    pub settings: settings::Settings
}

impl Default for EditorState {
    fn default() -> Self {
        let mut state = DockState::new(vec![EditorTab::Assets]);
        let surface = state.main_surface_mut();
        
        let [left, right] = surface.split_right(NodeIndex::root(), 0.7, vec![EditorTab::Inspector]);
        
        surface.split_below(right, 0.5, vec![EditorTab::Scene]);
        surface.split_below(left, 0.5, vec![EditorTab::Viewport, EditorTab::Settings]);

        Self { 
            state,
            viewportid: None,
            hovered_menu: None,
            menu_popup_rect: None,
            assets: assets::Assets::default(),
            settings: Settings::load()
        }
    }
}

pub fn editor_set_viewport_texture_id(world: &mut World, id: TextureId) {
    if !world.contains_resource::<EditorState>() {
        world.insert_resource(load_layout());
    }

    let mut dock_resource = world.remove_resource::<EditorState>().unwrap();

    dock_resource.viewportid = Some(id);

    world.insert_resource(dock_resource);
}

pub fn editor_update(world: &mut World) {
    if !world.contains_resource::<EditorState>() {
        world.insert_resource(load_layout());
    }

    let mut dock_resource = world.remove_resource::<EditorState>().unwrap();

    let mut viewer = EditorViewer { 
        world,
        viewportid: dock_resource.viewportid,
        assets: &mut dock_resource.assets,
        settings: &mut dock_resource.settings
    };
    
    if let Some(screen_rect) = ui().input(|i| i.raw.screen_rect) {     
        egui::Window::new("")
            .title_bar(false) 
            .movable(false) 
            .resizable(false) 
            .collapsible(false) 
            .order(egui::Order::Background) 
            .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0]) 
            .fixed_size(screen_rect.size())
            .frame(egui::Frame::default()
            .fill(ui().options(|o| { o.dark_style.visuals.panel_fill }))
            .inner_margin(0.0))
            .show(&ui(), |ui| {
            
            ui.vertical(|ui| {
                egui::Frame::default().inner_margin(egui::Margin::symmetric(8, 0)).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.style_mut().visuals.button_frame = false;
                        ui.spacing_mut().interact_size.y = 0.0;
                        ui.spacing_mut().button_padding = egui::vec2(6.0, 0.0);

                        let response = ui.add(
                            egui::Label::new(egui::RichText::new("Windows").size(12.0))
                                .sense(egui::Sense::hover())
                                .selectable(false),
                        );

                        let popup_id = egui::Id::new("windows_menu");

                        if response.hovered() {
                            dock_resource.hovered_menu = Some("Windows");
                        }

                        const HOVER_MARGIN: f32 = 20.0;
                        let pointer_pos = ui.ctx().pointer_latest_pos();
                        let hovering_popup = match (dock_resource.menu_popup_rect, pointer_pos) {
                            (Some(r), Some(p)) => r.expand(HOVER_MARGIN).contains(p),
                            _ => false,
                        };

                        if dock_resource.hovered_menu == Some("Windows") {
                            let area = egui::Area::new(popup_id)
                                .order(egui::Order::Foreground)
                                .fixed_pos(response.rect.left_bottom())
                                .show(ui.ctx(), |ui| {
                                    egui::Frame::popup(ui.style()).show(ui, |ui| {
                                        ui.set_min_width(160.0);

                                        for tab in EditorTab::all() {
                                            let path = dock_resource.state.find_tab(&tab);
                                            let mut open = path.is_some();

                                            if ui.checkbox(&mut open, tab.name()).changed() {
                                                if open {
                                                    dock_resource.state.push_to_focused_leaf(tab.clone());
                                                } else if let Some(loc) = path {
                                                    dock_resource.state.remove_tab(loc);
                                                }
                                            }
                                        }
                                    });
                                });

                            dock_resource.menu_popup_rect = Some(area.response.rect);

                            if area.response.rect.width() > 0.0 && !response.hovered() && !hovering_popup {
                                dock_resource.hovered_menu = None;
                                dock_resource.menu_popup_rect = None;
                            }
                        } else {
                            dock_resource.menu_popup_rect = None;
                        }
                    });
                });

                let space = ui.available_size();
                egui::Frame::default()
                    .inner_margin(0.0)
                    .fill(egui::Color32::from_gray(30))
                    .show(ui, |ui| {
                        ui.set_min_size(space); 
                        
                        egui_dock::DockArea::new(&mut dock_resource.state).show_inside(ui, &mut viewer);
                    });
                    
            });
            
        });
    }

    world.insert_resource(dock_resource);
}

#[derive(Serialize, Deserialize)]
pub struct CommonFileDescriptor {
    pub file_path: String,
    pub json_path: String
}

pub fn process_common_file_descriptor(path: &Path) -> (PathBuf, PathBuf) {
    let is_valid_json = match fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str::<CommonFileDescriptor>(&contents).is_ok(),
        Err(_) => false
    };

    let path_str = path.display().to_string().replace('\\', "/");
    
    let rest = if let Some(idx) = path_str.find("smq_proj/assets/") {
        &path_str[idx + "smq_proj/assets/".len()..]
    } else {
        panic!("error: filepath doesn't start with smq_proj/assets/': {:?}", path);
    };

    let new_file_path = PathBuf::from("smq_proj/data/file").join(rest);
    let new_json_path = PathBuf::from("smq_proj/data/json").join(rest);

    if !is_valid_json {
        if path.exists() {
            if let Some(parent) = new_file_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Err(_) = fs::rename(path, &new_file_path) {
                if fs::copy(path, &new_file_path).is_ok() {
                    let _ = fs::remove_file(path);
                }
            }
        }
    } else {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(descriptor) = serde_json::from_str::<CommonFileDescriptor>(&contents) {
                let new_file_path_str = new_file_path.display().to_string();
                if descriptor.file_path != new_file_path_str && Path::new(&descriptor.file_path).exists() {
                    if let Some(parent) = new_file_path.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    let _ = fs::rename(&descriptor.file_path, &new_file_path);
                }

                let new_json_path_str = new_json_path.display().to_string();
                if descriptor.json_path != new_json_path_str && Path::new(&descriptor.json_path).exists() {
                    if let Some(parent) = new_json_path.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    let _ = fs::rename(&descriptor.json_path, &new_json_path);
                }
            }
        }
    }

    let updated_descriptor = CommonFileDescriptor { 
        file_path: new_file_path.display().to_string(), 
        json_path: new_json_path.display().to_string()
    };    

    let json_string = serde_json::to_string_pretty(&updated_descriptor).unwrap();
    fs::write(path, json_string).expect("failed to save common file descriptor file");

    (new_file_path, new_json_path)
}
