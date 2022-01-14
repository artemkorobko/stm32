use commands::{
    command_executor::CommandExecutor, command_list::CommandExecutorList,
    command_upload::CommandExecutorUpload, command_version::CommandExecutorVersion,
    config::Command,
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
        Command::Version => CommandExecutorVersion::boxed(),
        Command::Ls => CommandExecutorList::new(Driver::new()?).boxed(),
        Command::Upload { source, target } => {
            CommandExecutorUpload::new(Driver::new()?, source, target).boxed()
        }
    };
    Ok(executor)
}
