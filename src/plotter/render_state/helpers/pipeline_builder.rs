use std::num::NonZeroU32;

use iced::widget::shader::wgpu::{
    self, BindGroupLayout, ColorTargetState, Device, FragmentState, PipelineLayoutDescriptor,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
    VertexBufferLayout, VertexState,
};


#[derive(Default)]
pub struct PipelineBuilder<'a> {
    label: Option<&'a str>,
    layout: Option<wgpu::PipelineLayout>,
    vertex: Option<VertexState<'a>>,
    primitive: Option<wgpu::PrimitiveState>,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: Option<wgpu::MultisampleState>,
    fragment: Option<wgpu::FragmentState<'a>>,
    multiview: Option<NonZeroU32>,

    device: Option<&'a Device>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            device: Some(device),
            ..Default::default()
        }
    }

    fn device(&self) -> &'a Device {
        self.device.unwrap()
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn layout(mut self, label: &'a str, bind_group_layouts: &[&BindGroupLayout]) -> Self {
        self.layout = Some(
            self.device()
                .create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some(label),
                    bind_group_layouts,
                    push_constant_ranges: &[],
                }),
        );
        self
    }

    pub fn vertex(
        mut self,
        module: &'a ShaderModule,
        entry_point: &'a str,
        buffers: &'a [VertexBufferLayout],
    ) -> Self {
        self.vertex = Some(VertexState {
            module,
            entry_point,
            buffers,
        });
        self
    }

    pub fn primitive(mut self, topology: PrimitiveTopology) -> Self {
        self.primitive = Some(PrimitiveState {
            topology,
            ..Default::default()
        });
        self
    }

    pub fn depth_stencil(mut self, depth_stencil: wgpu::DepthStencilState) -> Self {
        self.depth_stencil = Some(depth_stencil);
        self
    }

    pub fn multisample(mut self, multisample: wgpu::MultisampleState) -> Self {
        self.multisample = Some(multisample);
        self
    }

    pub fn fragment(
        mut self,
        module: &'a ShaderModule,
        entry_point: &'a str,
        targets: &'a [Option<ColorTargetState>],
    ) -> Self {
        self.fragment = Some(FragmentState {
            module,
            entry_point,
            targets,
        });
        self
    }

    pub fn multiview(mut self, multiview: NonZeroU32) -> Self {
        self.multiview = Some(multiview);
        self
    }

    pub fn build(self) -> RenderPipeline {
        self.device().create_render_pipeline(&RenderPipelineDescriptor {
            label: self.label,
            layout: self.layout.as_ref(),
            vertex: self.vertex.unwrap(),
            primitive: self.primitive.unwrap(),
            depth_stencil: self.depth_stencil,
            multisample: self.multisample.unwrap(),
            fragment: self.fragment,
            multiview: self.multiview,
        })
    }
}
