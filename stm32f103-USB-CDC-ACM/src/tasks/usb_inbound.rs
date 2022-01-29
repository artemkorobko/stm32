use crate::{app::usb_inbound, app::usb_outbound, device_id};

use super::usb_outbound::Outbound;

pub enum Inbound {
    Version,
    DeviceId,
    Unknown,
}

pub fn usb_inbound(_: usb_inbound::Context, inbound: Inbound) {
    match inbound {
        Inbound::Version => {
            let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap_or(0);
            let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap_or(0);
            let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap_or(0);
            let outbound = Outbound::Version(major, minor, patch);
            usb_outbound::spawn(outbound).ok();
        }
        Inbound::DeviceId => {
            let (id_0, id_1, id_2, id_3) = device_id::read();
            let outbound = Outbound::DeviceId(id_0, id_1, id_2, id_3);
            usb_outbound::spawn(outbound).ok();
        }
        Inbound::Unknown => {}
    }
}
