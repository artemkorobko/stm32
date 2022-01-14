use crate::driver::driver::Driver;

use super::command_executor::CommandExecutor;

pub struct CommandExecutorUpload {
    driver: Driver,
    source: String,
    target: String,
}

impl CommandExecutorUpload {
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

impl CommandExecutor for CommandExecutorUpload {
    fn exec(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
