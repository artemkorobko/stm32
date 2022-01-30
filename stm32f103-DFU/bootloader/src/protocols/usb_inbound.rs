use usb_device::UsbError;

use crate::drivers::cdc_acm::Device;

pub enum Inbound {
    DeviceVersion,
    DeviceId,
    DeviceMode,
    MemoryLayout,
    ReadDfuFlags,
    ResetDfuFlags,
    Unknown,
}

pub trait Reader {
    fn read_inbound(&mut self) -> Result<Inbound, UsbError>;
}

impl Reader for Device {
    fn read_inbound(&mut self) -> Result<Inbound, UsbError> {
        let mut buf = [0u8; 8];
        self.read(&mut buf)?;
        let inbound = match buf[0] {
            0 => Inbound::DeviceVersion,
            1 => Inbound::DeviceId,
            2 => Inbound::DeviceMode,
            3 => Inbound::MemoryLayout,
            4 => Inbound::ReadDfuFlags,
            5 => Inbound::ResetDfuFlags,
            _ => Inbound::Unknown,
        };
        Ok(inbound)
    }
}
