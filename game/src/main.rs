use std::fs;

use bevy_ecs::prelude::*;
use glam::*;

use smq_engine::{other::bvh::{BVHNode, LoadedBvhData, ModelInstance, load_bvh_data}, *};

use wgpu::*;

mod gen_bake;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoop;

use crate::gen_bake::GenAssets;

mod gen_game;

pub fn fps_logger(time: Res<Delta>) {
    egui::Window::new("FPS").default_width(400.0).show(&ui(), |ui| {
        ui.label(format!("fps: {}", 1.0/time.delta));
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
#[allow(unused)]
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

    world.spawn(PointLight {
        position: Vec3 { x: 5.0, y: 5.0, z: 0.0 },
        color: Vec3::ONE,
        power: 5.0
    });

    world.spawn(PointLight {
        position: Vec3 { x: 00.0, y: 5.0, z: 20.0 },
        color: Vec3::new(0.3, 0.9, 0.4),
        power: 5.0
    });

    world.spawn(DrawedObject {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        size: Vec3::ONE,
        meshid: 2
    });

    world.spawn(DrawedObject {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        size: Vec3::ONE,
        meshid: 3
    });

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut context: Context<'_, Renderer> = Context::new(world, schedule, event_loop.create_proxy(), ContextSettings { 
        present_mode: PresentMode::AutoNoVsync
    }, ContextEvents { 
        renderer: Some(render), 
        on_wgpu_load: Some(load_assets), 
        pre_schedule: None,
        on_exit: None
    });

    event_loop.run_app(&mut context).unwrap();
}

#[allow(unused)]
struct Renderer {
    gen_assets: GenAssets,
    mat1: RenderMaterial,
    mat2: RenderMaterial,
    bind_group0: BindGroupS,
    bind_group1: BindGroupS,
    bind_group2: BindGroupS,
    bind_group0post: BindGroupS,
    bvh_mesh: Mesh,
    bvh_bimbo: LoadedBvhData,
    instances_buffer: Buffer,
    bvh_buffer: Buffer
}

#[allow(unused)]
fn load_assets(context: &mut Context<Renderer>) {

    #[cfg(not(target_arch = "wasm32"))]
    let ssf_text_vec = fs::read("smq_proj/build/game.ssf").unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    let ssf_text = ssf_text_vec.as_slice();

    #[cfg(target_arch = "wasm32")]
    let ssf_text = include_bytes!("../../smq_proj/build/game.ssf");

    let ssf_data = load_ssf(ssf_text).unwrap();

    let mut gen_assets = GenAssets::init_gen_assets(context, &ssf_data);

    gen_assets.ubodata0.data.proj = camera::rh::proj::directx::perspective((60.0_f32).to_radians(), 16.0/9.0, 0.03, 500.0);

    gen_assets.ubodata0.upload_data(context);
    gen_assets.ubodata2.upload_data(context);

    let bind_group0 = BindGroupS::new(context, &[
        gen_assets.ubodata0.get_binding(ShaderStages::VERTEX)
    ], None);

    let bind_group1 = BindGroupS::new(context, &[
        gen_assets.ubodata1.get_binding(ShaderStages::VERTEX)
    ], None);

    let bind_group2 = BindGroupS::new(context, &[
        gen_assets.ubodata2.get_binding(ShaderStages::VERTEX),
        gen_assets.lakaka_png.get_texture_binding(ShaderStages::FRAGMENT),
        gen_assets.lakaka_png.get_sampler_binding(ShaderStages::FRAGMENT)
    ], None);

    let bvh_mesh = Mesh::new_with_custom_buffer_usages(context, ssf_data[gen_game::game::smq_proj_build_bvh_smf as usize].as_ref(), 32, None, wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE).unwrap();

    let bvh_bimbo = load_bvh_data(ssf_data[gen_game::game::smq_proj_build_bvh_bin as usize].as_ref()).unwrap();

    let holding1 = context.holding.as_mut().unwrap();

    let bvh_buffer = holding1.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        mapped_at_creation: false,
        size: (bvh_bimbo.mega_nodes.len() * std::mem::size_of::<BVHNode>()) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE
    });
    holding1.queue.write_buffer(&bvh_buffer, 0, bytemuck::cast_slice(&bvh_bimbo.mega_nodes));

    let instances_buffer = holding1.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        mapped_at_creation: false,
        size: (1024 * std::mem::size_of::<ModelInstance>()) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE
    });

    let bind_group0post = BindGroupS::new(context, &[
        gen_assets.ubodefferedinfo.get_binding(ShaderStages::FRAGMENT),
        gen_assets.renderer_main_renderTexture.attachments[0].get_texture_binding(ShaderStages::FRAGMENT, false),
        gen_assets.renderer_main_renderTexture.attachments[1].get_texture_binding(ShaderStages::FRAGMENT, false),
        gen_assets.renderer_main_renderTexture.attachments[2].get_texture_binding(ShaderStages::FRAGMENT, false),
        bvh_mesh.get_storage_binding(ShaderStages::FRAGMENT, BufferBindingType::Storage { read_only: true }),
        BindingS {
            entry_layout: wgpu::BindGroupLayoutEntry {
                binding: 0, 
                visibility: wgpu::ShaderStages::FRAGMENT, 
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None
            },
            entry: wgpu::BindGroupEntry { binding: 1, resource: bvh_buffer.as_entire_binding() }
        },
        BindingS {
            entry_layout: wgpu::BindGroupLayoutEntry { 
                binding: 0, 
                visibility: wgpu::ShaderStages::FRAGMENT, 
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None
            },
            entry: wgpu::BindGroupEntry { binding: 2, resource: instances_buffer.as_entire_binding() }
        }
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
        source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("../../smq_proj/data/file/renderer/shaders/textured.wgsl")))
    });

    let mat2layout = holding.device.create_pipeline_layout(&PipelineLayoutDescriptor { 
        label: None, 
        bind_group_layouts: &[
            Some(&bind_group0post.bind_group_layout),
            None,
            None,
            None
        ],
        immediate_size: 0
    });

    let mat2module = holding.device.create_shader_module(ShaderModuleDescriptor { 
        label: None, 
        source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("../../smq_proj/data/file/renderer/shaders/deffering.wgsl")))
    });

    let mat1 = RenderMaterial::new(context, Some(Face::Back), None, &gen_assets.renderer_main_renderTexture, 
        &mat1module, get_default_vert_layout(), Some(CompareFunction::Less), &mat1layout);

    let mat2 = RenderMaterial::new(context, None, None, &gen_assets.renderer_post_renderTexture, 
        &mat2module, None, None, &mat2layout);

    context.data = Some(Renderer { 
        gen_assets, 
        mat1,
        mat2,
        bind_group0,
        bind_group1,
        bind_group2,
        bind_group0post,
        bvh_mesh,
        bvh_bimbo,
        instances_buffer,
        bvh_buffer
    });
}

