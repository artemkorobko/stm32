use crate::{commands::helper, driver::prelude::*};

use super::executor::CommandExecutor;

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
        let device_mode = helper::open_by_serial(&self.driver, &self.serial)?.device_mode()?;
        match device_mode {
            DeviceMode::Fdu => log::info!("Device mode - DFU"),
            DeviceMode::Device => log::info!("Device mode - DEVICE"),
        }
        Ok(())
    }
}
