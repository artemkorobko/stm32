use crate::driver::driver::Driver;

use super::command_executor::CommandExecutor;

pub struct CommandExecutorVersion;

impl CommandExecutorVersion {
    pub fn boxed() -> Box<dyn CommandExecutor> {
        Box::new(Self {})
    }
}

impl CommandExecutor for CommandExecutorVersion {
    fn exec(&self) -> anyhow::Result<()> {
        let ver = Driver::version();
        log::info!("Driver version: {}.{}.{}", ver.major, ver.minor, ver.patch);
        Ok(())
    }
}
