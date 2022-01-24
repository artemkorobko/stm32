use crate::{commands::helper, driver::prelude::*};

use super::executor::CommandExecutor;

pub struct CommandFirmwareVersion {
    driver: Driver,
    serial: String,
}

impl CommandFirmwareVersion {
    pub fn new(driver: Driver, serial: String) -> Self {
        Self { driver, serial }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }
}

impl CommandExecutor for CommandFirmwareVersion {
    fn exec(&self) -> anyhow::Result<()> {
        if let Some(device) = helper::find_by_serial(&self.driver, &self.serial)? {
            log::info!("Found USB device with serial {}", self.serial);
            let version = device.open()?.firmware_version()?;
            log::info!(
                "USB device firmware version {}.{}.{}",
                version.major,
                version.minor,
                version.patch
            );
        } else {
            log::error!("Can't find USB device with serial {}", self.serial);
        }
        Ok(())
    }
}
