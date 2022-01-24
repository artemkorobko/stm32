use crate::{commands::helper, driver::prelude::*};

use super::executor::CommandExecutor;

pub struct CommandMemoryLayout {
    driver: Driver,
    serial: String,
}

impl CommandMemoryLayout {
    pub fn new(driver: Driver, serial: String) -> Self {
        Self { driver, serial }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }
}

impl CommandExecutor for CommandMemoryLayout {
    fn exec(&self) -> anyhow::Result<()> {
        let layout = helper::open_by_serial(&self.driver, &self.serial)?.dfu_memory_layout()?;
        log::info!(
            "Flash size: {}Kb ({} bytes)",
            layout.flash_size / 1024,
            layout.flash_size
        );
        log::info!("Flash start: {0:#X} ({0})", layout.flash_start);
        log::info!(
            "Flash max: {0:#X} ({0})",
            layout.flash_start + layout.flash_size
        );
        log::info!("Flash end: {0:#X} ({0})", layout.flash_end);
        Ok(())
    }
}
