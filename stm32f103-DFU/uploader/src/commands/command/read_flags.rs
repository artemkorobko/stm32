use crate::{
    commands::helper,
    driver::{prelude::*, protocol::dfu::DfuProtocol},
};

use super::executor::CommandExecutor;

pub struct CommandReadFlags {
    driver: Driver,
    serial: String,
}

impl CommandReadFlags {
    pub fn new(driver: Driver, serial: String) -> Self {
        Self { driver, serial }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }
}

impl CommandExecutor for CommandReadFlags {
    fn exec(&self) -> anyhow::Result<()> {
        let flags = helper::open_by_serial(&self.driver, &self.serial)?.dfu_read_flags()?;
        match flags {
            Some(flags) => {
                log::info!("Flash count {}", flags.flash_count);
            }
            None => log::info!("USB device has no flags set"),
        }
        Ok(())
    }
}
