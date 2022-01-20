use crate::driver::device::{opened::OpenedDevice, DEFAULT_IO_TIMEOUT};

pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

pub struct DeviceId {
    pub id_0: u16,
    pub id_1: u16,
    pub id_2: u32,
    pub id_3: u32,
}

pub trait CommonProtocol {
    fn firmware_version(&mut self) -> anyhow::Result<FirmwareVersion>;
    fn device_id(&mut self) -> anyhow::Result<DeviceId>;
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

    fn device_id(&mut self) -> anyhow::Result<DeviceId> {
        let buf = [1];
        self.write_all(&buf)?;
        let mut buf = [0; 64];
        self.read(&mut buf, DEFAULT_IO_TIMEOUT)?;
        Ok(DeviceId {
            id_0: buf[1] as u16 | ((buf[2] as u16) << 8),
            id_1: buf[3] as u16 | ((buf[4] as u16) << 8),
            id_2: buf[5] as u32
                | ((buf[6] as u32) << 8)
                | ((buf[7] as u32) << 16)
                | ((buf[8] as u32) << 24),
            id_3: buf[9] as u32
                | ((buf[10] as u32) << 8)
                | ((buf[11] as u32) << 16)
                | ((buf[12] as u32) << 24),
        })
    }
}
