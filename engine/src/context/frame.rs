
use std::error::Error;

use crate::{BindGroupS, RenderTexture, context::*};

const BLIT_SHADER: &str = "struct VertexOutput { @builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32>};
@vertex
fn v(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32((vertex_index << 1u) & 2u);
    let y = f32(vertex_index & 2u);
    out.uv = vec2<f32>(x, y);
    out.position = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    return out;
}
@group(0) @binding(0) var t_color: texture_2d<f32>;
@group(0) @binding(1) var s_color: sampler;
@fragment
fn f(in: VertexOutput) -> @location(0) vec4<f32> { return textureSample(t_color, s_color, in.uv); }";

// const BLIT_SHADER: &str = "struct A { @builtin(position) p: vec4<f32>, @location(0) uv: vec2<f32>};
// @vertex
// fn v(@builtin(vertex_index) B: u32) -> A {
//     var o: A;
//     let x = f32((B << 1u) & 2u);
//     let y = f32(B & 2u);
//     o.uv = vec2<f32>(x, y);
//     o.p = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
//     return o;
// }
// @group(0) @binding(0) var T: texture_2d<f32>;
// @group(0) @binding(1) var S: sampler;
// @fragment
// fn f(i: A) -> @location(0) vec4<f32> { return textureSample(T, S, i.uv); }";

pub struct FrameInfo {
    pub frame: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder
}

pub struct BlitInfo {
    pub blit_pipeline: RenderPipeline
}


impl Context<'_> {
    pub fn start_frame<'a>(&self) -> Result<FrameInfo, Box<dyn Error>> {
        let frame = self.surface.get_current_texture()?;

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { 
            label: None
        });

        return Ok(FrameInfo {  
            frame: frame,
            view,
            encoder
        });
    }

    pub fn end_frame(&self, info: FrameInfo) {
        self.queue.submit(std::iter::once(info.encoder.finish()));
        info.frame.present();        
    }

    pub fn get_render_pass<'a>(&'a self, info: &'a mut FrameInfo, render_texture: &RenderTexture) -> RenderPass<'a> {
        let mut color_attachments = Vec::new();
        for i in 0..render_texture.attachments.len() {
            color_attachments.push(Some(RenderPassColorAttachment {
                    view: &render_texture.attachments[i].view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store
                    }
                })
            );
        }

        if let Some(depth) = &render_texture.depth_texture {
            return info.encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &color_attachments.as_slice(),
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &depth.1,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store
                    }),
                    stencil_ops: None
                }),
                timestamp_writes: None,
                occlusion_query_set: None
            });  
        } else {
            return info.encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &color_attachments.as_slice(),
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None
            });  
        }     
    }

    pub fn get_compute_pass<'a>(&self, info: &'a mut FrameInfo) -> ComputePass<'a> {
        return info.encoder.begin_compute_pass(&ComputePassDescriptor {
            label: None,
            timestamp_writes: None
        });
    }

    pub fn create_blit_pipeline<'a>(&self, bind_group: &BindGroupS) -> BlitInfo {
        let blit_shader = self.device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(BLIT_SHADER.into())
        });    

        let blit_pipeline_layout = self.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group.bind_group_layout],
            push_constant_ranges: &[]
        });

        let blit_pipeline = self.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&blit_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &blit_shader,
                entry_point: Some("v"),
                buffers: &[],
                compilation_options: Default::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: &blit_shader,
                entry_point: Some("f"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.surface_format.clone(),
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL
                })],
                compilation_options: Default::default()
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None
        });

        return BlitInfo { 
            blit_pipeline
        };
    }

    pub fn blit_screen(&self, info: &mut FrameInfo, blit_info: &BlitInfo, bind_group: &BindGroupS) {
        {
            let mut blit_pass = info.encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &info.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None
            });

            blit_pass.set_pipeline(&blit_info.blit_pipeline);
            
            blit_pass.set_bind_group(0, &bind_group.bind_group, &[]);
            
            blit_pass.draw(0..3, 0..1);
        }
    }
}

