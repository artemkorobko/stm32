use rtic::Mutex;
use stm32f1xx_hal::flash;

use crate::{
    app::{usb_rx, FLASH_SIZE},
    device_id, dfu,
    drivers::cdc_acm::Device,
    protocols::{
        usb_inbound::{Inbound, Reader},
        usb_outbound::{Outbound, Writer},
    },
};

pub fn usb_rx(mut cx: usb_rx::Context) {
    if let Some(inbound) = cx.shared.usb.lock(poll) {
        if let Some(outbound) = handle_inbound(&mut cx, inbound) {
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

fn handle_inbound(cx: &mut usb_rx::Context, inbound: Inbound) -> Option<Outbound> {
    match inbound {
        Inbound::DeviceVersion => device_version(),
        Inbound::DeviceId => device_id(),
        Inbound::DeviceMode => device_mode(),
        Inbound::MemoryLayout => memory_layout(),
        Inbound::ReadDfuFlags => cx.shared.flash.lock(read_dfu_flags),
        Inbound::ResetDfuFlags => cx.shared.flash.lock(reset_dfu_flags),
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
    Some(Outbound::ModeDfu)
}

fn memory_layout() -> Option<Outbound> {
    let flash_size = FLASH_SIZE as u32 * flash::SZ_1K as u32;
    Some(Outbound::MemoryLayout(
        flash::FLASH_START,
        flash::FLASH_END,
        flash_size,
    ))
}

fn read_dfu_flags(flash: &mut flash::Parts) -> Option<Outbound> {
    let mut writer = flash.writer(flash::SectorSize::Sz1K, FLASH_SIZE);
    Some(match dfu::Flags::read(&mut writer) {
        Ok(flags) => match flags {
            Some(flags) => Outbound::DfuFlags(flags),
            None => Outbound::DfuFlagsEmpty,
        },
        Err(error) => Outbound::DfuFlagsError(error),
    })
}

fn reset_dfu_flags(flash: &mut flash::Parts) -> Option<Outbound> {
    let mut writer = flash.writer(flash::SectorSize::Sz1K, FLASH_SIZE);
    Some(match dfu::Flags::new().write(&mut writer) {
        Ok(_) => Outbound::ResetDfuFlagsOk,
        Err(error) => Outbound::ResetDfuFlagsErr(error),
    })
}
