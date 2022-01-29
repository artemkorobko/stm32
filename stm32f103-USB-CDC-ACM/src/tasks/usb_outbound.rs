use rtic::Mutex;

use crate::{app::usb_outbound, drivers::cdc_acm::Device};

pub enum Outbound {
    Version(u8, u8, u8),
    DeviceId(u16, u16, u32, u32),
}

pub fn usb_outbound(mut cx: usb_outbound::Context, outbound: Outbound) {
    cx.shared.usb.lock(|device| write(device, outbound));
}

fn write(device: &mut Device, outbound: Outbound) {
    match outbound {
        Outbound::Version(major, minor, patch) => {
            let buf = [0, major, minor, patch];
            device.write_all(&buf).ok();
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
            device.write_all(&buf).ok();
        }
    }
}
