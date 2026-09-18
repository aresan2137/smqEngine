use bevy_ecs::prelude::*;
use glam::*;

use smq_engine::*;

use wgpu::*;

mod gen_bake;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoop;

use crate::{gen_bake::Meshes, renderer::{DrawedObject, PointLight, Renderer, load_assets, render}};

mod gen_game;

mod renderer;

pub fn fps_logger(time: Res<Delta>) {
    egui::Window::new("FPS").default_width(400.0).show(&ui(), |ui| {
        ui.label(format!("fps: {}", 1.0/time.delta));
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
#[allow(unused)]
fn main() {
    logger::init();
    
    let mut world = World::new();
    world.insert_resource(Delta::new());
    world.insert_resource(Inputs::default());

    let mut schedule = Schedule::default();
    schedule.add_systems(free_camera_system);
    schedule.add_systems(fps_logger);

    world.spawn(
        FreeCamera {
            position: Vec3::ZERO,
            pitch: 0.0,
            yaw: 0.0,
            speed: 5.0,
            sensitivity: 0.3,
            is_controlling: false
        }
    );

    world.spawn(PointLight {
        position: Vec3 { x: -5.0, y: 1.0, z: 1.0 },
        color: Vec3::ONE,
        power: 5.0
    });

    world.spawn(PointLight {
        position: Vec3 { x: -0.5, y: 1.0, z: -15.0 },
        color: Vec3::new(0.3, 0.9, 0.4),
        power: 5.0
    });

    world.spawn(DrawedObject {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        size: Vec3::ONE,
        meshid: Meshes::file_mesh_Floor_smf
    });

    world.spawn(DrawedObject {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        size: Vec3::ONE,
        meshid: Meshes::file_mesh_Walls_smf
    });

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut context: Context<'_, Renderer> = Context::new(world, schedule, event_loop.create_proxy(), ContextSettings { 
        present_mode: PresentMode::AutoNoVsync
    }, ContextEvents { 
        renderer: Some(render), 
        on_wgpu_load: Some(load_assets), 
        pre_schedule: None,
        on_exit: None
    });

    event_loop.run_app(&mut context).unwrap();
}
