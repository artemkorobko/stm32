use usb_device::UsbError;

use crate::drivers::cdc_acm::Device;

use super::octet::{OctetHi, OctetLo};

pub enum Outbound {
    Version(u8, u8, u8),
    DeviceId(u16, u16, u32, u32),
    ModeDevice,
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
            Outbound::ModeDevice => {
                let buf = [2, 0];
                self.write_all(&buf)
            }
        }
    }
}
