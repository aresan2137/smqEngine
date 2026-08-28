use std::sync::Arc;

use wgpu::*;
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};

use crate::ui;

mod frame;
pub use frame::*;

mod egui;
#[allow(unused)]
pub use egui::*;

pub struct Context<'a> {
    pub device: Device,
    pub queue: Queue,
    pub window: Arc<Window>,
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
            device,
            queue,
            window,
            surface,
            egui_renderer,
            egui_state,
            surface_format,
            surface_config
        }, event_loop);
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }
}


