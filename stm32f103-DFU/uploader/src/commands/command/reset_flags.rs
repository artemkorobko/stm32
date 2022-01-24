use crate::{commands::helper, driver::prelude::*};

use super::executor::CommandExecutor;

pub struct CommandResetFlags {
    driver: Driver,
    serial: String,
}

impl CommandResetFlags {
    pub fn new(driver: Driver, serial: String) -> Self {
        Self { driver, serial }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }
}

impl CommandExecutor for CommandResetFlags {
    fn exec(&self) -> anyhow::Result<()> {
        let successful = helper::open_by_serial(&self.driver, &self.serial)?.dfu_reset_flags()?;
        if successful {
            log::info!("Dfu flags successfully reset");
        } else {
            log::error!("Dfu flags has not been reset");
        }
        Ok(())
    }
}
