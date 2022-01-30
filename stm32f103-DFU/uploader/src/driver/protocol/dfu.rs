use crate::driver::device::{opened::OpenedDevice, DEFAULT_IO_TIMEOUT};

use super::common;

pub struct MemoryLayout {
    pub flash_start: u32,
    pub flash_end: u32,
    pub flash_size: u32,
}

pub struct DfuFlags {
    pub writes: u8,
    pub flashed: bool,
}

pub trait DfuProtocol {
    fn dfu_memory_layout(&mut self) -> anyhow::Result<MemoryLayout>;
    fn dfu_read_flags(&mut self) -> anyhow::Result<Option<DfuFlags>>;
    fn dfu_reset_flags(&mut self) -> anyhow::Result<bool>;
}

impl DfuProtocol for OpenedDevice {
    fn dfu_memory_layout(&mut self) -> anyhow::Result<MemoryLayout> {
        let opcode = 3;
        let buf = [opcode];
        let size = self.write_all(&buf)?;
        common::validate_size_sent(size, 1)?;
        let mut buf = [0; 64];
        let size = self.read(&mut buf, DEFAULT_IO_TIMEOUT)?;
        common::validate_opcode(buf[0], opcode)?;
        common::validate_size_received(size, 13)?;
        Ok(MemoryLayout {
            flash_start: buf[1] as u32
                | (buf[2] as u32) << 8
                | (buf[3] as u32) << 16
                | (buf[4] as u32) << 24,
            flash_end: buf[5] as u32
                | (buf[6] as u32) << 8
                | (buf[7] as u32) << 16
                | (buf[8] as u32) << 24,
            flash_size: buf[9] as u32
                | (buf[10] as u32) << 8
                | (buf[11] as u32) << 16
                | (buf[12] as u32) << 24,
        })
    }

    fn dfu_read_flags(&mut self) -> anyhow::Result<Option<DfuFlags>> {
        let opcode = 4;
        let buf = [opcode];
        let size = self.write_all(&buf)?;
        common::validate_size_sent(size, 1)?;
        let mut buf = [0; 64];
        let size = self.read(&mut buf, DEFAULT_IO_TIMEOUT)?;
        common::validate_opcode(buf[0], opcode)?;
        let status = buf[1];
        if status == 0 {
            common::validate_size_received(size, 4)?;
            Ok(Some(DfuFlags {
                writes: buf[2],
                flashed: if buf[3] == 1 { true } else { false },
            }))
        } else if status == 0xff {
            anyhow::bail!("USB device DFU flags not present");
        } else {
            common::validate_size_received(size, 2)?;
            anyhow::bail!(
                "Can't read USB device DFU flags: {}",
                decode_flash_error(status)
            );
        }
    }

    fn dfu_reset_flags(&mut self) -> anyhow::Result<bool> {
        let opcode = 5;
        let buf = [opcode];
        let size = self.write_all(&buf)?;
        common::validate_size_sent(size, 1)?;
        let mut buf = [0; 64];
        let size = self.read(&mut buf, DEFAULT_IO_TIMEOUT)?;
        common::validate_opcode(buf[0], opcode)?;
        common::validate_size_received(size, 2)?;
        let status = buf[1];
        if status == 0 {
            Ok(true)
        } else {
            anyhow::bail!(
                "Can't reset USB device DFU flags: {}",
                decode_flash_error(status)
            )
        }
    }
}

fn decode_flash_error(error: u8) -> &'static str {
    match error {
        1 => "AddressLargerThanFlash",
        2 => "AddressMisaligned",
        3 => "LengthNotMultiple2",
        4 => "LengthTooLong",
        5 => "EraseError",
        6 => "ProgrammingError",
        7 => "WriteError",
        8 => "VerifyError",
        9 => "UnlockError",
        10 => "LockError",
        _ => "Unknown",
    }
}
