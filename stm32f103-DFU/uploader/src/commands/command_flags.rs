use crate::driver::prelude::*;

use super::command_executor::CommandExecutor;

pub struct CommandFlags {
    driver: Driver,
}

impl CommandFlags {
    pub fn new(driver: Driver) -> Self {
        Self { driver }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }
}

impl CommandExecutor for CommandFlags {
    fn exec(&self) -> anyhow::Result<()> {
        todo!()
    }
}
