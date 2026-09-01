use bevy_ecs::world::World;
use smq_engine::*;
use wgpu::*;

use crate::{code::FreeCamera, gen_bake};

use glam::*;

pub struct Renderer {
    gen_assets: gen_bake::GenAssets,
    mesh1: Mesh,
    mat1: Material,
    blitinfo: BlitInfo,
    glob_bind_1: BindGroupS,
    bind_group0: BindGroupS,
    bind_group1: BindGroupS,
    bind_group2: BindGroupS
}

impl Renderer {
    pub fn new(context: &mut Context) -> Self {
        let mut gen_assets = gen_bake::GenAssets::init_gen_assets(context);

        gen_assets.ubodata0.data.proj = Mat4::perspective_rh((60.0_f32).to_radians(), 16.0/9.0, 0.03, 500.0);

        gen_assets.ubodata0.upload_data(context);
        gen_assets.ubodata2.upload_data(context);

        let mesh1 = Mesh::new(
            context, include_bytes!("../../../smq_proj/data/file/mesh/Suzanne.smf"), 
            32, None, BufferUsages::COPY_DST | BufferUsages::VERTEX
        ).unwrap();

        let vert_layout = Material::get_default_vert_layout();

        let mat_module1 = Material::crate_shader_module(context, include_str!("../../../smq_proj/data/file/renderer/shaders/base.wgsl"));

        let bind_group0 = BindGroupS::new(context, &[
            gen_assets.ubodata0.get_binding(ShaderStages::VERTEX)
        ], None);

        let bind_group1 = BindGroupS::new(context, &[
            gen_assets.ubodata1.get_binding(ShaderStages::VERTEX)
        ], None);

        let bind_group2 = BindGroupS::new(context, &[
            gen_assets.ubodata2.get_binding(ShaderStages::VERTEX)
        ], None);

        let mat1 = Material::new(context, Some(Face::Back), None, &gen_assets.renderer_main_renderTexture, mat_module1, vert_layout, CompareFunction::Less, [
            Some(&bind_group0),
            Some(&bind_group1),
            Some(&bind_group2),
            None
        ]);

        let glob_bind_1 = BindGroupS::new(context, &[
            gen_assets.renderer_main_renderTexture.attachments[0].get_texture_binding(ShaderStages::FRAGMENT),
            gen_assets.renderer_main_renderTexture.attachments[0].get_sampler_binding(ShaderStages::FRAGMENT)
        ], None);

        let blitinfo = context.create_blit_pipeline(&glob_bind_1);

        return Self {
            gen_assets,
            mesh1,
            mat1,
            blitinfo,
            glob_bind_1,
            bind_group0,
            bind_group1,
            bind_group2
        };
    }

    fn prep_data(&mut self, context: &mut Context, world: &mut World) {
        let mut camera_query = world.query::<&FreeCamera>();

        if let Some(cam) = camera_query.iter(world).next() {
            let q_yaw = Quat::from_axis_angle(Vec3::Y, cam.yaw.to_radians());
            let q_pitch = Quat::from_axis_angle(Vec3::X, cam.pitch.to_radians());
            let rotation = (q_yaw * q_pitch).normalize();

            let forward = rotation * Vec3::new(0.0, 0.0, 1.0);
            let up = rotation * Vec3::new(0.0, 1.0, 0.0);

            let view = Mat4::look_to_lh(cam.position, forward, up);

            self.gen_assets.ubodata1.data.view = view;
            self.gen_assets.ubodata1.upload_data(context);
        }
    } 

    pub fn draw(&mut self, context: &mut Context, world: &mut World, full_output: egui::FullOutput) {
        let mut frame = context.start_frame().expect("failed to start frame");

        self.prep_data(context, world);

        {
            let mut render_pass = context.get_render_pass(&mut frame, &self.gen_assets.renderer_main_renderTexture);

            self.mat1.set_render_material(&mut render_pass);

            render_pass.set_vertex_buffer(0, self.mesh1.buffer.slice(..));

            render_pass.set_bind_group(0, &self.bind_group0.bind_group, &[]);
            render_pass.set_bind_group(1, &self.bind_group1.bind_group, &[]);
            render_pass.set_bind_group(2, &self.bind_group2.bind_group, &[]);

            render_pass.draw(0..self.mesh1.vertex_count, 0..1);
        }

        context.blit_screen(&mut frame, &self.blitinfo, &self.glob_bind_1);

        context.draw_egui(&mut frame, full_output);

        context.end_frame(frame);
    }
}

