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
        let device_id = helper::open_by_serial(&self.driver, &self.serial)?.device_id()?;
        log::info!(
            "USB device id {}-{}-{}-{}",
            device_id.id_0,
            device_id.id_1,
            device_id.id_2,
            device_id.id_3,
        );
        Ok(())
    }
}