#[derive(Component)]
struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub power: f32
}

#[derive(Component)]
struct DrawedObject {
    pub position: Vec3,
    pub rotation: Quat,
    pub size: Vec3,
    pub meshid: usize
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

        data.gen_assets.ubodata1.data.view = view;
        data.gen_assets.ubodata1.upload_data(context);

        data.gen_assets.ubodefferedinfo.data.camera_position = cam.position;
    } else {
        println!("NO CAMERA!!!");
    }

    let mut light_query = context.world.query::<&PointLight>();

    for (i, light) in light_query.iter(&context.world).enumerate() {
        if i >= 16 {
            log::warn!("TOO MUCH LIGHTS!!!");
            break;
        }

        data.gen_assets.ubodefferedinfo.data.light_count = i as u32 + 1;

        data.gen_assets.ubodefferedinfo.data.lights[i].position = light.position;
        data.gen_assets.ubodefferedinfo.data.lights[i].color = light.color;
        data.gen_assets.ubodefferedinfo.data.lights[i].power = light.power;
    }

    let mut instances_data = Vec::new();

    let mut objects_query = context.world.query::<&DrawedObject>();
    for (i, pos) in objects_query.iter(&context.world).enumerate() {
        //let model_matrix = Mat4::from_scale_rotation_translation(pos.size, pos.rotation, pos.position);
        let model_matrix = Mat4::IDENTITY;
        
        instances_data.push(ModelInstance {
            inv_model: model_matrix.inverse(),
            bvh_root_node: data.bvh_bimbo.bvh_roots[pos.meshid as usize],
            id: pos.meshid as u32,
            _padding: [0; 2]
        });

        //self.dyn_ubo.data[i].mvp = view_proj * model_matrix;
    }

    //self.dyn_ubo.upload_data(&context);

    context.holding.as_mut().unwrap().queue.write_buffer(&data.instances_buffer, 0, bytemuck::cast_slice(&instances_data));

    data.gen_assets.ubodefferedinfo.data.instance_count = 2;

    data.gen_assets.ubodefferedinfo.upload_data(context);

    if let Some(mut frame) = context.start_frame() {
        {
            let mut render_pass = context.get_render_pass(&mut frame, &data.gen_assets.renderer_main_renderTexture);

            render_pass.set_pipeline(&mut data.mat1.pipeline);

            render_pass.set_bind_group(0, &data.bind_group0.bind_group, &[]);
            render_pass.set_bind_group(1, &data.bind_group1.bind_group, &[]);
            render_pass.set_bind_group(2, &data.bind_group2.bind_group, &[]);

            render_pass.set_vertex_buffer(0, data.gen_assets.file_mesh_Walls_smf.buffer.slice(..));
            render_pass.draw(0..data.gen_assets.file_mesh_Walls_smf.vertex_count, 0..1);

            render_pass.set_vertex_buffer(0, data.gen_assets.file_mesh_Floor_smf.buffer.slice(..));
            render_pass.draw(0..data.gen_assets.file_mesh_Floor_smf.vertex_count, 0..1);
        }

        {
            let mut render_pass = context.get_render_pass(&mut frame, &data.gen_assets.renderer_post_renderTexture);

            render_pass.set_pipeline(&mut data.mat2.pipeline);

            render_pass.set_bind_group(0, &data.bind_group0post.bind_group, &[]);

            render_pass.draw(0..3, 0..1);
        }

        context.blit_screen(&mut frame, &data.gen_assets.blitinfo, &data.gen_assets.blit_group);

        context.draw_egui(&mut frame, full_output);

        context.end_frame(frame);
    }   

    context.data = Some(data);
}

