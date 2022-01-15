use crate::driver::{device::Device, device_detector::DefaultDeviceDetector, driver::Driver};

use super::command_executor::CommandExecutor;

pub struct CommandFv {
    driver: Driver,
    serial: String,
}

impl CommandFv {
    pub fn new(driver: Driver, serial: String) -> Self {
        Self { driver, serial }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }

    fn read_firmware_version(&self, device: &mut Device) -> anyhow::Result<(u8, u8, u8)> {
        device.reset()?;
        Ok((0, 0, 0))
    }
}

impl CommandExecutor for CommandFv {
    fn exec(&self) -> anyhow::Result<()> {
        let device = self
            .driver
            .open_device(&DefaultDeviceDetector::boxed(), &self.serial)?;
        match device {
            Some(mut device) => {
                let (major, minor, patch) = self.read_firmware_version(&mut device)?;
                log::info!("Firmware version: {}.{}.{}", major, minor, patch);
            }
            None => log::error!("No device found matching serial {}", self.serial),
        }
        Ok(())
    }
}
