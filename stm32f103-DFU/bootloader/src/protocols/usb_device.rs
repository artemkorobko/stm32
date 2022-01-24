use stm32f1xx_hal::flash;
use usb_device::UsbError;

use crate::{dfu, drivers::cdc_acm::Device};

use super::octet::{OctetHi, OctetLo};

pub enum Inbound {
    Version,
    DeviceId,
    Mode,
    MemoryLayout,
    ReadDfuFlags,
    ResetDfuFlags,
    Unknown,
}

pub trait Reader {
    fn read_inbound(&mut self) -> Result<Inbound, UsbError>;
    fn poll_read_inbound(&mut self, handler: impl FnOnce(Inbound)) -> Result<(), UsbError>;
}

impl Reader for Device {
    fn read_inbound(&mut self) -> Result<Inbound, UsbError> {
        let mut buf = [0u8; 64];
        self.read(&mut buf)?;
        let inbound = match buf[0] {
            0 => Inbound::Version,
            1 => Inbound::DeviceId,
            2 => Inbound::Mode,
            3 => Inbound::MemoryLayout,
            4 => Inbound::ReadDfuFlags,
            5 => Inbound::ResetDfuFlags,
            _ => Inbound::Unknown,
        };
        Ok(inbound)
    }

    fn poll_read_inbound(&mut self, handler: impl FnOnce(Inbound)) -> Result<(), UsbError> {
        if self.poll() {
            let inbound = self.read_inbound()?;
            handler(inbound);
        }

        Ok(())
    }
}

pub enum Outbound {
    Version(u8, u8, u8),
    DeviceId(u16, u16, u32, u32),
    ModeDfu,
    MemoryLayout(u32, u32, u32),
    DfuFlags(&'static dfu::Flags),
    DfuFlagsError(flash::Error),
    ResetDfuFlagsOk,
    ResetDfuFlagsErr(flash::Error),
}

pub trait Writer {
    fn write_outbound(&mut self, outbound: Outbound) -> Result<usize, UsbError>;
}

impl Writer for Device {
    fn write_outbound(&mut self, outbound: Outbound) -> Result<usize, UsbError> {
        match outbound {
            Outbound::Version(major, minor, patch) => {
                let buf = [0, major, minor, patch];
                self.write_all(&buf)
            }
            Outbound::DeviceId(id_0, id_1, id_2, id_3) => {
                let buf = [
                    1,
                    id_0.octet_1(),
                    id_0.octet_2(),
                    id_1.octet_1(),
                    id_1.octet_2(),
                    id_2.octet_1(),
                    id_2.octet_2(),
                    id_2.octet_3(),
                    id_2.octet_4(),
                    id_3.octet_1(),
                    id_3.octet_2(),
                    id_3.octet_3(),
                    id_3.octet_4(),
                ];
                self.write_all(&buf)
            }
            Outbound::ModeDfu => {
                let buf = [2, 200];
                self.write_all(&buf)
            }
            Outbound::MemoryLayout(flash_start, flash_end, flash_size) => {
                let buf = [
                    3,
                    flash_start.octet_1(),
                    flash_start.octet_2(),
                    flash_start.octet_3(),
                    flash_start.octet_4(),
                    flash_end.octet_1(),
                    flash_end.octet_2(),
                    flash_end.octet_3(),
                    flash_end.octet_4(),
                    flash_size.octet_1(),
                    flash_size.octet_2(),
                    flash_size.octet_3(),
                    flash_size.octet_4(),
                ];
                self.write_all(&buf)
            }
            Outbound::DfuFlags(flags) => {
                let flashed = if flags.flashed { 1 } else { 0 };
                let buf = [4, 0, flags.writes, flashed];
                self.write_all(&buf)
            }
            Outbound::DfuFlagsError(error) => {
                let buf = [4, flash_error_code(error)];
                self.write_all(&buf)
            }
            Outbound::ResetDfuFlagsOk => {
                let buf = [5, 0];
                self.write_all(&buf)
            }
            Outbound::ResetDfuFlagsErr(error) => {
                let buf = [5, flash_error_code(error)];
                self.write_all(&buf)
            }
        }
    }
}

fn flash_error_code(error: flash::Error) -> u8 {
    match error {
        flash::Error::AddressLargerThanFlash => 1,
        flash::Error::AddressMisaligned => 2,
        flash::Error::LengthNotMultiple2 => 3,
        flash::Error::LengthTooLong => 4,
        flash::Error::EraseError => 5,
        flash::Error::ProgrammingError => 6,
        flash::Error::WriteError => 7,
        flash::Error::VerifyError => 8,
        flash::Error::UnlockError => 9,
        flash::Error::LockError => 10,
    }
}
