use crate::driver::prelude::*;

use super::executor::CommandExecutor;

pub struct CommandReadFlags {
    driver: Driver,
}

impl CommandReadFlags {
    pub fn new(driver: Driver) -> Self {
        Self { driver }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }
}

impl CommandExecutor for CommandReadFlags {
    fn exec(&self) -> anyhow::Result<()> {
        todo!()
    }
}
