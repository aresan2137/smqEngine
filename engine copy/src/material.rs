use wgpu::*;

use crate::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default)]
pub struct MaterialJson {
    pub atachments: Vec<String>,
    pub culling: String,
}

pub enum PipelineType {
    Render(RenderPipeline),
    Compute(ComputePipeline),
}

pub struct Material {
    pub pipeline: PipelineType
}

#[allow(dead_code)]
impl Material {
    pub fn from_asset_creator_render(context: &Context, code: &str, layout: &[Option<BindGroupLayout>; 4], creator_data: AssetCreatorRenderWgsl) -> Self {
        let mut attachments = Vec::new();

        for attachment in json_data.atachments.iter() {
            let format = json_helper::str_to_texture_format(&attachment);

            attachments.push(Some(ColorTargetState {
                format: format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL
            }));
        }

        let vert_layout = wgpu::VertexBufferLayout {
            array_stride: 32,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 12 + 8,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        };

        let layout_full = &context.create_material_layout(layout);

        let module = context.device.create_shader_module(ShaderModuleDescriptor { 
            label: None, 
            source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(code))
        });

        let pipeline = context.device.create_render_pipeline(&RenderPipelineDescriptor { 
            label: None, 
            layout: Some(layout_full), 
            vertex: VertexState { 
                module: &module,
                entry_point: Some("vs_main"), 
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[vert_layout]
            }, 
            primitive: PrimitiveState { 
                topology: PrimitiveTopology::TriangleList, 
                strip_index_format: None, 
                front_face: FrontFace::Ccw, 
                cull_mode: json_helper::str_to_face_culling(&json_data.culling), 
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill, 
                conservative: false 
            }, 
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Greater,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState { 
                count: 1, 
                mask: !0, 
                alpha_to_coverage_enabled: false
            }, 
            fragment: Some(FragmentState { 
                module: &module, 
                entry_point: Some("fs_main"), 
                compilation_options: PipelineCompilationOptions::default(),
                targets: attachments.as_slice()
            }), 
            multiview: None, 
            cache: None 
        });

        return Self { 
            pipeline: PipelineType::Render(pipeline)
        };
    }

    pub fn compute(context: &Context, code: &str, layout: &[&BindGroupLayout]) -> Self {
        let cs_module = context.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(code.into()),
        });

        let compute_pipeline_layout = context.create_material_layout(layout);

        let compute_pipeline = context.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            module: &cs_module,
            entry_point: Some("cs_main"),
            compilation_options: Default::default(),
            cache: None
        });

        return Self { 
            pipeline: PipelineType::Compute(compute_pipeline)
        };
    }

    pub fn set_render_material(&self, render_pass: &mut RenderPass<'_>) {
        if let PipelineType::Render(render_pipeline) = &self.pipeline {
            render_pass.set_pipeline(render_pipeline);
        } else { panic!("using compute shader as render shader"); }
    }

    pub fn set_compute_material(&self, compute_pass: &mut ComputePass<'_>) {
        if let PipelineType::Compute(render_pipeline) = &self.pipeline {
            compute_pass.set_pipeline(render_pipeline);
        } else { panic!("using render shader as compute shader"); }
    }

    pub fn dispach_compute(&self, context: &Context, groups_x: u32, groups_y: u32, groups_z: u32, info: &mut FrameInfo, bind_groups: &[&BindGroup]) {
    {
        let mut compute_pass = context.get_compute_pass(info);

        self.set_compute_material(&mut compute_pass);

        for (i, &bind_group) in bind_groups.iter().enumerate() {
            compute_pass.set_bind_group(i as u32, bind_group, &[]);
        }

        compute_pass.dispatch_workgroups(groups_x, groups_y, groups_z);
    }
}
}
