use usb_device::UsbError;

use crate::drivers::cdc_acm::Device;

pub enum Inbound {
    Version,
    DeviceId,
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
                    id_0 as u8,
                    (id_0 >> 8) as u8,
                    id_1 as u8,
                    (id_1 >> 8) as u8,
                    id_2 as u8,
                    (id_2 >> 8) as u8,
                    (id_2 >> 16) as u8,
                    (id_2 >> 24) as u8,
                    id_3 as u8,
                    (id_3 >> 8) as u8,
                    (id_3 >> 16) as u8,
                    (id_3 >> 24) as u8,
                ];
                self.write_all(&buf)
            }
        }
    }
}
