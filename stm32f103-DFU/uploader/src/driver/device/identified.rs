pub struct IdentifiedDevice {
    inner: rusb::Device<rusb::Context>,
    descriptor: rusb::DeviceDescriptor,
    handle: rusb::DeviceHandle<rusb::Context>,
    vendor: String,
    product: String,
    serial: String,
}

impl IdentifiedDevice {
    pub fn new(
        inner: rusb::Device<rusb::Context>,
        descriptor: rusb::DeviceDescriptor,
        handle: rusb::DeviceHandle<rusb::Context>,
        vendor: String,
        product: String,
        serial: String,
    ) -> Self {
        Self {
            inner,
            descriptor,
            handle,
            vendor,
            product,
            serial,
        }
    }

    pub fn vendor_id(&self) -> u16 {
        self.descriptor.vendor_id()
    }

    pub fn product_id(&self) -> u16 {
        self.descriptor.product_id()
    }

    pub fn vendor(&self) -> &str {
        &self.vendor
    }

    pub fn product(&self) -> &str {
        &self.product
    }

    pub fn serial_number(&self) -> &str {
        &self.serial
    }
}
