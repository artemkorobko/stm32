use commands::command::{
    firmware_version::CommandFirmwareVersion, list::CommandList,
    device_id::CommandDeviceId, device_mode::CommandDeviceMode, executor::CommandExecutor,
    read_flags::CommandReadFlags, upload::CommandUpload, version::CommandVersion,
};
use commands::config::Command;
use structopt::StructOpt;

use crate::driver::driver::Driver;

mod commands;
mod driver;

fn main() -> anyhow::Result<()> {
    init_log_system();
    create_command_executor(Command::from_args())?.exec()
}

fn init_log_system() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
}

fn create_command_executor(command: Command) -> anyhow::Result<Box<dyn CommandExecutor>> {
    let executor: Box<dyn CommandExecutor> = match command {
        Command::Version => CommandVersion::boxed(),
        Command::Ls => CommandList::new(Driver::new()?).boxed(),
        Command::Rv { serial } => CommandFirmwareVersion::new(Driver::new()?, serial).boxed(),
        Command::Rd { serial } => CommandDeviceId::new(Driver::new()?, serial).boxed(),
        Command::Rm { serial } => CommandDeviceMode::new(Driver::new()?, serial).boxed(),
        Command::Rf { serial } => CommandReadFlags::new(Driver::new()?, serial).boxed(),
        Command::Uf { source, target } => {
            CommandUpload::new(Driver::new()?, source, target).boxed()
        }
    };
    Ok(executor)
}
