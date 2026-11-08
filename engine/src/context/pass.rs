use crate::{RenderTexture, context::*};

impl<D> Context<'_, D> {
    pub fn get_render_pass<'a>(
        &'a self,
        info: &'a mut FrameInfo,
        render_texture: &RenderTexture,
    ) -> RenderPass<'a> {
        let mut color_attachments = Vec::new();
        for i in 0..render_texture.attachments.len() {
            color_attachments.push(Some(RenderPassColorAttachment {
                view: &render_texture.attachments[i].view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
                depth_slice: None,
            }));
        }

        if let Some(depth) = &render_texture.depth_texture {
            return info.encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &color_attachments.as_slice(),
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &depth.1,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        } else {
            return info.encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &color_attachments.as_slice(),
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }
    }

    pub fn get_render_pass_with_others_depth<'a>(
        &'a self,
        info: &'a mut FrameInfo,
        render_texture: &RenderTexture,
        depth_texture: &RenderTexture,
        preserve_depth: bool
    ) -> RenderPass<'a> {
        let mut color_attachments = Vec::new();
        for i in 0..render_texture.attachments.len() {
            color_attachments.push(Some(RenderPassColorAttachment {
                view: &render_texture.attachments[i].view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
                depth_slice: None,
            }));
        }

        let load = if preserve_depth { LoadOp::Load } else { LoadOp::Clear(1.0) };

        if let Some(depth) = &depth_texture.depth_texture {
            return info.encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &color_attachments.as_slice(),
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &depth.1,
                    depth_ops: Some(Operations {
                        load,
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        } else {
            panic!("wanted to used else's depth but else doesn't have depth");
        }
    }

    pub fn get_compute_pass<'a>(&self, info: &'a mut FrameInfo) -> ComputePass<'a> {
        return info.encoder.begin_compute_pass(&ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
    }
}
