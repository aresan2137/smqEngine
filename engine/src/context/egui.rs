use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use egui_wgpu::ScreenDescriptor;
use wgpu::*;

use crate::{Context, FrameInfo};

pub fn ui() -> egui::Context {
    static CTX: OnceLock<egui::Context> = OnceLock::new();
    CTX.get_or_init(|| egui::Context::default()).clone()
}

pub fn drop_point(
    ui: &mut egui::Ui,
    existing: Option<String>,
    pre: impl FnOnce(&mut egui::Ui),
    post: impl FnOnce(&mut egui::Ui, Option<Arc<PathBuf>>),
) {
    ui.horizontal(|ui| {
        pre(ui);

        let (drop_rect, drop_response) =
            ui.allocate_exact_size(egui::vec2(200.0, 30.0), egui::Sense::hover());

        let bg_color = if drop_response.dnd_hover_payload::<PathBuf>().is_some() {
            egui::Color32::from_rgb(100, 150, 200)
        } else {
            egui::Color32::from_rgb(50, 50, 50)
        };

        ui.painter().rect_filled(drop_rect, 4.0, bg_color);
        ui.painter().rect_stroke(
            drop_rect,
            4.0,
            egui::Stroke::new(1.0, egui::Color32::GRAY),
            egui::StrokeKind::Inside,
        );

        ui.put(
            drop_rect,
            egui::Label::new(if let Some(var) = existing {
                format!("file:{var}")
            } else {
                "drop file here".to_string()
            })
            .selectable(false),
        );

        post(ui, drop_response.dnd_release_payload::<PathBuf>());
    });
}

pub struct EguiDrawData {
    textures_delta: egui::TexturesDelta,
    paint_jobs: Vec<egui::ClippedPrimitive>,
    screen_descriptor: ScreenDescriptor
}
impl<D> Context<'_, D> {
    pub fn start_egui_record(&mut self) {
        let raw_input = self
            .holding
            .as_mut()
            .unwrap()
            .egui_state
            .take_egui_input(self.window.as_ref().unwrap());
        ui().begin_pass(raw_input);
    }

    pub fn end_egui_record(&mut self) -> egui::FullOutput {
        let full_output = ui().end_pass();

        let platform_output = full_output.platform_output.clone();
        self.holding
            .as_mut()
            .unwrap()
            .egui_state
            .handle_platform_output(self.window.as_ref().unwrap(), platform_output);

        return full_output;
    }

    pub fn draw_egui_with_own_pass(&mut self, info: &mut FrameInfo, full_output: egui::FullOutput) {
        let data = self.draw_egui_prepare(info, full_output);
        {
            let mut ui_pass = info
                .encoder
                .begin_render_pass(&RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &info.view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: Operations {
                            load: LoadOp::Load,
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                })
                .forget_lifetime();

            self.draw_egui_in_pass(&data, &mut ui_pass);
        }

        self.draw_egui_post_pass(data);
    }

    pub fn draw_egui_prepare(
        &mut self,
        info: &mut FrameInfo,
        full_output: egui::FullOutput
    ) -> EguiDrawData {
        let holding = self.holding.as_mut().unwrap();
        let paint_jobs = ui().tessellate(full_output.shapes, 1.0);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [holding.surface_config.width, holding.surface_config.height],
            pixels_per_point: 1.0,
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            for delta in image_delta {
                holding
                    .egui_renderer
                    .update_texture(&holding.device, &holding.queue, *id, delta);
            }
        }

        holding.egui_renderer.update_buffers(
            &holding.device,
            &holding.queue,
            &mut info.encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        return EguiDrawData {
            textures_delta: full_output.textures_delta,
            paint_jobs,
            screen_descriptor,
        };
    }

    pub fn draw_egui_in_pass(&self, data: &EguiDrawData, pass: &mut RenderPass<'static>) {
        let holding = self.holding.as_ref().unwrap();
        holding
            .egui_renderer
            .render(pass, &data.paint_jobs[..], &data.screen_descriptor);
    }

    pub fn draw_egui_post_pass(&mut self, mut data: EguiDrawData) {
        let holding = self.holding.as_mut().unwrap();
        for id in &data.textures_delta.free {
            holding.egui_renderer.free_texture(id);
        }
        data.textures_delta.clear();
    }
}
