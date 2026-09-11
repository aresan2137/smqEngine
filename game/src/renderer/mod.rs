// use std::fs;

// use bevy_ecs::world::World;
// use smq_engine::*;
// use wgpu::*;

// use crate::{code::FreeCamera, gen_bake::GenAssets};

// use glam::*;

// pub struct Renderer {
//     gen_assets: GenAssets,
//     // mat1: RenderMaterial,
//     // bind_group0: BindGroupS,
//     // bind_group1: BindGroupS,
//     // bind_group2: BindGroupS,
//     // mesh1: Mesh,
//     // bind_group_blit: BindGroupS,
//     // blit_pipeline: BlitInfo
// }

// impl Renderer {
//     pub fn new(context: &mut Context) -> Self {
//         let mut gen_assets = GenAssets::init_gen_assets(context);

//         // gen_assets.renderer_ubodata0_ubo.data.proj = camera::rh::proj::directx::perspective((60.0_f32).to_radians(), 16.0/9.0, 0.03, 500.0);

//         // gen_assets.renderer_ubodata0_ubo.upload_data(context);
//         // gen_assets.renderer_ubodata2_ubo.upload_data(context);

//         // let bind_group0 = BindGroupS::new(context, &[
//         //     gen_assets.renderer_ubodata0_ubo.get_binding(ShaderStages::VERTEX)
//         // ], None);

//         // let bind_group1 = BindGroupS::new(context, &[
//         //     gen_assets.renderer_ubodata1_ubo.get_binding(ShaderStages::VERTEX)
//         // ], None);

//         // let bind_group2 = BindGroupS::new(context, &[
//         //     gen_assets.renderer_ubodata2_ubo.get_binding(ShaderStages::VERTEX),
            
//         // ], None);

//         // let mat1layout = context.device.create_pipeline_layout(&PipelineLayoutDescriptor { 
//         //     label: None, 
//         //     bind_group_layouts: &[
//         //         Some(&bind_group0.bind_group_layout),
//         //         Some(&bind_group1.bind_group_layout),
//         //         Some(&bind_group2.bind_group_layout),
//         //         None
//         //     ],
//         //     immediate_size: 0
//         // });

//         // let mat1module = context.device.create_shader_module(ShaderModuleDescriptor { 
//         //     label: None, 
//         //     source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("../../../smq_proj/data/file/renderer/shaders/base.wgsl")))
//         // });

//         // let mat1 = context.device.create_render_pipeline(&RenderPipelineDescriptor { 
//         //     label: None,
//         //     layout: Some(&mat1layout),
//         //     vertex: VertexState {
//         //         module: &mat1module,
//         //         entry_point: Some("vs_main"),
//         //         compilation_options: PipelineCompilationOptions::default(),
//         //         buffers: &[
//         //             Some(VertexBufferLayout { 
//         //                 array_stride: 32, 
//         //                 step_mode: VertexStepMode::Vertex, 
//         //                 attributes: &[
//         //                     VertexAttribute { 
//         //                         format: VertexFormat::Float32x3,
//         //                         offset: 0,
//         //                         shader_location: 0
//         //                     },
//         //                     VertexAttribute { 
//         //                         format: VertexFormat::Float32x2,
//         //                         offset: 12,
//         //                         shader_location: 1
//         //                     },
//         //                     VertexAttribute { 
//         //                         format: VertexFormat::Float32x3,
//         //                         offset: 20,
//         //                         shader_location: 2
//         //                     }
//         //                 ]
//         //             })
//         //         ]
//         //     }, 
//         //     primitive: PrimitiveState { 
//         //         topology: PrimitiveTopology::TriangleList,
//         //         strip_index_format: None,
//         //         front_face: FrontFace::Ccw,
//         //         cull_mode: None,
//         //         unclipped_depth: false,
//         //         polygon_mode: PolygonMode::Fill,
//         //         conservative: false
//         //     }, 
//         //     depth_stencil: Some(DepthStencilState { 
//         //         format: TextureFormat::Depth32Float,
//         //         depth_write_enabled: Some(true),
//         //         depth_compare: Some(CompareFunction::Less),
//         //         stencil: StencilState::default(),
//         //         bias: DepthBiasState::default()
//         //     }), 
//         //     multisample: MultisampleState { 
//         //         count: 1, 
//         //         mask: !0, 
//         //         alpha_to_coverage_enabled: false 
//         //     }, 
//         //     fragment: Some(FragmentState { 
//         //         module: &mat1module,
//         //         entry_point: Some("fs_main"),
//         //         compilation_options: PipelineCompilationOptions::default(),
//         //         targets: &[Some(ColorTargetState { format: TextureFormat::Rgba8Unorm, blend: None, write_mask: ColorWrites::ALL })]
//         //     }), 
//         //     multiview_mask: None, 
//         //     cache: None
//         // });

