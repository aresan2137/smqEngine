use std::collections::VecDeque;

use bevy_ecs::prelude::*;
use glam::*;

use kira::{AudioManager, AudioManagerSettings, DefaultBackend, sound::static_sound::{StaticSoundData, StaticSoundSettings}};
use smq_engine::*;

use wgpu::*;

mod gen_bake;

mod code;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoop;

use crate::{code::*, gen_bake::Meshes, renderer::{DrawedObject, PointLight, Renderer, load_assets, render}};

mod gen_game;

mod renderer;

const HISTORY: usize = 1000;

#[derive(Resource)]
pub struct FrameStats {
    history: VecDeque<f32>,
    avg_fps: f32,
    low_1: f32,
    low_01: f32,
}

impl Default for FrameStats {
    fn default() -> Self {
        Self {
            history: VecDeque::with_capacity(HISTORY),
            avg_fps: 0.0,
            low_1: 0.0,
            low_01: 0.0,
        }
    }
}

impl FrameStats {
    pub fn push(&mut self, dt_secs: f32) {
        if self.history.len() == HISTORY {
            self.history.pop_front();
        }
        self.history.push_back(dt_secs);

        let mut sorted: Vec<f32> = self.history.iter().copied().collect();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let n = sorted.len();
        if n == 0 { return; }

        let idx_1 = (n as f32 * 0.01).ceil() as usize;
        let idx_01 = (n as f32 * 0.001).ceil() as usize;

        let worst_1: f32 = sorted[..idx_1.max(1)].iter().sum::<f32>() / idx_1.max(1) as f32;
        let worst_01: f32 = sorted[..idx_01.max(1)].iter().sum::<f32>() / idx_01.max(1) as f32;

        self.low_1 = 1.0 / worst_1;
        self.low_01 = 1.0 / worst_01;
        self.avg_fps = 1.0 / (sorted.iter().sum::<f32>() / n as f32);
    }
}

pub fn fps_logger(time: Res<Delta>, mut stats: ResMut<FrameStats>) {
    stats.push(time.delta);

    egui::Window::new("FPS").default_width(400.0).show(&ui(), |ui| {
        ui.label(format!("fps: {:.1}", 1.0 / time.delta));
        ui.label(format!("avg: {:.1}", stats.avg_fps));
        ui.label(format!("1% low: {:.1}", stats.low_1));
        ui.label(format!("0.1% low: {:.1}", stats.low_01));
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
#[allow(unused)]
fn main() {
    logger::init();

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

    let sound_data = StaticSoundData::from_file("smq_proj/data/file/audio/music.mp3").unwrap();

    manager.play(sound_data);
    
    let mut world = World::new();
    world.insert_resource(Delta::new());
    world.insert_resource(Inputs::default());
    world.insert_resource(FrameStats::default());

    let mut schedule = Schedule::default();
    schedule.add_systems(player_movment_system);
    schedule.add_systems(move_player_light);
    schedule.add_systems(fps_logger);

    world.spawn((
        FreeCamera {
            position: Vec3::new(0.0, 1.2, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            speed: 2.0,
            sensitivity: 0.3,
            is_controlling: false
        },
        PointLight {
            position: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            color: Vec3::ONE,
            power: 0.05
        }
    ));

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
        present_mode: PresentMode::AutoVsync
    }, ContextEvents { 
        renderer: Some(render), 
        on_wgpu_load: Some(load_assets), 
        pre_schedule: None,
        on_exit: None
    });

    event_loop.run_app(&mut context).unwrap();
}
