use rtic::Mutex;

use crate::{
    app::usb_rx,
    device_id,
    drivers::cdc_acm::Device,
    protocols::{
        usb_inbound::{Inbound, Reader},
        usb_outbound::{Outbound, Writer},
    },
};

pub fn usb_rx(mut cx: usb_rx::Context) {
    if let Some(inbound) = cx.shared.usb.lock(poll) {
        if let Some(outbound) = handle_inbound(inbound) {
            send(&mut cx, outbound);
        }
    }
}

fn poll(device: &mut Device) -> Option<Inbound> {
    if device.poll() {
        device.read_inbound().ok()
    } else {
        None
    }
}

fn send(cx: &mut usb_rx::Context, outbound: Outbound) {
    cx.shared.usb.lock(|device| {
        device.write_outbound(outbound).ok();
    })
}

fn handle_inbound(inbound: Inbound) -> Option<Outbound> {
    match inbound {
        Inbound::DeviceVersion => device_version(),
        Inbound::DeviceId => device_id(),
        Inbound::DeviceMode => device_mode(),
        Inbound::Unknown => None,
    }
}

fn device_version() -> Option<Outbound> {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap_or(0);
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap_or(0);
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap_or(0);
    Some(Outbound::Version(major, minor, patch))
}

fn device_id() -> Option<Outbound> {
    let (id_0, id_1, id_2, id_3) = device_id::read();
    Some(Outbound::DeviceId(id_0, id_1, id_2, id_3))
}

fn device_mode() -> Option<Outbound> {
    Some(Outbound::ModeDevice)
}
