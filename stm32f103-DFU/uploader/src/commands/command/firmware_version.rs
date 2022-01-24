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
        let version = helper::open_by_serial(&self.driver, &self.serial)?.firmware_version()?;
        log::info!(
            "USB device firmware version {}.{}.{}",
            version.major,
            version.minor,
            version.patch
        );
        Ok(())
    }
}
