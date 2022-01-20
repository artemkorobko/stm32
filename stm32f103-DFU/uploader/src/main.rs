use commands::{
    command_device_id::CommandDeviceId, command_device_mode::CommandDeviceMode,
    command_executor::CommandExecutor, command_firmware_version::CommandFirmwareVersion,
    command_list::CommandList, command_read_flags::CommandReadFlags, command_upload::CommandUpload,
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
        Command::Rv { serial } => CommandFirmwareVersion::new(Driver::new()?, serial).boxed(),
        Command::Rd { serial } => CommandDeviceId::new(Driver::new()?, serial).boxed(),
        Command::Rm { serial } => CommandDeviceMode::new(Driver::new()?, serial).boxed(),
        Command::Rf => CommandReadFlags::new(Driver::new()?).boxed(),
        Command::Uf { source, target } => {
            CommandUpload::new(Driver::new()?, source, target).boxed()
        }
    };
    Ok(executor)
}
