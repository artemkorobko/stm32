use crate::driver::device::{opened::OpenedDevice, DEFAULT_IO_TIMEOUT};

pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

pub trait CommonProtocol {
    fn firmware_version(&mut self) -> anyhow::Result<FirmwareVersion>;
}

impl CommonProtocol for OpenedDevice {
    fn firmware_version(&mut self) -> anyhow::Result<FirmwareVersion> {
        let buf = [0];
        self.write_all(&buf)?;
        let mut buf = [0; 64];
        self.read(&mut buf, DEFAULT_IO_TIMEOUT)?;
        Ok(FirmwareVersion {
            major: buf[1],
            minor: buf[2],
            patch: buf[3],
        })
    }
}
