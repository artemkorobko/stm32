use structopt::StructOpt;

#[derive(StructOpt)]
/// STM32 DFU Driver
pub enum Command {
    /// Print driver version
    Version,
    /// List connected devices
    Ls,
    /// Read firmware version
    Rv {
        /// Serial number
        serial: String,
    },
    /// Read device id
    Rd {
        /// Serial number
        serial: String,
    },
    /// Read device mode
    Rm {
        /// Serial number
        serial: String,
    },
    /// Read device memory layout
    Rml {
        /// Serial number
        serial: String,
    },
    /// Read DFU flags
    Rf {
        /// Serial number
        serial: String,
    },
    /// Reset DFU flags
    Rdf {
        /// Serial number
        serial: String,
    },
    /// Upload firmware
    Uf {
        /// Firmware to upload path
        #[structopt(short, long)]
        source: String,
        /// Target board serial number
        #[structopt(short, long)]
        target: String,
    },
}
