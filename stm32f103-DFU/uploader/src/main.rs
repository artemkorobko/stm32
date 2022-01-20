use commands::{
    command_device_id::CommandDeviceId, command_device_mode::CommandDeviceMode,
    command_executor::CommandExecutor, command_firmware_version::CommandFirmwareVersion,
    command_flags::CommandFlags, command_list::CommandList, command_upload::CommandUpload,
    command_version::CommandVersion, config::Command,
};
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
        Command::Fv { serial } => CommandFirmwareVersion::new(Driver::new()?, serial).boxed(),
        Command::Id { serial } => CommandDeviceId::new(Driver::new()?, serial).boxed(),
        Command::Mode { serial } => CommandDeviceMode::new(Driver::new()?, serial).boxed(),
        Command::Flags => CommandFlags::new(Driver::new()?).boxed(),
        Command::Upload { source, target } => {
            CommandUpload::new(Driver::new()?, source, target).boxed()
        }
    };
    Ok(executor)
}
