use bevy_ecs::prelude::*;
use glam::{Quat, Vec3};
use smq_engine::FreeCamera;

use crate::{
    gen_bake::{Meshes, Textures},
    renderer::TransparentObject,
};

#[derive(Resource)]
pub struct GameState {
    collected_coal: u32,
    has_coal: bool,
}

impl Default for GameState {
    fn default() -> Self {
        return Self {
            collected_coal: 0,
            has_coal: false,
        };
    }
}

#[derive(PartialEq, Eq)]
pub enum DropPointMode {
    Load,
    Unload,
}

#[derive(Component)]
pub struct DropPoint {
    mode: DropPointMode,
    uses: i32,
    min: Vec3,
    max: Vec3,
}

impl DropPoint {
    pub fn from_position_size(
        position: Vec3,
        size: Vec3,
        mode: DropPointMode,
        uses: i32,
    ) -> (Self, TransparentObject) {
        return (
            Self {
                mode,
                uses,
                min: Vec3::new(
                    position.x - (size.x / 2.0),
                    position.x - (size.x / 2.0),
                    position.x - (size.x / 2.0),
                ),
                max: Vec3::new(
                    position.x + (size.x / 2.0),
                    position.x + (size.x / 2.0),
                    position.x + (size.x / 2.0),
                ),
            },
            TransparentObject {
                position,
                rotation: Quat::IDENTITY,
                size,
                meshid: Meshes::mesh_Cube_smf,
                textureid: Textures::rock_png,
            },
        );
    }

    pub fn is_point_inside(&self, point: &Vec3) -> bool {
        return point.x > self.min.x
            && point.x < self.max.x
            && point.y > self.min.y
            && point.y < self.max.y
            && point.z > self.min.z
            && point.z < self.max.z;
    }
}

pub fn drop_point_system(
    mut points: Query<&mut DropPoint>,
    player: Query<&FreeCamera>,
    mut state: ResMut<GameState>,
) {
    if let Some(player) = player.iter().next() {
        for point in points.iter_mut() {
            if point.mode == DropPointMode::Load {
                if !state.has_coal {
                    if point.is_point_inside(&player.position) {
                        state.has_coal = true;
                    }
                }
            } else {
                if state.has_coal {
                    if point.is_point_inside(&player.position) {
                        state.has_coal = false;
                        state.collected_coal += 1;
                    }
                }
            }
        }
    }
}


