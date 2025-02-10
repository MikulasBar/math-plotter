use bytemuck::NoUninit;
use iced::widget::shader::wgpu::{
    self, BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingType, Buffer,
    BufferBindingType, BufferUsages, Device, ShaderStages,
};


pub struct BindGroupBuilder<'a> {
    device: &'a Device,
    label: &'a str,
    layout_entries: Vec<BindGroupLayoutEntry>,

    // This is a temporary storage for bindgroup entries
    // we cannot use Vec<BindGroupEntry> because it will complain about lifetimes
    entry_holders: Vec<EntryHolder>,
}

impl<'a> BindGroupBuilder<'a> {
    pub fn new(device: &'a Device, label: &'a str) -> Self {
        Self {
            device,
            label,
            layout_entries: Vec::new(),
            entry_holders: Vec::new(),
        }
    }

    pub fn add_entry<T>(
        mut self,
        label: &'a str,
        binding: u32,
        usage: BufferUsages,
        visibility: ShaderStages,
        count: Option<u32>,
        contents: &'a [T],
    ) -> Self
    where
        T: NoUninit,
    {
        let layout = BindGroupLayoutEntry {
            binding,
            visibility,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: count.map(|c| c.try_into().unwrap()),
        };

        let buffer = super::buffer_init(&self.device, label, usage, contents);
        let entry_holder = EntryHolder { binding, buffer };

        self.layout_entries.push(layout);
        self.entry_holders.push(entry_holder);
        self
    }

    fn build_layout(&self) -> BindGroupLayout {
        self.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(format!("{}-layout", self.label).as_str()),
                entries: &self.layout_entries,
            })
    }

    fn build_entries(&self) -> Vec<BindGroupEntry> {
        self.entry_holders
            .iter()
            .map(|holder| BindGroupEntry {
                binding: holder.binding,
                resource: holder.buffer.as_entire_binding(),
            })
            .collect()
    }

    pub fn build(self) -> (BindGroup, BindGroupLayout) {
        let layout = self.build_layout();
        let entries = self.build_entries();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(self.label),
            layout: &layout,
            entries: &entries,
        });

        (bind_group, layout)
    }
}

struct EntryHolder {
    pub binding: u32,
    pub buffer: Buffer,
}
