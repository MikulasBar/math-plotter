use iced::widget::shader::wgpu::{
    self, Color, CommandEncoder, LoadOp, Operations, RenderPass, RenderPassTimestampWrites, StoreOp,
};

#[derive(Default)]
pub struct RenderPassBuilder<'a> {
    label: Option<&'a str>,
    color_attachment: Option<wgpu::RenderPassColorAttachment<'a>>,
    depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    timestamp_writes: Option<RenderPassTimestampWrites<'a>>,
    occlusion_query_set: Option<&'a wgpu::QuerySet>,
}


impl<'a> RenderPassBuilder<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn color_attachment(
        mut self,
        view: &'a wgpu::TextureView,
        // resolve_target: Option<&'a wgpu::TextureView>,
        load: LoadOp<Color>,
        store: StoreOp,
    ) -> Self {
        self.color_attachment = Some(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: Operations { load, store },
        });
        self
    }

    pub fn build(self, encoder: &'a mut CommandEncoder) -> RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: self.label,
            color_attachments: &[self.color_attachment],
            depth_stencil_attachment: self.depth_stencil_attachment,
            timestamp_writes: self.timestamp_writes,
            occlusion_query_set: self.occlusion_query_set,
        })
    }
}
