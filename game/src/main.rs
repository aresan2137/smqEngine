use bevy_ecs::prelude::*;
use glam::*;

use wgpu::Color;
use winit::{event::{Event, WindowEvent}, keyboard::{PhysicalKey}};

use smq_engine::*;

mod components;
use components::*;

mod code;
use code::*;

mod map;

#[allow(unused)]
fn main() {
    pollster::block_on(run());
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{code::free_cam::f11_system, map::save_aabbs};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshID {
    Wall = 0,
    Floor = 1,
    Cube = 2,
    Cart = 3
}

pub fn fps_logger(time: Res<Delta>) {
    egui::Window::new("FPS")
        .default_width(400.0)
        .show(&ui(), |ui| {
            ui.label(format!("fps: {}", 1.0/time.delta));
        });

}

#[allow(deprecated)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let _ = logger::init();

    let (mut context, event_loop) = Context::new().await;
    
    let mut world = World::new();
    world.insert_resource(Delta::new(&context.window));
    world.insert_resource(InputState::default());

    let mut schedule = Schedule::default();
    schedule.add_systems(free_cam::free_camera_system);
    //schedule.add_systems(movment::player_movment_system);
    schedule.add_systems(movment::move_player_light);
    schedule.add_systems(fps_logger);

    cart_system::cart_system_start(&mut world, &mut schedule);

    map::create_map(&mut world, &mut schedule);

    world.spawn((free_cam::FreeCamera {
        position: Vec3::new(0.0, 1.2, 0.0),
        pitch: 0.0,
        yaw: 0.0,
        speed: 6.0, // 2.0
        sensitivity: 0.3,
        is_controlling: false,
    }
    , PointLight {
        position: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0},
        power: 50.0 //1.0
    }
));

    let mut assets = renderer::Assets::new(&context, vec![
        include_bytes!("../../smq_proj/assets/objs/Walls.smf") as &[u8],
        include_bytes!("../../smq_proj/assets/objs/Floor.smf") as &[u8],
        include_bytes!("../../smq_proj/assets/objs/Cube.smf") as &[u8],
        include_bytes!("../../smq_proj/assets/objs/Cart.smf") as &[u8]
    ]);

    event_loop.run(move |event, elwt| {
        match event {
            Event::DeviceEvent { event: winit::event::DeviceEvent::MouseMotion { delta }, .. } => {
                let mut input = world.resource_mut::<InputState>();
                input.mouse_delta.0 += delta.0 as f32;
                input.mouse_delta.1 += delta.1 as f32;
            }        
            Event::WindowEvent { event: ref event_window, window_id } if window_id == context.window.id() => {
                let response = context.egui_state.on_window_event(&context.window, event_window);
        
                if response.consumed {
                    return; 
                }

                match event_window {
                    WindowEvent::KeyboardInput { event, .. } => {
                        if let PhysicalKey::Code(keycode) = event.physical_key {
                            world.get_resource_mut::<InputState>().expect("input not found").on_input(event, keycode);
                        }

                        f11_system(&world, &context.window);
                    }
                    WindowEvent::CloseRequested => {
                        save_aabbs(&mut world);

                        elwt.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        if physical_size.width == 0 || physical_size.height == 0 {
                            assets.is_minimized = true;
                        } else {
                            assets.is_minimized = false;
                            context.resize(*physical_size);
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        context.start_egui_record();

                        world.get_resource_mut::<Delta>().expect("delta not found").update_delta();

                        schedule.run(&mut world);

                        assets.draw(&mut context, &mut world);

                        world.get_resource_mut::<InputState>().expect("input not found").end_frame();
                    }
                    _ => {}
                }
            }
            winit::event::Event::AboutToWait => {
                context.window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}