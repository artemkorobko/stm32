use std::time;

use anyhow::Context;

use super::{
    device_helper,
    endpoint_helper::{self, EndPointLookup},
};

pub struct EndPoint {
    inner: EndPointLookup,
    driver_detached: bool,
}

impl EndPoint {
    pub fn open<T>(
        handle: &mut rusb::DeviceHandle<T>,
        lookup: EndPointLookup,
    ) -> anyhow::Result<Self>
    where
        T: rusb::UsbContext,
    {
        let mut driver_detached = false;
        if endpoint_helper::is_kernel_driver_active(handle, lookup.interface)? {
            endpoint_helper::detach_kernel_driver(handle, lookup.interface)?;
            driver_detached = true;
        }

        device_helper::set_active_configuration(handle, lookup.configuration)?;
        endpoint_helper::claim_interface(handle, lookup.interface)?;
        endpoint_helper::set_alternate_setting(handle, lookup.interface, lookup.setting)?;

        Ok(Self {
            inner: lookup,
            driver_detached,
        })
    }

    pub fn read<T: rusb::UsbContext>(
        &mut self,
        handle: &rusb::DeviceHandle<T>,
        buf: &mut [u8],
        timeout: time::Duration,
    ) -> anyhow::Result<usize> {
        match self.inner.transfer_type {
            rusb::TransferType::Bulk => handle
                .read_bulk(self.inner.address, buf, timeout)
                .with_context(|| {
                    let address = self.inner.address;
                    format!("Unable to bulk read from address {}", address)
                }),
            rusb::TransferType::Interrupt => handle
                .read_interrupt(self.inner.address, buf, timeout)
                .with_context(|| {
                    let address = self.inner.address;
                    format!("Unable to interrupt read from address {}", address)
                }),
            _ => Err(anyhow::anyhow!(
                "Unsupported read transfer type {:?} for address {}",
                self.inner.transfer_type,
                self.inner.address,
            )),
        }
    }

    pub fn write<T: rusb::UsbContext>(
        &mut self,
        handle: &rusb::DeviceHandle<T>,
        buf: &[u8],
        timeout: time::Duration,
    ) -> anyhow::Result<usize> {
        match self.inner.transfer_type {
            rusb::TransferType::Bulk => handle
                .write_bulk(self.inner.address, buf, timeout)
                .with_context(|| {
                    let address = self.inner.address;
                    format!("Unable to bulk write to address {}", address)
                }),
            rusb::TransferType::Interrupt => handle
                .write_interrupt(self.inner.address, buf, timeout)
                .with_context(|| {
                    let address = self.inner.address;
                    format!("Unable to interrupt write to address {}", address)
                }),
            _ => Err(anyhow::anyhow!(
                "Unsupported write transfer type {:?} for address {}",
                self.inner.transfer_type,
                self.inner.address,
            )),
        }
    }

    pub fn close<T>(self, handle: &mut rusb::DeviceHandle<T>) -> anyhow::Result<()>
    where
        T: rusb::UsbContext,
    {
        if self.driver_detached {
            endpoint_helper::attach_kernel_driver(handle, self.inner.interface)?;
        }

        Ok(())
    }
}
