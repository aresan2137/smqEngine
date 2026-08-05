use bevy_ecs::prelude::*;
use glam::*;
use winit::keyboard::KeyCode;

use crate::{code::free_cam, components::*};

pub fn player_movment_system(time: Res<Delta>, input: Res<InputState>, mut query: Query<&mut free_cam::FreeCamera>) {
    if let Some(mut player) = query.iter_mut().next() {
        
        if input.pressed(KeyCode::KeyZ) {
            player.is_controlling = !player.is_controlling;
        }
        if input.pressed(KeyCode::Escape) {
            player.is_controlling = false;
        }

        if !player.is_controlling {
            return; 
        }

        player.yaw -= input.mouse_delta.0 * player.sensitivity;
        player.pitch -= input.mouse_delta.1 * player.sensitivity;
        player.pitch = player.pitch.clamp(-85.0, 85.0);

        let q_yaw = Quat::from_axis_angle(Vec3::Y, player.yaw.to_radians());
        let rotation = q_yaw.normalize();

        let forward = rotation * Vec3::new(0.0, 0.0, -1.0);
        let right = rotation * Vec3::new(1.0, 0.0, 0.0);

        let velocity = player.speed * time.delta;

        if input.down(KeyCode::KeyW) { player.position += forward * velocity; }
        if input.down(KeyCode::KeyS) { player.position -= forward * velocity; }
        if input.down(KeyCode::KeyD) { player.position += right * velocity; }
        if input.down(KeyCode::KeyA) { player.position -= right * velocity; }
    }
}

pub fn move_player_light(mut query: Query<(&free_cam::FreeCamera, &mut PointLight)>) {
    if let Some((player, mut point_light)) = query.iter_mut().next() {
        let q_yaw = Quat::from_axis_angle(Vec3::Y, player.yaw.to_radians());
        let rotation = q_yaw.normalize();

        let forward = rotation * Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);

        let offset = forward * 0.2 + up * 0.2;

        point_light.position = player.position + offset;
    }
}