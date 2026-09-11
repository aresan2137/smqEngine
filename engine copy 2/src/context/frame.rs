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

pub struct FrameInfo {
    pub frame: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder
}

pub struct BlitInfo {
    pub blit_pipeline: RenderPipeline
}

impl Context<'_> {
    pub fn start_frame(&mut self) -> Option<FrameInfo> {
        let frame = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(frame) => frame,
            CurrentSurfaceTexture::Suboptimal(frame) => frame,
            CurrentSurfaceTexture::Timeout => return None,
            CurrentSurfaceTexture::Occluded => return None,
            CurrentSurfaceTexture::Outdated => todo!(),
            CurrentSurfaceTexture::Lost => todo!(),
            CurrentSurfaceTexture::Validation => panic!("validation")
        };

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { 
            label: None
        });

        Some(FrameInfo {  
            frame,
            view,
            encoder
        })
    }

    pub fn end_frame(&self, info: FrameInfo) {
        self.queue.submit(std::iter::once(info.encoder.finish()));

        self.queue.present(info.frame);
    }

    // pub fn get_compute_pass<'a>(&self, info: &'a mut FrameInfo) -> ComputePass<'a> {
    //     return info.encoder.begin_compute_pass(&ComputePassDescriptor {
    //         label: None,
    //         timestamp_writes: None
    //     });
    // }

    pub fn create_blit_pipeline<'a>(&self, bind_group: &BindGroupS) -> BlitInfo {
        let blit_shader = self.device.create_shader_module(ShaderModuleDescriptor { 
            label: None, 
            source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(BLIT_SHADER))
        });

        let blit_pipeline_layout = self.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&bind_group.bind_group_layout)],
            immediate_size: 0
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
            multiview_mask: None,
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
                    depth_slice: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None
            });

            blit_pass.set_pipeline(&blit_info.blit_pipeline);
            
            blit_pass.set_bind_group(0, &bind_group.bind_group, &[]);
            
            blit_pass.draw(0..3, 0..1);
        }
    }

    // pub fn clear_surface(&self, info: &mut FrameInfo) {
    //     info.encoder.begin_render_pass(&RenderPassDescriptor {
    //         label: None,
    //         color_attachments: &[Some(RenderPassColorAttachment {
    //             view: &info.view, 
    //             resolve_target: None,
    //             ops: Operations {
    //                 load: LoadOp::Clear(Color::BLACK), 
    //                 store: StoreOp::Store
    //             },
    //         })],
    //         depth_stencil_attachment: None,
    //         timestamp_writes: None,
    //         occlusion_query_set: None
    //     });
    // }
}

