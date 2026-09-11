use glam::*;
use bevy_ecs::prelude::*;
use std::{collections::HashSet};
use winit::{event::KeyEvent, keyboard::{KeyCode}};
use web_time::Instant;

#[derive(Resource)]
pub struct Delta {
    pub delta: f32,
    pub last_render_time: Instant
}

impl Delta {
    pub fn new() -> Self {
        let last_render_time = Instant::now();

        return Self {
            delta: 0.0,
            last_render_time
        };        
    }

    pub fn update_delta(&mut self) {
        let now = Instant::now();
        self.delta = now.duration_since(self.last_render_time).as_secs_f32();
        self.last_render_time = now;
    }
}

#[derive(Resource, Default)]
pub struct Inputs {
    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,

    pub mouse_delta: (f32, f32)
}

impl Inputs {
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