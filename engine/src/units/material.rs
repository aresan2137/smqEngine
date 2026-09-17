use wgpu::*;

use crate::*;

pub struct RenderMaterial {
    pub pipeline: RenderPipeline
}

pub fn crate_shader_module<D>(context: &mut Context<D>, code: &str) -> ShaderModule {
    return context.holding.as_ref().unwrap().device.create_shader_module(ShaderModuleDescriptor { 
        label: None, 
        source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(code))
    });
}

pub fn get_default_vert_layout() -> Option<VertexBufferLayout<'static>> {
        return Some(VertexBufferLayout {
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
        });
    }

impl RenderMaterial {
    pub fn new<D>(
        context: &mut Context<D>, 
        culling: Option<Face>, 
        label: Option<&str>, 
        render_texture: &RenderTexture, 
        module: &ShaderModule, 
        vert_layout: Option<VertexBufferLayout>, 
        depth_compare: Option<CompareFunction>, 
        layout: &PipelineLayout
    ) -> Self {
        let holding = context.holding.as_mut().unwrap();
        let mut attachments = Vec::new();

        for attachment in render_texture.attachments.iter() {
            attachments.push(Some(ColorTargetState {
                format: attachment.format,
                blend: None,
                write_mask: ColorWrites::ALL
            }));
        }       

        let depth_stencil = if let Some(depth_data) = &render_texture.depth_texture {
            Some(DepthStencilState {
                format: depth_data.2,
                depth_write_enabled: Some(true), 
                depth_compare,
                stencil: StencilState::default(),
                bias: DepthBiasState::default()
            })
        } else {
            None
        };

        let pipeline = holding.device.create_render_pipeline(&RenderPipelineDescriptor { 
            label,
            layout: Some(&layout),
            vertex: VertexState {
                module: module,
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
            depth_stencil, 
            multisample: MultisampleState {
                count: 1, 
                mask: !0, 
                alpha_to_coverage_enabled: false 
            }, 
            fragment: Some(FragmentState { 
                module: module,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &attachments
            }), 
            multiview_mask: None, 
            cache: None
        });

        return Self { 
            pipeline
        };
    }

    pub fn set(&self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
    }
}

pub fn compute<D>(context: &Context<D>, layout: &[Option<&BindGroupS>; 4], module: &ShaderModule, eantry_point: &str, label: Option<&str>) -> ComputePipeline {
    let holding = context.holding.as_ref().unwrap();

    let mut layouts: [Option<&BindGroupLayout>; 4] = [None, None, None, None];

    if let Some(lays) = layout[0] {
        layouts[0] = Some(&lays.bind_group_layout);
    }
    if let Some(lays) = layout[1] {
        layouts[1] = Some(&lays.bind_group_layout);
    }
    if let Some(lays) = layout[2] {
        layouts[2] = Some(&lays.bind_group_layout);
    }
    if let Some(lays) = layout[3] {
        layouts[3] = Some(&lays.bind_group_layout);
    }

    let mat1layout = holding.device.create_pipeline_layout(&PipelineLayoutDescriptor { 
        label, 
        bind_group_layouts: &[
            layouts[0],
            layouts[1],
            layouts[2],
            layouts[3],
        ],
        immediate_size: 0
    });

    return holding.device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: None,
        layout: Some(&mat1layout),
        module,
        entry_point: Some(eantry_point),
        compilation_options: Default::default(),
        cache: None
    });
}

