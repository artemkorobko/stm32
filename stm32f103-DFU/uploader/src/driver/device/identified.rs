use anyhow::Context;

use crate::driver::endpoint::{Endpoint, EndpointConfig};

use super::opened::OpenedDevice;

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

    pub fn open(mut self) -> anyhow::Result<OpenedDevice> {
        let read_ep_config = self.find_bulk_endpoint(rusb::Direction::In)?;
        let read_ep = Endpoint::open(&mut self.handle, read_ep_config)?;
        let write_ep_config = self.find_bulk_endpoint(rusb::Direction::Out)?;
        let write_ep = Endpoint::open(&mut self.handle, write_ep_config)?;
        self.handle
            .reset()
            .with_context(|| format!("Can't reset USB device"))?;
        Ok(OpenedDevice::new(self.handle, read_ep, write_ep))
    }

    fn find_bulk_endpoint(&self, direction: rusb::Direction) -> anyhow::Result<EndpointConfig> {
        for config_index in 0..self.descriptor.num_configurations() {
            let config_descriptor = self
                .inner
                .config_descriptor(config_index)
                .with_context(|| format!("Can't read USB device config descriptor"))?;
            for interface in config_descriptor.interfaces() {
                for interface_descriptor in interface.descriptors() {
                    for endpoint_descriptor in interface_descriptor.endpoint_descriptors() {
                        if endpoint_descriptor.transfer_type() == rusb::TransferType::Bulk
                            && endpoint_descriptor.direction() == direction
                        {
                            return Ok(EndpointConfig::new(
                                config_descriptor.number(),
                                interface_descriptor.interface_number(),
                                interface_descriptor.setting_number(),
                                endpoint_descriptor.address(),
                            ));
                        }
                    }
                }
            }
        }

        match direction {
            rusb::Direction::In => anyhow::bail!("Can't find input bulk endpoint with direction"),
            rusb::Direction::Out => anyhow::bail!("Can't find output bulk endpoint with direction"),
        }
    }
}
