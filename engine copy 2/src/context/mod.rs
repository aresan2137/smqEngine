use std::sync::Arc;

use egui_wgpu::RendererOptions;
use wgpu::{wgt::DeviceDescriptor, *};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};

use crate::ui;

mod frame;
pub use frame::*;

mod pass;
pub use pass::*;

mod egui;
pub use egui::*;


pub struct Context<'a> {
    pub window: Arc<Window>,
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'a>,
    pub egui_renderer: egui_wgpu::Renderer,
    pub egui_state: egui_winit::State,
    pub surface_format: TextureFormat,
    pub surface_config: SurfaceConfiguration
}

impl Context<'_> {
    #[allow(deprecated)]
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
            compatible_surface: Some(&surface), 
            apply_limit_buckets: false
        }).await.expect("failed to get adapter");

        let (device, queue) = adapter.request_device(&DeviceDescriptor { 
            label: Some("smq main"), 
            required_features: Features::empty(), 
            required_limits: Limits::default(), 
            experimental_features: ExperimentalFeatures::disabled(), 
            memory_hints: MemoryHints::Performance, 
            trace: Trace::Off 
        }).await.expect("failed to get device");

        let adapter_info = adapter.get_info();
        log::info!("using {} with {}", adapter_info.name, adapter_info.backend);

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let size = window.inner_size();

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: PresentMode::AutoVsync,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: SurfaceColorSpace::Auto
        };

        surface.configure(&device, &surface_config);

        let egui_renderer = egui_wgpu::Renderer::new(&device, surface_format, RendererOptions {
            msaa_samples: 1,
            depth_stencil_format: None,
            dithering: false,
            predictable_texture_filtering: true
        });

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
            device,
            queue,
            surface,
            egui_renderer,
            egui_state,
            surface_format,
            surface_config
        }, event_loop);
    }
}


