use glam::*;
use bevy_ecs::prelude::*;
use std::{collections::HashSet, time::Duration};
use winit::{event::KeyEvent, keyboard::{KeyCode}, window::Window};
use web_time::Instant;


#[derive(Resource)]
pub struct Delta {
    pub delta: f32,
    pub target_frame_duration: Duration,
    pub last_render_time: Instant,
    pub next_frame_time: Instant
}

impl Delta {
    pub fn new(window: &Window) -> Self {
        let target_fps = window.current_monitor()
            .and_then(|monitor| monitor.refresh_rate_millihertz())
            .map(|mhz| mhz as f64 / 1000.0)
            .unwrap_or(60.0);
        
        let target_frame_duration = std::time::Duration::from_secs_f64(1.0 / target_fps);

        let last_render_time = Instant::now();
        let next_frame_time = Instant::now();

        return Self {
            delta: 0.0,
            target_frame_duration,
            last_render_time,
            next_frame_time
        };
    }

    pub fn update_delta(&mut self) {
        let current_time = Instant::now();
        if current_time < self.next_frame_time {
            std::thread::sleep(self.next_frame_time - current_time);
        }
        self.next_frame_time = Instant::now() + self.target_frame_duration;

        let now = Instant::now();
        self.delta = now.duration_since(self.last_render_time).as_secs_f32();
        self.last_render_time = now;
    }
}

#[derive(Resource, Default)]
pub struct InputState {
    pub keys_pressed: HashSet<KeyCode>,
    pub mouse_delta: (f32, f32),
    pub controlling: bool
}

impl InputState {
    pub fn on_input(&mut self, event: &KeyEvent, keycode: KeyCode, window: &Window) {
        if event.state == winit::event::ElementState::Pressed {
            self.keys_pressed.insert(keycode);

            if !event.repeat {
                if keycode == winit::keyboard::KeyCode::KeyZ {
                    self.controlling = !self.controlling;
                    window.set_cursor_visible(!self.controlling);
                    
                    let grab_mode = if self.controlling {
                        winit::window::CursorGrabMode::Confined
                    } else {
                        winit::window::CursorGrabMode::None
                    };
                    let _ = window.set_cursor_grab(grab_mode);
                }
                else if keycode == KeyCode::Escape {
                    self.controlling = false;
                    window.set_cursor_visible(true);
                    let _ = window.set_cursor_grab(winit::window::CursorGrabMode::None);
                }
            }
        } else {
            self.keys_pressed.remove(&keycode);
        }
    }
}