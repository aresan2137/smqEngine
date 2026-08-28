use bevy_ecs::prelude::*;
use glam::*;

use winit::{event::{Event, WindowEvent}, keyboard::{PhysicalKey}};

use smq_engine::*;

mod components;
use components::*;

mod editor;
use editor::*;

fn main() {
    pollster::block_on(run());
}

#[allow(deprecated)]
pub async fn run() {
    let _ = logger::init();

    let (mut context, event_loop) = Context::new().await;

    let mut world = World::new();
    world.insert_resource(Delta::new(&context.window));
    world.insert_resource(InputState::default());

    let mut is_minimized = false;

    event_loop.run(move |event, elwt| {
        match event {
            winit::event::Event::DeviceEvent { event: winit::event::DeviceEvent::MouseMotion { delta }, .. } => {
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
                            world.get_resource_mut::<InputState>().expect("input not found").on_input(event, keycode, &context.window);
                        }
                    }
                    WindowEvent::CloseRequested => {           
                        save_editor_layout(&world);

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


                        editor_update(&mut world);

                        let full_output = context.end_egui_record();

                        if is_minimized {
                            return; 
                        }

                        if let Some(mut frame) = context.start_frame() {
                                
                            context.draw_egui(&mut frame, full_output);

                            context.end_frame(frame); 
                        }
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