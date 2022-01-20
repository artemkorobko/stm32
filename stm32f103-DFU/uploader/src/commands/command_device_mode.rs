use crate::driver::prelude::*;

use super::{command_executor::CommandExecutor, command_helper};

pub struct CommandDeviceMode {
    driver: Driver,
    serial: String,
}

impl CommandDeviceMode {
    pub fn new(driver: Driver, serial: String) -> Self {
        Self { driver, serial }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }
}

impl CommandExecutor for CommandDeviceMode {
    fn exec(&self) -> anyhow::Result<()> {
        if let Some(device) = command_helper::find_by_serial(&self.driver, &self.serial)? {
            log::info!("Found USB device with serial {}", self.serial);
            let device_mode = device.open()?.device_mode()?;
            match device_mode {
                DeviceMode::Fdu => log::info!("Device mode - DFU"),
                DeviceMode::Device => log::info!("Device mode - DEVICE"),
            }
        } else {
            log::error!("Can't find USB device with serial {}", self.serial);
        }
        Ok(())
    }
}
