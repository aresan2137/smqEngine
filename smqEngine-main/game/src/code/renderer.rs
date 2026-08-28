use std::vec;

use bevy_ecs::prelude::*;
use glam::*;

use wgpu::*;

use smq_engine::*;

use crate::{code::bvh::MegaGeometryData, *};

const MAX_LIGHTS: usize = 16;

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct PointLightData {
    pub position_and_intensity: Vec4,
    pub color_and_radius: Vec4,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LightsUniforms {
    pub lights: [PointLightData; MAX_LIGHTS],
}

impl Default for LightsUniforms {
    fn default() -> Self {
        Self {
            lights: [PointLightData::default(); MAX_LIGHTS],
        }
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniforms {
    pub inv_view_proj: Mat4,
    pub camera_pos: Vec4, 
    pub light_count: u32, 
    pub instance_count: u32,
    pub _padding: [u32; 2],
}

#[repr(C)]
#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct DynUBO {
    pub mvp: Mat4,
    pub _padding1: Mat4,
    pub _padding2: Mat4,
    pub _padding3: Mat4,
}

pub struct Assets {
    pub is_minimized: bool,
    dyn_ubo: DynamicUBO<DynUBO>,
    camera_ubo: UBO<CameraUniforms>,
    render_texture: RenderTexture,
    mat: Material,
    draw_bind_group: BindGroupS,
    comp: Material,
    comp_bind_group: BindGroupS,
    comp_binding_2: BindGroupS,
    mesh_bind_group: BindGroupS,
    blit_pipeline: BlitInfo,
    blit_bind_group: BindGroupS,
    mega_data: MegaGeometryData,
    instances_buffer: Buffer,
    light_ubo: UBO<LightsUniforms>,
    meshes: Vec<Mesh>,
    texture_bind_group: BindGroupS
}

impl Assets {
    pub fn new(context: &Context, mesh_data: Vec<&[u8]>) -> Self {
        let is_minimized = false;

        let render_texture = RenderTexture::from_json(context, include_str!("../../../smq_proj/assets/main.renderTexture"));
        let blit_render_texture = RenderTexture::from_json(context, include_str!("../../../smq_proj/assets/blit.renderTexture"));

        //let texture = TextureS::from_color(&context, Color {r: 255.0, g: 255.0, b: 255.0, a: 255.0});
        let texture = TextureS::from_json(context, include_bytes!("../../../smq_proj/assets/graphics.png"), include_str!("../../../smq_proj/assets/graphics.png.json"));

        let ubo_data = vec![DynUBO::default(); 1024];

        let dyn_ubo  = DynamicUBO::<DynUBO>::new(context, ubo_data);

        let lights_data = LightsUniforms::default();
        let light_ubo = UBO::<LightsUniforms>::new(context, lights_data);

        let camera_ubo_data = CameraUniforms::default();
        let camera_ubo = UBO::<CameraUniforms>::new(context, camera_ubo_data);

        let mut meshes = Vec::new();

        for &data in mesh_data.iter() {
            meshes.push(Mesh::new(context, data, 32));
        }

        let mega_data = bvh::build_mega_geometry(mesh_data.as_slice(), 32);

        let mesh = Mesh::from_raw_bytes(&context, mega_data.vertex_count, &mega_data.mega_vertex_bytes);

        let bvh_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            mapped_at_creation: false,
            size: (mega_data.mega_nodes.len() * std::mem::size_of::<bvh::BVHNode>()) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });
        context.queue.write_buffer(&bvh_buffer, 0, bytemuck::cast_slice(&mega_data.mega_nodes));

        let instances_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            mapped_at_creation: false,
            size: (1024 * std::mem::size_of::<bvh::ModelInstance>()) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });        

        let mesh_bind_group = BindGroupS::from_bindings(&context, vec![
            mesh.get_storage_binding(),
            BindingS {
                entry_layout: wgpu::BindGroupLayoutEntry {
                    binding: 1, 
                    visibility: wgpu::ShaderStages::COMPUTE, 
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None
                },
                entry: wgpu::BindGroupEntry { binding: 1, resource: bvh_buffer.as_entire_binding() }
            },
            BindingS {
                entry_layout: wgpu::BindGroupLayoutEntry { 
                    binding: 2, 
                    visibility: wgpu::ShaderStages::COMPUTE, 
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None
                },
                entry: wgpu::BindGroupEntry { binding: 2, resource: instances_buffer.as_entire_binding() }
            }
        ]);

        let lut = TextureS::from_json(context, include_bytes!("../../../smq_proj/assets/lut.png"), include_str!("../../../smq_proj/assets/lut.png.json"));
        
        let comp_binding_2 = BindGroupS::from_bindings(&context, vec![
            camera_ubo.get_binding(ShaderStages::COMPUTE| ShaderStages::VERTEX),
            light_ubo.get_binding(ShaderStages::COMPUTE),
            lut.get_texture_binding(ShaderStages::COMPUTE),
            lut.get_sampler_binding(ShaderStages::COMPUTE)
        ]);
        
        let draw_bind_group = BindGroupS::from_bindings(&context, vec![
            dyn_ubo.get_binding(ShaderStages::VERTEX_FRAGMENT)
        ]);

        let texture_bind_group = BindGroupS::from_bindings(&context, vec![
            texture.get_texture_binding(ShaderStages::FRAGMENT),
            texture.get_sampler_binding(ShaderStages::FRAGMENT)
        ]);

        let mat = Material::from_json(&context, include_str!("../../../smq_proj/assets/basic.wgsl.json"), include_str!("../../../smq_proj/assets/basic.wgsl"), &[
            &draw_bind_group.bind_group_layout,
            &texture_bind_group.bind_group_layout
        ]);

        let comp_bind_group = BindGroupS::from_bindings(&context, vec![
            render_texture.attachments[0].get_texture_binding(),
            render_texture.attachments[1].get_texture_binding(),
            blit_render_texture.attachments[0].get_writible_texture_binding(),
            render_texture.get_depth_binding()
        ]);

        let comp = Material::compute(&context, include_str!("../../../smq_proj/assets/compute.wgsl"), &[
            &comp_bind_group.bind_group_layout,
            &comp_binding_2.bind_group_layout,
            &mesh_bind_group.bind_group_layout
        ]);    

        let blit_bind_group = BindGroupS::from_bindings(&context, vec![
            blit_render_texture.attachments[0].get_texture_binding(),
            blit_render_texture.attachments[0].get_sampler_binding()
        ]);

        let blit_pipeline = context.create_blit_pipeline(&blit_bind_group);
        
        return Self {  
            is_minimized,
            dyn_ubo,
            camera_ubo,
            render_texture,
            mat,
            draw_bind_group,
            comp,
            comp_bind_group,
            comp_binding_2,
            mesh_bind_group,
            blit_pipeline,
            blit_bind_group,
            mega_data,
            instances_buffer,
            light_ubo,
            meshes,
            texture_bind_group
        };
    }

    pub fn draw(&mut self, context: &mut Context, world: &mut World) {
        let full_output = context.end_egui_record();

        if self.is_minimized {
            return; 
        }

        let aspect_ratio = 16.0/9.0;

        let mut current_cam_pos = Vec3::ZERO;
        let mut camera_query = world.query::<&free_cam::FreeCamera>();
        let view = if let Some(cam) = camera_query.iter(&world).next() {

            context.window.set_cursor_visible(!cam.is_controlling);
            let grab_mode = if cam.is_controlling {
                winit::window::CursorGrabMode::Locked 
            } else {
                winit::window::CursorGrabMode::None
            };

            if let Err(_) = context.window.set_cursor_grab(grab_mode) {
                if cam.is_controlling {
                    let _ = context.window.set_cursor_grab(winit::window::CursorGrabMode::Confined);
                }
            }

            current_cam_pos = cam.position.clone();

            let q_yaw = Quat::from_axis_angle(Vec3::Y, cam.yaw.to_radians());
            let q_pitch = Quat::from_axis_angle(Vec3::X, cam.pitch.to_radians());
            let rotation = (q_yaw * q_pitch).normalize();
            let forward = rotation * Vec3::new(0.0, 0.0, -1.0);

            Mat4::look_at_rh(cam.position, cam.position + forward, Vec3::Y)
        } else {
            Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y)
        };

        let proj = Mat4::perspective_rh(
            f32::to_radians(60.0),
            aspect_ratio,
            50.0,
            0.1
        );

        let view_proj = proj * view;

        let mut instances_data = Vec::new();

        let mut objects_query = world.query::<&Position3D>();
        for (i, pos) in objects_query.iter(world).enumerate() {
            let model_matrix = Mat4::from_scale_rotation_translation(pos.size, pos.rotation, pos.position);
                instances_data.push(bvh::ModelInstance {
                inv_model: model_matrix.inverse(),
                bvh_root_node: self.mega_data.bvh_roots[pos.mesh as usize],
                id: pos.mesh as u32,
                _padding: [0; 2]
            });

            self.dyn_ubo.data[i].mvp = view_proj * model_matrix;
        }

        self.dyn_ubo.upload_data(&context);

        context.queue.write_buffer(&self.instances_buffer, 0, bytemuck::cast_slice(&instances_data));

        let mut light_query = world.query::<&PointLight>();
        let mut lights = 0;
        for (i, point_light) in light_query.iter(world).enumerate() {
            self.light_ubo.data.lights[i] = PointLightData {
                position_and_intensity: Vec4::new(point_light.position.x, point_light.position.y, point_light.position.z, point_light.power),
                color_and_radius: Vec4::new(point_light.color.r as f32, point_light.color.g as f32, point_light.color.b as f32, (point_light.power / 0.01).sqrt() * 4.0),
            };
            lights = i;
        }

        self.camera_ubo.data.inv_view_proj = view_proj.inverse();
        self.camera_ubo.data.camera_pos = current_cam_pos.extend(1.0);
        self.camera_ubo.data.light_count = lights as u32 + 1;
        self.camera_ubo.data.instance_count = instances_data.len() as u32;
        self.camera_ubo.upload_data(&context);

        self.light_ubo.upload_data(&context);

        if let Some(mut frame) = context.start_frame() {
            {
                let mut render_pass = context.get_render_pass(&mut frame, &self.render_texture);

                self.mat.set_render_material(&mut render_pass);

                render_pass.set_bind_group(1, &self.texture_bind_group.bind_group, &[]);

                for i in 0..instances_data.len() {
                    render_pass.set_vertex_buffer(0, self.meshes[instances_data[i].id as usize].buffer.slice(..));
                    render_pass.set_bind_group(0, &self.draw_bind_group.bind_group, &[(i*256) as u32]);

                    render_pass.draw(0..self.meshes[instances_data[i].id as usize].vertex_count, 0..1);
                }                
            }

            let groups_x = (384 + 7) / 8;
            let groups_y = (216 + 7) / 8;
            self.comp.dispach_compute(&context, groups_x, groups_y, 1, &mut frame, &[
                &self.comp_bind_group.bind_group,
                &self.comp_binding_2.bind_group,
                &self.mesh_bind_group.bind_group
            ]);
            
            context.blit_screen(&mut frame, &self.blit_pipeline, &self.blit_bind_group);

            context.draw_egui(&mut frame, full_output);

            context.end_frame(frame); 
        }
    }
}