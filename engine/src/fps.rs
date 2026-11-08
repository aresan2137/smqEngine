use std::collections::VecDeque;

use bevy_ecs::prelude::*;

use crate::*;

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
        if n == 0 {
            return;
        }

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

    egui::Window::new("FPS")
        .default_width(400.0)
        .show(&ui(), |ui| {
            ui.label(format!("fps: {:.1}", 1.0 / time.delta));
            ui.label(format!("avg: {:.1}", stats.avg_fps));
            ui.label(format!("1% low: {:.1}", stats.low_1));
            ui.label(format!("0.1% low: {:.1}", stats.low_01));
        });
}
