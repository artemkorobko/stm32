use structopt::StructOpt;

#[derive(StructOpt)]
/// STM32 DFU Driver
pub enum Command {
    /// Print driver version
    Version,
    /// List connected devices
    Ls,
    /// Upload firmware
    Upload {
        /// Firmware to upload path
        #[structopt(short, long)]
        source: String,
        /// Target board serial number
        #[structopt(short, long)]
        target: String,
    },
}
