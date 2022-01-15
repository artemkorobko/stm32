use anyhow::Context;

pub struct Metadata {
    pub vid: u16,
    pub pid: u16,
    pub vendor: String,
    pub product: String,
    pub serial: String,
}

pub struct Device {
    inner: rusb::Device<rusb::Context>,
    handle: rusb::DeviceHandle<rusb::Context>,
    meta: Metadata,
}

impl Device {
    pub fn new(
        inner: rusb::Device<rusb::Context>,
        handle: rusb::DeviceHandle<rusb::Context>,
        meta: Metadata,
    ) -> Self {
        Self {
            inner,
            handle,
            meta,
        }
    }

    pub fn metadata(&self) -> &Metadata {
        &self.meta
    }

    pub fn reset(&mut self) -> anyhow::Result<()> {
        self.handle
            .reset()
            .with_context(|| format!("Unable to reset device {}", self.metadata().serial))
    }
}
