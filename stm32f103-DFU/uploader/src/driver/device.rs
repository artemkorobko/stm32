use anyhow::Context;

use super::{endpoint::EndPoint, endpoint_helper};

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
    descriptor: rusb::DeviceDescriptor,
    meta: Metadata,
}

impl Device {
    pub fn new(
        inner: rusb::Device<rusb::Context>,
        handle: rusb::DeviceHandle<rusb::Context>,
        descriptor: rusb::DeviceDescriptor,
        meta: Metadata,
    ) -> Self {
        Self {
            inner,
            handle,
            descriptor,
            meta,
        }
    }

    pub fn metadata(&self) -> &Metadata {
        &self.meta
    }

    pub fn writer(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn open(mut self) -> anyhow::Result<OpenedDevice> {
        self.reset()?;
        let readable_endpoint = self.find_readable_endpoint()?;
        let readable_endpoint = EndPoint::open(&mut self.handle, readable_endpoint)?;
        let writeable_endpoint = self.find_writeable_endpoint()?;
        let writeable_endpoint = EndPoint::open(&mut self.handle, writeable_endpoint)?;
        Ok(OpenedDevice {
            inner: self.inner,
            handle: self.handle,
            readable_endpoint,
            writeable_endpoint,
        })
    }

    fn reset(&mut self) -> anyhow::Result<()> {
        self.handle
            .reset()
            .with_context(|| format!("Unable to reset device {}", self.metadata().serial))
    }

    fn find_readable_endpoint(&mut self) -> anyhow::Result<endpoint_helper::EndPointLookup> {
        self.find_readable_typed_endpoint(rusb::TransferType::Bulk)
            .or_else(|| self.find_readable_typed_endpoint(rusb::TransferType::Interrupt))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Unable to find readable endpoint for device {}",
                    self.meta.serial
                )
            })
    }

    fn find_readable_typed_endpoint(
        &mut self,
        transfer_type: rusb::TransferType,
    ) -> Option<endpoint_helper::EndPointLookup> {
        endpoint_helper::find_endpoint(
            &mut self.inner,
            &self.descriptor,
            transfer_type,
            endpoint_helper::Direction::Input,
        )
    }

    fn find_writeable_endpoint(&mut self) -> anyhow::Result<endpoint_helper::EndPointLookup> {
        self.find_writeable_typed_endpoint(rusb::TransferType::Bulk)
            .or_else(|| self.find_writeable_typed_endpoint(rusb::TransferType::Interrupt))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Unable to find writeable endpoint for device {}",
                    self.meta.serial
                )
            })
    }

    fn find_writeable_typed_endpoint(
        &mut self,
        transfer_type: rusb::TransferType,
    ) -> Option<endpoint_helper::EndPointLookup> {
        endpoint_helper::find_endpoint(
            &mut self.inner,
            &self.descriptor,
            transfer_type,
            endpoint_helper::Direction::Output,
        )
    }
}

pub struct OpenedDevice {
    inner: rusb::Device<rusb::Context>,
    handle: rusb::DeviceHandle<rusb::Context>,
    readable_endpoint: EndPoint,
    writeable_endpoint: EndPoint,
}
