use std::sync::Arc;

use bevy_ecs::prelude::*;
use egui_wgpu::RendererOptions;
use wgpu::*;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{DeviceEvent, DeviceId, WindowEvent}, event_loop::ActiveEventLoop, keyboard::PhysicalKey, window::{Window, WindowId}};
use winit::event_loop::EventLoopProxy;

use crate::{Delta, Inputs, f11_system};

mod egui;
pub use egui::*;

mod frame;
pub use frame::*;

mod pass;
#[allow(unused)]
pub use pass::*;

pub struct Context<'a, D> {
    pub window: Option<Arc<Window>>,
    pub world: World,
    pub schedule: Schedule,
    is_minimized: bool,
    events: ContextEvents<D>,
    pub settings: ContextSettings,
    pub surface: Option<Arc<Surface<'a>>>,
    pub holding: Option<ContextWgpuHolding>,
    pub data: Option<D>,
    pub proxy: EventLoopProxy<ContextUserEvent>
}

pub struct ContextEvents<D> {
    pub renderer: Option<fn(&mut Context<D>, ::egui::FullOutput)>,
    pub on_wgpu_load: Option<fn(&mut Context<D>)>,
    pub pre_schedule: Option<fn(&mut Context<D>)>,
    pub on_exit: Option<fn(&mut Context<D>)>
}

#[derive(Clone)]
pub struct ContextSettings {
    pub present_mode: PresentMode 
}

impl Default for ContextSettings {
    fn default() -> Self {
        return Self { 
            present_mode: PresentMode::AutoVsync
        };
    }
}


impl<D> Context<'_, D> {
    pub fn new(world: World, schedule: Schedule, proxy: EventLoopProxy<ContextUserEvent>, settings: ContextSettings, events: ContextEvents<D>) -> Self {        
        return Self {
            window: None,
            world,
            schedule,
            is_minimized: false,
            events,
            settings,
            surface: None,
            holding: None,
            data: None,
            proxy
        };
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            if let Some(holding) = self.holding.as_mut() {
                if holding.surface_config.width != new_size.width || holding.surface_config.height != new_size.height {
                    holding.surface_config.width = new_size.width;
                    holding.surface_config.height = new_size.height;
                    self.surface.as_ref().unwrap().configure(&holding.device, &holding.surface_config);
                }
            }            
        }
    }
}

pub struct ContextWgpuHolding {
    pub device: Device, 
    pub queue: Queue,
    pub egui_renderer: egui_wgpu::Renderer,
    pub egui_state: egui_winit::State,
    pub surface_config: wgt::SurfaceConfiguration<Vec<TextureFormat>>,
    pub surface_format: TextureFormat
}

impl ContextWgpuHolding {
    async fn new(window: Arc<Window>, instance: &Instance, surface: &Surface<'_>, settings: &ContextSettings) -> Self {
        let adapter = instance.request_adapter(&RequestAdapterOptions { 
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false, 
            compatible_surface: Some(&surface),
            apply_limit_buckets: false
        }).await.expect("failed to get adapter");

        let (device, queue) = adapter.request_device(&DeviceDescriptor { 
            label: None, 
            required_features: Features::empty(), 
            required_limits: Limits::defaults(), 
            experimental_features: ExperimentalFeatures::disabled(), 
            memory_hints: MemoryHints::Performance, 
            trace: Trace::Off
        }).await.expect("failed to get device");
        
        let adapter_info = adapter.get_info();
        log::info!("using {} with {}", adapter_info.name, adapter_info.backend);

        let surface_format = surface.get_capabilities(&adapter).formats[0];

        let size = window.inner_size();

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: settings.present_mode,
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
            predictable_texture_filtering: false 
        });

        let egui_state = egui_winit::State::new(
            ui(),
            ui().viewport_id(),
            &window,
            Some(window.scale_factor() as f32),
            None,
            None
        );

        return Self {
            device,
            queue,
            egui_renderer,
            egui_state,
            surface_config,
            surface_format
        };
    }
}

#[allow(unused)]
pub enum ContextUserEvent {
    GpuReady(ContextWgpuHolding)
}

impl<D> ApplicationHandler<ContextUserEvent> for Context<'_, D> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("game")
                .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720));

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

            let instance = Instance::default();

            let surface = Arc::new(instance.create_surface(window.clone()).expect("failed to get surface"));

            #[cfg(target_arch = "wasm32")]
            {
                let event_proxy = self.proxy.clone();

                let window_clone = window.clone();
                let surface_clone = surface.clone();    
                let settings_clone = self.settings.clone();            

                wasm_bindgen_futures::spawn_local(async move {
                    let _ = event_proxy.send_event(ContextUserEvent::GpuReady(ContextWgpuHolding::new(window_clone, &instance, &surface_clone, &settings_clone).await));
                });
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                self.holding = Some(pollster::block_on(ContextWgpuHolding::new(window.clone(), &instance, &surface, &self.settings)));
            }

            self.surface = Some(surface);
            self.window = Some(window);

            #[cfg(not(target_arch = "wasm32"))]
            if let Some(func) = self.events.on_wgpu_load {
                (func)(self);
            }            
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: ContextUserEvent) {
        match event {
            ContextUserEvent::GpuReady(holding) => {
                self.holding = Some(holding);
                if let Some(func) = self.events.on_wgpu_load {
                    (func)(self);
                }
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.as_ref() else { return };
        if window_id != window.id() { return; }

        if let Some(holding) = self.holding.as_mut() {
            let response = holding.egui_state.on_window_event(window.as_ref(), &event);
            
            if response.consumed {
                return;
            }
        }

        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    self.world.get_resource_mut::<Inputs>().expect("input not found").on_input(&event, keycode);
                }

                #[cfg(not(target_arch = "wasm32"))]
                f11_system(&self.world, &self.window);
            }
            WindowEvent::CloseRequested => {
                if let Some(func) = self.events.on_exit {
                    (func)(self);
                }

                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if physical_size.width == 0 || physical_size.height == 0 {
                    self.is_minimized = true;
                } else {
                    self.is_minimized = false;
                    self.resize(&physical_size);
                }
            }
            WindowEvent::RedrawRequested => {
                if self.holding.is_some() {
                    self.start_egui_record();

                    self.world.get_resource_mut::<Delta>().expect("delta not found").update_delta();

                    if let Some(func) = self.events.pre_schedule {
                        (func)(self);
                    }

                    self.schedule.run(&mut self.world);

                    let mut full_output = self.end_egui_record();

                    if self.is_minimized { 
                        full_output.textures_delta.clear();
                        return; 
                    }

                    if let Some(func) = self.events.renderer {
                        (func)(self, full_output);
                    }

                    self.world.get_resource_mut::<Inputs>().expect("input not found").end_frame();
                }
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            let mut input = self.world.resource_mut::<Inputs>();
            input.mouse_delta.0 += delta.0 as f32;
            input.mouse_delta.1 += delta.1 as f32;
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

