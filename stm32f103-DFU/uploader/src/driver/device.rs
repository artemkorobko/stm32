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
}
