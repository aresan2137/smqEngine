use wgpu::*;

use crate::{Context, FrameInfo, ui};

impl Context<'_> {
    pub fn start_egui_record(&mut self) {
        let raw_input = self.egui_state.take_egui_input(&self.window);
        ui().begin_pass(raw_input);
    } 

    pub fn end_egui_record(&mut self) -> egui::FullOutput {
        let full_output = ui().end_pass(); 

        let platform_output = full_output.platform_output.clone();
        self.egui_state.handle_platform_output(&self.window, platform_output);

        return full_output;
    }

    pub fn draw_egui<'a>(&mut self, info: &'a mut FrameInfo, full_output: egui::FullOutput) {
        let paint_jobs = ui().tessellate(full_output.shapes, 1.0);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.surface_config.width, self.surface_config.height],
            pixels_per_point: 1.0,
        };

        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer.update_texture(&self.device, &self.queue, *id, image_delta);
        }

        self.egui_renderer.update_buffers(&self.device, &self.queue, &mut info.encoder, &paint_jobs, &screen_descriptor);

        {
            let mut ui_pass = info.encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &info.view, 
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load, 
                        store: StoreOp::Store
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None
            }).forget_lifetime();

            self.egui_renderer.render(&mut ui_pass, &paint_jobs[..], &screen_descriptor);
        }

        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }
    }
}