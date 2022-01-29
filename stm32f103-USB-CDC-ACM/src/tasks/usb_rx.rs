use rtic::Mutex;

use crate::{app::usb_inbound, app::usb_rx, drivers::cdc_acm::Device};

use super::usb_inbound::Inbound;

pub fn usb_rx(mut cx: usb_rx::Context) {
    cx.shared.usb.lock(poll);
}

fn poll(device: &mut Device) {
    if device.poll() {
        if let Some(inbound) = read_inbound(device) {
            usb_inbound::spawn(inbound).ok();
        }
    }
}

fn read_inbound(device: &mut Device) -> Option<Inbound> {
    let mut buf = [0u8; 8];
    device.read(&mut buf).ok()?;
    match buf[0] {
        0 => Some(Inbound::Version),
        1 => Some(Inbound::DeviceId),
        _ => None,
    }
}
