//#![windows_subsystem = "windows"]

use bevy_ecs::prelude::*;
use egui::TextureFilter;
use glam::*;

use wgpu::FilterMode;
use winit::{event::{Event, WindowEvent}, keyboard::{PhysicalKey}};

mod components;
use components::*;

use smq_engine::*;

use crate::{editor::{editor_set_viewport_texture_id, editor_update, save_editor_layout}, free_cam::{FreeCamera, free_camera_system}};

mod editor;
mod bake;
mod renderer;
pub mod gen_bake;
pub mod free_cam;

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

    let mut schedule = Schedule::default();
    schedule.add_systems(free_camera_system);

    let mut is_minimized = false;

    let mut renderer = renderer::Renderer::new(&mut context);

    world.spawn(FreeCamera {
        position: Vec3::ZERO,
        pitch: 0.0,
        yaw: 0.0,
        speed: 5.0,
        sensitivity: 0.3,
        is_controlling: false
    });

    editor_set_viewport_texture_id(&mut world, renderer.gen_assets.renderer_main_renderTexture.create_egui_texture_id(&mut context, 0, FilterMode::Linear));

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
                            world.get_resource_mut::<InputState>().expect("input not found").on_input(event, keycode);
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

                        schedule.run(&mut world);

                        editor_update(&mut world);

                        let full_output = context.end_egui_record();

                        if is_minimized {
                            return; 
                        }

                        renderer.draw(&mut context, &mut world, full_output);

                        world.get_resource_mut::<InputState>().expect("input nie istnieje").end_frame();                        
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