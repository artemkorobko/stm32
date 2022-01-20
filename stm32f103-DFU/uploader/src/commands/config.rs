use structopt::StructOpt;

#[derive(StructOpt)]
/// STM32 DFU Driver
pub enum Command {
    /// Print driver version
    Version,
    /// List connected devices
    Ls,
    /// Print firmware version
    Fv {
        /// Serial number
        serial: String,
    },
    /// Print device id
    Id {
        /// Serial number
        serial: String,
    },
    /// Print DFU flags
    Flags,
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
