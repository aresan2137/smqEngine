use bevy_ecs::prelude::*;
use glam::*;
use std::collections::HashSet;
use web_time::Instant;
use winit::{event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta}, keyboard::KeyCode};

#[derive(Resource)]
pub struct Delta {
    pub delta: f32,
    pub last_render_time: Instant,
}

impl Delta {
    pub fn new() -> Self {
        let last_render_time = Instant::now();

        return Self {
            delta: 0.0,
            last_render_time,
        };
    }

    pub fn update_delta(&mut self) {
        let now = Instant::now();
        self.delta = now.duration_since(self.last_render_time).as_secs_f32();
        self.last_render_time = now;
    }
}

#[derive(Resource)]
pub struct Inputs {
    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,

    mouse_down: HashSet<MouseButton>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,

    pub scroll_delta: Vec2,
    pub mouse_delta: Vec2,
    pub mouse_pos: Vec2,

    pub window_size: UVec2
}

impl Default for Inputs {
    fn default() -> Self {
        return Self { 
            keys_down: HashSet::new(), 
            keys_pressed: HashSet::new(), 
            keys_released: HashSet::new(), 
            mouse_down: HashSet::new(), 
            mouse_pressed: HashSet::new(), 
            mouse_released: HashSet::new(), 
            scroll_delta: Vec2::ZERO, 
            mouse_delta: Vec2::ZERO, 
            mouse_pos: Vec2::ZERO,
            window_size: UVec2::new(1280, 720) 
        };
    }
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

    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_down.contains(&button)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }

    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_released.contains(&button)
    }

    pub fn on_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        match state {
            ElementState::Pressed => {
                if !self.mouse_down.contains(&button) {
                    self.mouse_pressed.insert(button);
                }
                self.mouse_down.insert(button);
            }
            ElementState::Released => {
                self.mouse_down.remove(&button);
                self.mouse_released.insert(button);
            }
        }
    }

    pub fn on_mouse_wheel(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(x, y) => {
                self.scroll_delta.x += x;
                self.scroll_delta.y += y;
            }
            MouseScrollDelta::PixelDelta(pos) => {
                self.scroll_delta.x += pos.x as f32;
                self.scroll_delta.y += pos.y as f32;
            }
        }
    }

    pub fn end_frame(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        
        self.mouse_pressed.clear();
        self.mouse_released.clear();
        
        self.scroll_delta = Vec2::ZERO;
        self.mouse_delta = Vec2::ZERO;
    }
}
