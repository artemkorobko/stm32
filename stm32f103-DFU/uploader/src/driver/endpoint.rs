use std::time;

use anyhow::Context;
pub struct EndpointConfig {
    configuration: u8,
    interface: u8,
    setting: u8,
    address: u8,
}

impl EndpointConfig {
    pub fn new(configuration: u8, interface: u8, setting: u8, address: u8) -> Self {
        Self {
            configuration,
            interface,
            setting,
            address,
        }
    }
}

pub struct Endpoint {
    config: EndpointConfig,
    driver_detached: bool,
}

impl Endpoint {
    pub fn open(
        handle: &mut rusb::DeviceHandle<rusb::Context>,
        config: EndpointConfig,
    ) -> anyhow::Result<Self> {
        let mut driver_detached = false;
        if Self::is_kernel_driver_active(handle, config.interface)? {
            Self::detach_kernel_driver(handle, config.interface)?;
            driver_detached = true;
        }

        Self::set_active_configuration(handle, config.configuration)?;
        Self::claim_interface(handle, config.interface)?;
        Self::set_alternate_setting(handle, config.interface, config.setting)?;

        Ok(Self {
            config,
            driver_detached,
        })
    }

    pub fn read(
        &mut self,
        handle: &rusb::DeviceHandle<rusb::Context>,
        buf: &mut [u8],
        timeout: time::Duration,
    ) -> anyhow::Result<usize> {
        handle
            .read_bulk(self.config.address, buf, timeout)
            .with_context(|| format!("Unable to bulk read from address {}", self.config.address))
    }

    pub fn write(
        &mut self,
        handle: &rusb::DeviceHandle<rusb::Context>,
        buf: &[u8],
        timeout: time::Duration,
    ) -> anyhow::Result<usize> {
        handle
            .write_bulk(self.config.address, buf, timeout)
            .with_context(|| format!("Unable to bulk write to address {}", self.config.address))
    }

    pub fn close(&mut self, handle: &mut rusb::DeviceHandle<rusb::Context>) -> anyhow::Result<()> {
        if self.driver_detached {
            Self::attach_kernel_driver(handle, self.config.interface)?;
        }

        Ok(())
    }

    fn is_kernel_driver_active(
        handle: &rusb::DeviceHandle<rusb::Context>,
        interface: u8,
    ) -> anyhow::Result<bool> {
        handle.kernel_driver_active(interface).with_context(|| {
            format!(
                "Unable to verify if kernel driver is attached to interface {}",
                interface
            )
        })
    }

    fn detach_kernel_driver(
        handle: &mut rusb::DeviceHandle<rusb::Context>,
        interface: u8,
    ) -> anyhow::Result<()> {
        handle.detach_kernel_driver(interface).with_context(|| {
            format!(
                "Unable to detach kernel driver from interface {}",
                interface
            )
        })
    }

    fn attach_kernel_driver(
        handle: &mut rusb::DeviceHandle<rusb::Context>,
        interface: u8,
    ) -> anyhow::Result<()> {
        handle
            .attach_kernel_driver(interface)
            .with_context(|| format!("Unable to attach kernel driver to interface {}", interface))
    }

    fn set_active_configuration(
        handle: &mut rusb::DeviceHandle<rusb::Context>,
        configuration: u8,
    ) -> anyhow::Result<()> {
        handle
            .set_active_configuration(configuration)
            .with_context(|| format!("Unable to set active configuration {}", configuration))
    }

    fn claim_interface(
        handle: &mut rusb::DeviceHandle<rusb::Context>,
        interface: u8,
    ) -> anyhow::Result<()> {
        handle
            .claim_interface(interface)
            .with_context(|| format!("Unable to claim interface {}", interface))
    }

    pub fn set_alternate_setting(
        handle: &mut rusb::DeviceHandle<rusb::Context>,
        interface: u8,
        setting: u8,
    ) -> anyhow::Result<()> {
        handle
            .set_alternate_setting(interface, setting)
            .with_context(|| {
                format!(
                    "Unable to set alternate setting {} to interface {}",
                    setting, interface
                )
            })
    }
}
