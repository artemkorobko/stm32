use crate::driver::prelude::*;

use super::command_executor::CommandExecutor;

pub struct CommandUpload {
    driver: Driver,
    source: String,
    target: String,
}

impl CommandUpload {
    pub fn new(driver: Driver, source: String, target: String) -> Self {
        Self {
            driver,
            source,
            target,
        }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }
}

impl CommandExecutor for CommandUpload {
    fn exec(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
