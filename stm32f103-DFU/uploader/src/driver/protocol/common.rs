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

pub enum DeviceMode {
    Fdu,
    Device,
}

pub trait CommonProtocol {
    fn firmware_version(&mut self) -> anyhow::Result<FirmwareVersion>;
    fn device_id(&mut self) -> anyhow::Result<DeviceId>;
    fn device_mode(&mut self) -> anyhow::Result<DeviceMode>;
}

impl CommonProtocol for OpenedDevice {
    fn firmware_version(&mut self) -> anyhow::Result<FirmwareVersion> {
        let opcode = 0;
        let buf = [opcode];
        let size = self.write_all(&buf)?;
        validate_size_sent(size, 1)?;
        let mut buf = [0; 64];
        let size = self.read(&mut buf, DEFAULT_IO_TIMEOUT)?;
        validate_opcode(buf[0], opcode)?;
        validate_size_received(size, 4)?;
        Ok(FirmwareVersion {
            major: buf[1],
            minor: buf[2],
            patch: buf[3],
        })
    }

    fn device_id(&mut self) -> anyhow::Result<DeviceId> {
        let opcode = 1;
        let buf = [opcode];
        let size = self.write_all(&buf)?;
        validate_size_sent(size, 1)?;
        let mut buf = [0; 64];
        let size = self.read(&mut buf, DEFAULT_IO_TIMEOUT)?;
        validate_opcode(buf[0], opcode)?;
        validate_size_received(size, 13)?;
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

    fn device_mode(&mut self) -> anyhow::Result<DeviceMode> {
        let opcode = 2;
        let buf = [opcode];
        let size = self.write_all(&buf)?;
        validate_size_sent(size, 1)?;
        let mut buf = [0; 64];
        let size = self.read(&mut buf, DEFAULT_IO_TIMEOUT)?;
        validate_opcode(buf[0], opcode)?;
        validate_size_received(size, 2)?;
        if buf[1] == 200 {
            Ok(DeviceMode::Fdu)
        } else {
            Ok(DeviceMode::Device)
        }
    }
}

pub fn validate_size_sent(sent: usize, expected: usize) -> anyhow::Result<()> {
    if sent == expected {
        Ok(())
    } else {
        anyhow::bail!(
            "Can't send {} bytes to USB device, sent: {}",
            expected,
            sent,
        )
    }
}

pub fn validate_size_received(received: usize, expected: usize) -> anyhow::Result<()> {
    if received == expected {
        Ok(())
    } else {
        anyhow::bail!(
            "Can't receive {} bytes from USB device, received: {}",
            expected,
            received,
        )
    }
}

pub fn validate_opcode(received: u8, expected: u8) -> anyhow::Result<()> {
    if received == expected {
        Ok(())
    } else {
        anyhow::bail!(
            "Invalid opcode {} received from USB device, expected: {}",
            received,
            expected,
        )
    }
}
