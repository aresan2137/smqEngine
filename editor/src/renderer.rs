use bevy_ecs::world::World;
use smq_engine::*;
use wgpu::*;

use crate::{free_cam::FreeCamera, gen_bake};

use glam::*;

pub struct Renderer {
    pub gen_assets: gen_bake::GenAssets,
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

        gen_assets.renderer_ubodata0_ubo.data.proj = Mat4::perspective_rh((60.0_f32).to_radians(), 16.0/9.0, 0.03, 500.0);

        gen_assets.renderer_ubodata0_ubo.upload_data(context);
        gen_assets.renderer_ubodata2_ubo.upload_data(context);

        let mesh1 = Mesh::new(
            context, include_bytes!("../../smq_proj/data/file/mesh/Suzanne.smf"), 
            32, None, BufferUsages::COPY_DST | BufferUsages::VERTEX
        ).unwrap();

        let vert_layout = Material::get_default_vert_layout();

        let mat_module1 = Material::crate_shader_module(context, "
struct UboData0 {
    proj: mat4x4f
}

struct UboData1 {
    view: mat4x4f
}

struct UboData2 {
    model: mat4x4f
}

@group(0) @binding(0) var<uniform> data0: UboData0;

@group(1) @binding(0) var<uniform> data1: UboData1;

@group(2) @binding(0) var<uniform> data2: UboData2;

struct VertexData {
    @location(0) position: vec3f,
    @location(2) normal: vec3f
}

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) uv: vec2f,
    @location(1) normal: vec3f
}

@vertex
fn vs_main(in: VertexData) -> VertexOutput {
    var out: VertexOutput;

    out.pos = data0.proj * data1.view * data2.model * vec4f(in.position, 1.0);
    out.normal = in.normal;

    return out;
}

const light = vec3f(5.0, 5.0, 3.0);

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let dit = dot(normalize(in.normal), normalize(light));
    let lit = max(dit, 0.0);

    return vec4f(lit, lit, lit, 1.0); 
}");

        let bind_group0 = BindGroupS::new(context, &[
            gen_assets.renderer_ubodata0_ubo.get_binding(ShaderStages::VERTEX)
        ], None);

        let bind_group1 = BindGroupS::new(context, &[
            gen_assets.renderer_ubodata1_ubo.get_binding(ShaderStages::VERTEX)
        ], None);

        let bind_group2 = BindGroupS::new(context, &[
            gen_assets.renderer_ubodata2_ubo.get_binding(ShaderStages::VERTEX)
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

            self.gen_assets.renderer_ubodata1_ubo.data.view = view;
            self.gen_assets.renderer_ubodata1_ubo.upload_data(context);
        }
    } 

    pub fn draw(&mut self, context: &mut Context, world: &mut World, full_output: egui::FullOutput) {

        self.prep_data(context, world);

        if let Ok(mut frame) = context.start_frame() {  
            {
                let mut render_pass = context.get_render_pass(&mut frame, &self.gen_assets.renderer_main_renderTexture);

                self.mat1.set_render_material(&mut render_pass);

                render_pass.set_vertex_buffer(0, self.mesh1.buffer.slice(..));

                render_pass.set_bind_group(0, &self.bind_group0.bind_group, &[]);
                render_pass.set_bind_group(1, &self.bind_group1.bind_group, &[]);
                render_pass.set_bind_group(2, &self.bind_group2.bind_group, &[]);

                render_pass.draw(0..self.mesh1.vertex_count, 0..1);
            }
                        
            context.draw_egui(&mut frame, full_output);

            context.end_frame(frame);
        }       
    }
}

