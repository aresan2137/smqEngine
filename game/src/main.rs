use bevy_ecs::prelude::*;
use glam::*;

use winit::{event::{Event, WindowEvent}, keyboard::{PhysicalKey}};

use smq_engine::*;

mod components;
use components::*;

mod code;
use code::*;

mod renderer;

mod gen_bake;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use renderer::*;

pub fn fps_logger(time: Res<Delta>) {
    egui::Window::new("FPS").default_width(400.0).show(&ui(), |ui| {
        ui.label(format!("fps: {}", 1.0/time.delta));
    });
}

#[allow(unused)]
fn main() {
    pollster::block_on(run());
}

#[allow(deprecated)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    logger::init();

    let (mut context, event_loop) = Context::new().await;
    
    let mut world = World::new();
    world.insert_resource(Delta::new(&context.window));
    world.insert_resource(InputState::default());

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

    let mut is_minimized = false;

    let mut renderer = Renderer::new(&mut context);

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
                        elwt.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        if physical_size.width == 0 || physical_size.height == 0 {
                            is_minimized = true;
                        } else {
                            is_minimized = false;
                            context.resize(*physical_size);
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        context.start_egui_record();

                        world.get_resource_mut::<Delta>().expect("delta not found").update_delta();

                        schedule.run(&mut world);

                        let full_output = context.end_egui_record();

                        if is_minimized { return; }

                        renderer.draw(&mut context, &mut world, full_output);

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