//         // let mesh1 = Mesh::new(context, 
//         //     include_bytes!("../../../smq_proj/data/file/mesh/Suzanne.smf"), 32, None, 
//         //     BufferUsages::COPY_DST | BufferUsages::VERTEX).unwrap();

//         // let bind_group_blit = BindGroupS::new(context, &[
//         //     gen_assets.renderer_main_renderTexture.attachments[0].get_texture_binding(ShaderStages::FRAGMENT),
//         //     gen_assets.renderer_main_renderTexture.attachments[0].get_sampler_binding(ShaderStages::FRAGMENT)
//         // ], None);

//         // let blit_pipeline = context.create_blit_pipeline(&bind_group_blit); 

//         return Self {
//             gen_assets,
//             // mat1,
//             // bind_group0,
//             // bind_group1,
//             // bind_group2,
//             // mesh1,
//             // bind_group_blit,
//             // blit_pipeline
//         };
//     }

//     fn prep_data(&mut self, context: &mut Context, world: &mut World) {
//         // let mut camera_query = world.query::<&FreeCamera>();

//         // if let Some(cam) = camera_query.iter(world).next() {
//         //     let q_yaw = Quat::from_axis_angle(Vec3::Y, cam.yaw.to_radians());
//         //     let q_pitch = Quat::from_axis_angle(Vec3::X, cam.pitch.to_radians());
//         //     let rotation = (q_yaw * q_pitch).normalize();

//         //     let forward = rotation * Vec3::new(0.0, 0.0, 1.0);
//         //     let up = rotation * Vec3::new(0.0, 1.0, 0.0);

//         //     let view = glam::camera::lh::view::look_to_mat4(cam.position, forward, up);
//         //     //let view = glam::camera::rh::view::look_to_mat4(cam.position, forward, up);

//         //     self.gen_assets.renderer_ubodata1_ubo.data.view = view;
//         //     self.gen_assets.renderer_ubodata1_ubo.upload_data(context);

//         //     println!("x: {}, y: {}, z: {}", cam.position.x, cam.position.y, cam.position.z);
//         // } else {
//         //     println!("NO CAMERA!!!");
//         // }
//     } 

//     pub fn draw(&mut self, context: &mut Context, world: &mut World) {
//         // if let Some(mut frame) = context.start_frame() {
//         //     self.prep_data(context, world);

//         //     {
//         //         let mut render_pass = context.get_render_pass(&mut frame, &self.gen_assets.renderer_main_renderTexture);

//         //         render_pass.set_pipeline(&mut self.mat1);

//         //         render_pass.set_vertex_buffer(0, self.mesh1.buffer.slice(..));

//         //         render_pass.set_bind_group(0, &self.bind_group0.bind_group, &[]);
//         //         render_pass.set_bind_group(1, &self.bind_group1.bind_group, &[]);
//         //         render_pass.set_bind_group(2, &self.bind_group2.bind_group, &[]);

//         //         render_pass.draw(0..self.mesh1.vertex_count, 0..1);
//         //     }

//         //     context.blit_screen(&mut frame, &self.blit_pipeline, &self.bind_group_blit);

//         //     //context.draw_egui(&mut frame, full_output);

//         //     context.end_frame(frame);
//         // }

        

        
//     }
// }

