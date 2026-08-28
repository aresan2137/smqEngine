use std::sync::Arc;

use wgpu::*;
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};

use crate::*;

#[cfg(not(target_arch = "wasm32"))]
const BLIT_SHADER: &str = "struct VertexOutput { @builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32>};
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
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
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> { return textureSample(t_color, s_color, in.uv); }";

#[cfg(target_arch = "wasm32")]
const BLIT_SHADER: &str = "struct VertexOutput { @builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32>};
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
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
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(t_color, s_color, in.uv).rgb;

    color = pow(color, vec3<f32>(1.0 / 2.2));

    return vec4<f32>(color, 1.0);
}";



pub struct Context<'a> {
    pub window: Arc<Window>,
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'a>,
    pub surface_format: TextureFormat,
    pub surface_config: SurfaceConfiguration,
    pub egui_renderer: egui_wgpu::Renderer,
    pub egui_state: egui_winit::State
}

pub struct FrameInfo {
    pub frame: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder
}

pub struct BlitInfo {
    pub blit_pipeline: RenderPipeline
}

#[allow(deprecated)]
impl Context<'_> {
    pub async fn new() -> (Self, EventLoop<()>) {
        #[cfg(target_arch = "wasm32")]
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let event_loop = EventLoop::new().unwrap();
        let window_attributes = Window::default_attributes()
            .with_title("game")
            .with_inner_size(PhysicalSize::new(1280, 720));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            let canvas = window.canvas().expect("failed to get canvas");
            
            canvas.set_width(1280);
            canvas.set_height(720);

            canvas.set_attribute("tabindex", "0").unwrap();

            let web_window = web_sys::window().unwrap();
            let document = web_window.document().unwrap();
            let body = document.body().unwrap();
            body.append_child(&canvas).unwrap();
        }

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

        let instance = Instance::default();

        let surface = instance.create_surface(window.clone()).expect("surface creation failed");

        let adapter = instance.request_adapter(&RequestAdapterOptions { 
            power_preference: PowerPreference::HighPerformance, 
            force_fallback_adapter: false, 
            compatible_surface: Some(&surface)
        }).await.expect("adapter request failed");

        let (device, queue) = adapter.request_device(&DeviceDescriptor { 
            label: None, 
            required_features: Features::empty(), 
            required_limits: Limits::default(), 
            memory_hints: MemoryHints::Performance 
        }, None).await.expect("device request failed");

        let adapter_info = adapter.get_info();
        log::info!("using {} with {}", adapter_info.name, adapter_info.backend);

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: 1280,
            height: 720,
            present_mode: PresentMode::Fifo, 
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };
        
        surface.configure(&device, &surface_config);

        let egui_renderer = egui_wgpu::Renderer::new(
            &device, 
            surface_format,
            None, 
            1,
            false
        );

        let egui_state = egui_winit::State::new(
        ui().clone(),
            ui().viewport_id(),
            &window,
            Some(window.scale_factor() as f32),
            None,
            None
        );

        return (Self {
            window,
            instance,
            adapter,
            device,
            queue,
            surface,
            surface_config,
            surface_format,
            egui_renderer,
            egui_state
        }, event_loop);
    }

    pub fn create_blit_pipeline<'a>(&self, bind_group: &BindGroupS) -> BlitInfo {
        let blit_shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(BLIT_SHADER.into()),
        });    

        let blit_pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group.bind_group_layout],
            push_constant_ranges: &[],
        });

        let blit_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&blit_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &blit_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &blit_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.surface_format.clone(),
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        return BlitInfo { 
            blit_pipeline
        };
    }

    pub fn blit_screen(&self, info: &mut FrameInfo, blit_info: &BlitInfo, bind_group: &BindGroupS) {
        {
            let mut blit_pass = info.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &info.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            blit_pass.set_pipeline(&blit_info.blit_pipeline);
            
            blit_pass.set_bind_group(0, &bind_group.bind_group, &[]);
            
            blit_pass.draw(0..3, 0..1);
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn start_frame<'a>(&self) -> Option<FrameInfo> {
        let surface_texture = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Outdated) => {
                return None; 
            }
            Err(wgpu::SurfaceError::Timeout) => {
                return None;
            }
            Err(wgpu::SurfaceError::Lost) => {
                return None;
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                log::error!("out of vram memory");
                panic!();
            }
            Err(wgpu::SurfaceError::Other) => {
                return None;
            }
        };

        let view = surface_texture.texture.create_view(&TextureViewDescriptor::default());

        let encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { 
            label: None 
        });

        return Some(FrameInfo {  
            frame: surface_texture,
            view,
            encoder
        });
    }

    pub fn end_frame(&self, info: FrameInfo) {
        self.queue.submit(std::iter::once(info.encoder.finish()));
        info.frame.present();        
    }

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
            let mut ui_pass = info.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &info.view, 
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, 
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            }).forget_lifetime();

            self.egui_renderer.render(&mut ui_pass, &paint_jobs[..], &screen_descriptor);
        }

        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }
    }

    pub fn create_material_layout(&self, bind_group_layouts: &[&BindGroupLayout]) -> PipelineLayout {
        return self.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: bind_group_layouts,
            push_constant_ranges: &[],
        });
    }

    pub fn get_render_pass<'a>(&'a self, info: &'a mut FrameInfo, render_texture: &RenderTexture) -> RenderPass<'a> {
        let mut color_attachments = Vec::new();
        for i in 0..render_texture.attachments.len() {
            color_attachments.push(Some(RenderPassColorAttachment {
                    view: &render_texture.attachments[i].view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })
            );
        }

        if let Some(view) = &render_texture.depth_view {
            return info.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &color_attachments.as_slice(),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });  
        } else {
            return info.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &color_attachments.as_slice(),
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });  
        }     
    }

    pub fn get_compute_pass<'a>(&self, info: &'a mut FrameInfo) -> ComputePass<'a> {
        return info.encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None
        });
    }
}