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

impl Material {
    pub fn crate_shader_module(context: &mut Context, code: &str) -> ShaderModule {
        return context.device.create_shader_module(ShaderModuleDescriptor { 
            label: None, 
            source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(code))
        });
    }

    pub fn get_default_vert_layout() -> VertexBufferLayout<'static> {
        return VertexBufferLayout {
            array_stride: 32,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: 12 + 8,
                    shader_location: 2,
                    format: VertexFormat::Float32x3,
                },
            ],
        };
    }

    pub fn new(
        context: &mut Context, 
        culling: Option<Face>, 
        label: Option<&str>, 
        render_texture: &RenderTexture, 
        module: ShaderModule, 
        vert_layout: VertexBufferLayout, 
        depth_compare: CompareFunction, 
        layout: [Option<&BindGroupS>; 4]
    ) -> Self {
        let mut attachments = Vec::new();

        for attachment in render_texture.attachments.iter() {
            attachments.push(Some(ColorTargetState {
                format: attachment.format,
                blend: None,
                write_mask: ColorWrites::ALL
            }));
        }       

        let mut layouts: Vec<&BindGroupLayout> = Vec::new();

        for lay in layout.iter() {
            if let Some(lays) = lay {
                layouts.push(&lays.bind_group_layout);
            }
        }

        let pipeline = context.device.create_render_pipeline(&RenderPipelineDescriptor {
            label, 
            layout: Some(&context.device.create_pipeline_layout(&PipelineLayoutDescriptor { 
                label, 
                bind_group_layouts: &layouts,
                push_constant_ranges: &[]
            })), 
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
                cull_mode: culling, 
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill, 
                conservative: false 
            }, 
            depth_stencil: if let Some(depth) = &render_texture.depth_texture {
                Some(DepthStencilState {
                    format: depth.2,
                    depth_write_enabled: true,
                    depth_compare,
                    stencil: StencilState::default(),
                    bias: DepthBiasState::default(),
                })
            } else { None },
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

    pub fn compute(context: &Context, layout: &[Option<BindGroupS>; 4], module: &ShaderModule, eantry_point: &str) -> Self {
        let mut layouts: Vec<&BindGroupLayout> = Vec::new();

        for lay in layout.iter() {
            if let Some(lays) = lay {
                layouts.push(&lays.bind_group_layout);
            }
        }

        let compute_pipeline = context.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&context.device.create_pipeline_layout(&PipelineLayoutDescriptor { 
                label: None, 
                bind_group_layouts: &layouts,
                push_constant_ranges: &[]
            })),
            module,
            entry_point: Some(eantry_point),
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
}
