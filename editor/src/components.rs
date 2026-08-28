use glam::*;
use bevy_ecs::prelude::*;
use std::{collections::HashSet, time::Duration};
use winit::{event::KeyEvent, keyboard::{KeyCode}, window::Window};
use web_time::Instant;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Resource)]
pub struct Delta {
    pub delta: f32,
    pub target_frame_duration: Duration,
    pub last_render_time: Instant,
    pub next_frame_time: Instant
}

#[cfg(target_arch = "wasm32")]
#[derive(Resource)]
pub struct Delta {
    pub delta: f32,
    pub last_render_time: Instant
}

impl Delta {
    pub fn new(window: &Window) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let target_fps = window.current_monitor()
                .and_then(|monitor| monitor.refresh_rate_millihertz())
                .map(|mhz| mhz as f64 / 1000.0)
                .unwrap_or(60.0);
            
            let target_frame_duration = std::time::Duration::from_secs_f64(1.0 / target_fps);

            
            let next_frame_time = Instant::now();

            let last_render_time = Instant::now();

            return Self {
                delta: 0.0,
                target_frame_duration,
                last_render_time,
                next_frame_time
            };
        }

        #[cfg(target_arch = "wasm32")] {
            let last_render_time = Instant::now();

            return Self {
                delta: 0.0,
                last_render_time
            };
        }
    }

    pub fn update_delta(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let current_time = Instant::now();
            if current_time < self.next_frame_time {
                std::thread::sleep(self.next_frame_time - current_time);
            }
            self.next_frame_time = Instant::now() + self.target_frame_duration;
        }    

        let now = Instant::now();
        self.delta = now.duration_since(self.last_render_time).as_secs_f32();
        self.last_render_time = now;
    }
}

#[derive(Resource, Default)]
pub struct InputState {
    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,

    pub mouse_delta: (f32, f32)
}

#[allow(unused)]
impl InputState {
    pub fn down(&self, keycode: KeyCode) -> bool {
        self.keys_down.contains(&keycode)
    }

    pub fn pressed(&self, keycode: KeyCode) -> bool {
        self.keys_pressed.contains(&keycode)
    }

    pub fn released(&self, keycode: KeyCode) -> bool {
        self.keys_released.contains(&keycode)
    }

    pub fn on_input(&mut self, event: &KeyEvent, keycode: KeyCode) {
        match event.state {
            winit::event::ElementState::Pressed => {
                if !event.repeat {
                    self.keys_pressed.insert(keycode);
                }
                self.keys_down.insert(keycode);
            }
            winit::event::ElementState::Released => {
                self.keys_down.remove(&keycode);
                self.keys_released.insert(keycode);
            }
        }
    }

    pub fn end_frame(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_delta = (0.0, 0.0);
    }
}