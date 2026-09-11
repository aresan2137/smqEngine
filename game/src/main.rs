use bevy_ecs::prelude::*;
use glam::*;

use smq_engine::*;

use wgpu::*;

mod code;
use code::*;

mod renderer;
use renderer::*;

mod gen_bake;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoop;

use crate::gen_bake::GenAssets;

pub fn fps_logger(time: Res<Delta>) {
    egui::Window::new("FPS").default_width(400.0).show(&ui(), |ui| {
        ui.label(format!("fps: {}", 1.0/time.delta));
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() {    
    logger::init();
    
    let mut world = World::new();
    world.insert_resource(Delta::new());
    world.insert_resource(Inputs::default());

    let mut schedule = Schedule::default();
    schedule.add_systems(free_camera_system);
    schedule.add_systems(fps_logger);

    world.spawn(
        FreeCamera {
            position: Vec3::ZERO,
            pitch: 0.0,
            yaw: 0.0,
            speed: 5.0,
            sensitivity: 0.3,
            is_controlling: false
        }
    );

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut context: Context<'_, Renderer> = Context::new(world, schedule, render, load_assets);

    event_loop.run_app(&mut context).unwrap();
}

#[allow(unused)]
struct Renderer {
    gen_assets: GenAssets,
    mat1: RenderMaterial,
    bind_group0: BindGroupS,
    bind_group1: BindGroupS,
    bind_group2: BindGroupS,
    mesh1: Mesh,
    bind_group_blit: BindGroupS,
    blit_pipeline: BlitInfo
}

#[allow(unused)]
fn load_assets(context: &mut Context<Renderer>) {
    let mut gen_assets = GenAssets::init_gen_assets(context);

    gen_assets.renderer_ubodata0_ubo.data.proj = camera::rh::proj::directx::perspective((60.0_f32).to_radians(), 16.0/9.0, 0.03, 500.0);

    gen_assets.renderer_ubodata0_ubo.upload_data(context);
    gen_assets.renderer_ubodata2_ubo.upload_data(context);

    let bind_group0 = BindGroupS::new(context, &[
        gen_assets.renderer_ubodata0_ubo.get_binding(ShaderStages::VERTEX)
    ], None);

    let bind_group1 = BindGroupS::new(context, &[
        gen_assets.renderer_ubodata1_ubo.get_binding(ShaderStages::VERTEX)
    ], None);

    let bind_group2 = BindGroupS::new(context, &[
        gen_assets.renderer_ubodata2_ubo.get_binding(ShaderStages::VERTEX),
        
    ], None);

    let holding = context.holding.as_mut().unwrap();

    let mat1layout = holding.device.create_pipeline_layout(&PipelineLayoutDescriptor { 
        label: None, 
        bind_group_layouts: &[
            Some(&bind_group0.bind_group_layout),
            Some(&bind_group1.bind_group_layout),
            Some(&bind_group2.bind_group_layout),
            None
        ],
        immediate_size: 0
    });

    let mat1module = holding.device.create_shader_module(ShaderModuleDescriptor { 
        label: None, 
        source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("../../smq_proj/data/file/renderer/shaders/base.wgsl")))
    });

    let mat1 = holding.device.create_render_pipeline(&RenderPipelineDescriptor { 
        label: None,
        layout: Some(&mat1layout),
        vertex: VertexState {
            module: &mat1module,
            entry_point: Some("vs_main"),
            compilation_options: PipelineCompilationOptions::default(),
            buffers: &[
                Some(VertexBufferLayout { 
                    array_stride: 32, 
                    step_mode: VertexStepMode::Vertex, 
                    attributes: &[
                        VertexAttribute { 
                            format: VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0
                        },
                        VertexAttribute { 
                            format: VertexFormat::Float32x2,
                            offset: 12,
                            shader_location: 1
                        },
                        VertexAttribute { 
                            format: VertexFormat::Float32x3,
                            offset: 20,
                            shader_location: 2
                        }
                    ]
                })
            ]
        }, 
        primitive: PrimitiveState { 
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            unclipped_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false
        }, 
        depth_stencil: Some(DepthStencilState { 
            format: TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(CompareFunction::Less),
            stencil: StencilState::default(),
            bias: DepthBiasState::default()
        }), 
        multisample: MultisampleState { 
            count: 1, 
            mask: !0, 
            alpha_to_coverage_enabled: false 
        }, 
        fragment: Some(FragmentState { 
            module: &mat1module,
            entry_point: Some("fs_main"),
            compilation_options: PipelineCompilationOptions::default(),
            targets: &[Some(ColorTargetState { format: TextureFormat::Rgba8Unorm, blend: None, write_mask: ColorWrites::ALL })]
        }), 
        multiview_mask: None, 
        cache: None
    });

    let mesh1 = Mesh::new(context, 
        include_bytes!("../../smq_proj/data/file/mesh/Suzanne.smf"), 32, None).unwrap();

    let bind_group_blit = BindGroupS::new(context, &[
        gen_assets.renderer_main_renderTexture.attachments[0].get_texture_binding(ShaderStages::FRAGMENT),
        gen_assets.renderer_main_renderTexture.attachments[0].get_sampler_binding(ShaderStages::FRAGMENT)
    ], None);

    let blit_pipeline = context.create_blit_pipeline(&bind_group_blit); 
    
    context.data = Some(Renderer { 
        gen_assets, 
        mat1: RenderMaterial { pipeline: mat1 }, 
        bind_group0, 
        bind_group1, 
        bind_group2, 
        mesh1, 
        bind_group_blit, 
        blit_pipeline 
    });
}

#[allow(unused)]
fn render(context: &mut Context<Renderer>, full_output: ::egui::FullOutput) {
    let mut data = context.data.take().unwrap();

    let mut camera_query = context.world.query::<&FreeCamera>();

    if let Some(cam) = camera_query.iter(&context.world).next() {
        let q_yaw = Quat::from_axis_angle(Vec3::Y, cam.yaw.to_radians());
        let q_pitch = Quat::from_axis_angle(Vec3::X, cam.pitch.to_radians());
        let rotation = (q_yaw * q_pitch).normalize();

        let forward = rotation * Vec3::new(0.0, 0.0, 1.0);
        let up = rotation * Vec3::new(0.0, 1.0, 0.0);

        let view = glam::camera::lh::view::look_to_mat4(cam.position, forward, up);

        data.gen_assets.renderer_ubodata1_ubo.data.view = view;
        data.gen_assets.renderer_ubodata1_ubo.upload_data(context);
    } else {
        println!("NO CAMERA!!!");
    }

    if let Some(mut frame) = context.start_frame() {
        {
                let mut render_pass = context.get_render_pass(&mut frame, &data.gen_assets.renderer_main_renderTexture);

                render_pass.set_pipeline(&mut data.mat1.pipeline);

                render_pass.set_vertex_buffer(0, data.mesh1.buffer.slice(..));

                render_pass.set_bind_group(0, &data.bind_group0.bind_group, &[]);
                render_pass.set_bind_group(1, &data.bind_group1.bind_group, &[]);
                render_pass.set_bind_group(2, &data.bind_group2.bind_group, &[]);

                render_pass.draw(0..data.mesh1.vertex_count, 0..1);
            }

            context.blit_screen(&mut frame, &data.blit_pipeline, &data.bind_group_blit);

        context.draw_egui(&mut frame, full_output);

        context.end_frame(frame);
    }   

    context.data = Some(data);
}

