use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::*;
use winit::{keyboard::KeyCode, window::Window};

use crate::components::*;

#[derive(Component)]
pub struct FreeCamera {
    pub position: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub speed: f32,
    pub sensitivity: f32,
    pub is_controlling: bool
}

#[allow(dead_code)]
pub fn free_camera_system(time: Res<Delta>, input: Res<InputState>, mut query: Query<&mut FreeCamera>) {
    for mut cam in query.iter_mut() {
        
        if input.pressed(KeyCode::KeyZ) {
            cam.is_controlling = !cam.is_controlling;
        }
        if input.pressed(KeyCode::Escape) {
            cam.is_controlling = false;
        }

        if !cam.is_controlling {
            continue; 
        }

        cam.yaw -= input.mouse_delta.0 * cam.sensitivity;
        cam.pitch -= input.mouse_delta.1 * cam.sensitivity;
        cam.pitch = cam.pitch.clamp(-89.0, 89.0);

        let q_yaw = Quat::from_axis_angle(Vec3::Y, cam.yaw.to_radians());
        let q_pitch = Quat::from_axis_angle(Vec3::X, cam.pitch.to_radians());
        let rotation = (q_yaw * q_pitch).normalize();

        let forward = rotation * Vec3::new(0.0, 0.0, -1.0);
        let right = rotation * Vec3::new(1.0, 0.0, 0.0);
        let up = rotation * Vec3::new(0.0, 1.0, 0.0);

        let velocity = cam.speed * time.delta;

        if input.down(KeyCode::KeyW) { cam.position += forward * velocity; }
        if input.down(KeyCode::KeyS) { cam.position -= forward * velocity; }
        if input.down(KeyCode::KeyD) { cam.position += right * velocity; }
        if input.down(KeyCode::KeyA) { cam.position -= right * velocity; }
        if input.down(KeyCode::KeyE) { cam.position += up * velocity; }
        if input.down(KeyCode::KeyQ) { cam.position -= up * velocity; }
    }
}

pub fn f11_system(world: &World, window: &Arc<Window>) {
    let input = world.get_resource::<InputState>().expect("input not found");

    if input.pressed(KeyCode::F11) {
        if window.fullscreen().is_some() {
            window.set_fullscreen(None);
        } else {
            window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        }
    }
}