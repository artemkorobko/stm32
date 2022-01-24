use crate::{commands::helper, driver::prelude::*};

use super::executor::CommandExecutor;

pub struct CommandDeviceId {
    driver: Driver,
    serial: String,
}

impl CommandDeviceId {
    pub fn new(driver: Driver, serial: String) -> Self {
        Self { driver, serial }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }
}

impl CommandExecutor for CommandDeviceId {
    fn exec(&self) -> anyhow::Result<()> {
        if let Some(device) = helper::find_by_serial(&self.driver, &self.serial)? {
            log::info!("Found USB device with serial {}", self.serial);
            let device_id = device.open()?.device_id()?;
            log::info!(
                "USB device id {}-{}-{}-{}",
                device_id.id_0,
                device_id.id_1,
                device_id.id_2,
                device_id.id_3,
            );
        } else {
            log::error!("Can't find USB device with serial {}", self.serial);
        }
        Ok(())
    }
}
