use crate::driver::device::{opened::OpenedDevice, DEFAULT_IO_TIMEOUT};

use super::common;

pub struct DfuFlags {
    pub flash_count: u32,
}

pub trait DfuProtocol {
    fn dfu_read_flags(&mut self) -> anyhow::Result<Option<DfuFlags>>;
}

impl DfuProtocol for OpenedDevice {
    fn dfu_read_flags(&mut self) -> anyhow::Result<Option<DfuFlags>> {
        let buf = [3];
        let size = self.write_all(&buf)?;
        common::validate_size_sent(size, 1)?;
        let mut buf = [0; 64];
        let size = self.read(&mut buf, DEFAULT_IO_TIMEOUT)?;
        common::validate_opcode(buf[0], 3)?;
        let status = buf[1];
        if status == 0 {
            common::validate_size_received(size, 6)?;
            Ok(Some(DfuFlags {
                flash_count: buf[2] as u32
                    | (buf[3] as u32) << 8
                    | (buf[4] as u32) << 16
                    | (buf[5] as u32) << 24,
            }))
        } else if status == 1 {
            common::validate_size_received(size, 2)?;
            Ok(None)
        } else {
            anyhow::bail!(
                "Unknown DFU flags status {} received, expected 0 or 1",
                status
            )
        }
    }
}
