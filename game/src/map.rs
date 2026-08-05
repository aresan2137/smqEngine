use bevy_ecs::prelude::*;
use glam::*;
use std::fs;

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Component, Clone, Debug, Deserialize, Serialize)]
pub struct MapAABB {
    pub position_min: Vec3,
    pub position_max: Vec3
}

impl MapAABB {

    // pub fn from_position_size(position: Vec3, size: Vec3) -> Self {
    //     Self { 
    //         position_min: position - size / 2.0, 
    //         position_max: position + size / 2.0
    //     }
    // }

    pub fn from_position_size(position: Vec3, size: Vec3) -> (Self, Position3D) {
        (Self { 
            position_min: position - size / 2.0, 
            position_max: position + size / 2.0
        }, Position3D {
            position,
            rotation: Quat::IDENTITY,
            size,
            mesh: MeshID::Cube
        })
    }
}

pub fn create_map(world: &mut World, schedule: &mut Schedule) {
    world.spawn(Position3D {
        position: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        rotation: Quat::IDENTITY,
        size: Vec3::ONE,
        mesh: MeshID::Wall
    });

    world.spawn(Position3D {
        position: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        rotation: Quat::IDENTITY,
        size: Vec3::ONE,
        mesh: MeshID::Floor
    });

    world.spawn(PointLight {
        position: Vec3 { x: -4.75, y: 1.8, z: 1.25 },
        color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0},
        power: 8.0
    });

    world.spawn(PointLight {
        position: Vec3 { x: -0.65, y: 1.8, z: -15.5 },
        color: Color { r: 56.0/255.0, g: 1.0, b: 109.0/255.0, a: 1.0},
        power: 12.0
    });

    load_aabbs(world);

    schedule.add_systems(editor_system);
}

#[derive(Resource, Default)]
pub struct AABBEditor {
    pub selected: Option<Entity>,
}

#[derive(Serialize, Deserialize)]
struct AABBFile {
    aabbs: Vec<MapAABB>,
}

pub fn editor_system(mut editor: ResMut<AABBEditor>, mut aabb_query: Query<(Entity, &mut MapAABB, &mut Position3D)>, mut commands: Commands) {
    egui::Window::new("AABB Editor")
        .default_width(400.0)
        .show(&ui(), |ui| {

            ui.heading("AABBs");
            ui.separator();

            if ui.button("add").clicked() {
                commands.spawn(MapAABB::from_position_size(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)));
            }

            let aabbs_info: Vec<(Entity, Vec3, Vec3)> = aabb_query.iter().map(|(entity, aabb, _)| {
                let position = (aabb.position_min + aabb.position_max) / 2.0;
                let size = aabb.position_max - aabb.position_min;
                (entity, position, size)
            }).collect();

            for (index, (entity, position, size)) in aabbs_info.iter().enumerate() {
                let selected = editor.selected == Some(*entity);
                let text = format!("AABB {}  |  Pos: {:.1}, {:.1}, {:.1}  |  Size: {:.1}, {:.1}, {:.1}", index, position.x, position.y, position.z, size.x, size.y, size.z);

                if ui.selectable_label(selected, text).clicked() {
                    editor.selected = Some(*entity);
                }
            }

            ui.separator();

            let Some(selected_entity) = editor.selected else {
                ui.label("No AABB selected.");
                return;
            };

            let Ok((_, mut aabb, _)) = aabb_query.get_mut(selected_entity) else {
                editor.selected = None;
                return;
            };

            let mut position = (aabb.position_min + aabb.position_max) / 2.0;
            let mut size = aabb.position_max - aabb.position_min;

            ui.label("Position");
            ui.horizontal(|ui| {
                ui.label("X");
                ui.add(egui::DragValue::new(&mut position.x).speed(0.1));
                ui.label("Y");
                ui.add(egui::DragValue::new(&mut position.y).speed(0.1));
                ui.label("Z");
                ui.add(egui::DragValue::new(&mut position.z).speed(0.1));
            });

            ui.label("Size");
            ui.horizontal(|ui| {
                ui.label("X");
                ui.add(egui::DragValue::new(&mut size.x).speed(0.1).range(0.01..=1000.0));
                ui.label("Y");
                ui.add(egui::DragValue::new(&mut size.y).speed(0.1).range(0.01..=1000.0));
                ui.label("Z");
                ui.add(egui::DragValue::new(&mut size.z).speed(0.1).range(0.01..=1000.0));
            });

            aabb.position_min = position - size / 2.0;
            aabb.position_max = position + size / 2.0;
        });

    for (entity, aabb, mut transform) in aabb_query.iter_mut() {
        let position = (aabb.position_min + aabb.position_max) / 2.0;
        let size = aabb.position_max - aabb.position_min;

        transform.size = size;

        if Some(entity) == editor.selected {
            transform.position = position + Vec3::Y;
        } else {
            transform.position = position;
        }
    }
}

pub fn save_aabbs(world: &mut World) {
    let mut query = world.query::<&MapAABB>();
    let aabbs: Vec<MapAABB> = query.iter(world).cloned().collect();
    let file = AABBFile { aabbs };
    
    let json = serde_json::to_string_pretty(&file).expect("failed to process to string");
    
    fs::write("aabbs.json", json).expect("failed to write AABBs to file"); 
}

pub fn load_aabbs(world: &mut World) {
    world.init_resource::<AABBEditor>();

    let json = match fs::read_to_string("aabbs.json") {
        Ok(content) => content,
        Err(e) => {
            return;
        }
    };

    let file: AABBFile = match serde_json::from_str(&json) {
        Ok(data) => data,
        Err(e) => {
            return; 
        }
    };

    let entities: Vec<Entity> = {
        let mut query = world.query_filtered::<Entity, With<MapAABB>>();
        query.iter(world).collect()
    };

    for entity in entities {
        world.despawn(entity);
    }

    for aabb in file.aabbs {
        let position = (aabb.position_min + aabb.position_max) / 2.0;
        let size = aabb.position_max - aabb.position_min;

        world.spawn((
            aabb,
            Position3D {
                position,
                rotation: Quat::IDENTITY,
                size,
                mesh: MeshID::Cube
            },
        ));
    }
}