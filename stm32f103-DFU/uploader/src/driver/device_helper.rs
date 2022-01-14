use std::time;

use anyhow::Context;

pub fn read_descriptor<T>(dev: &rusb::Device<T>) -> anyhow::Result<rusb::DeviceDescriptor>
where
    T: rusb::UsbContext,
{
    dev.device_descriptor().with_context(|| {
        format!(
            "Unable to read USB device descriptor at address {} of bus {}",
            dev.address(),
            dev.bus_number()
        )
    })
}

pub fn open<T>(dev: &rusb::Device<T>) -> anyhow::Result<rusb::DeviceHandle<T>>
where
    T: rusb::UsbContext,
{
    dev.open().with_context(|| {
        format!(
            "Unable to open USB device at address {} of bus {}",
            dev.address(),
            dev.bus_number()
        )
    })
}

pub fn first_language<T: rusb::UsbContext>(
    handle: &rusb::DeviceHandle<T>,
    timeout: time::Duration,
) -> anyhow::Result<rusb::Language> {
    handle
        .read_languages(timeout)
        .with_context(|| {
            let dev = handle.device();
            format!(
                "Unable to read languages from USB device at address {} of bus {}",
                dev.address(),
                dev.bus_number()
            )
        })?
        .first()
        .cloned()
        .ok_or_else(|| {
            let dev = handle.device();
            anyhow::anyhow!(
                "USB device at address {} of bus {} does not have any language",
                dev.address(),
                dev.bus_number()
            )
        })
}

pub fn vendor<T: rusb::UsbContext>(
    handle: &rusb::DeviceHandle<T>,
    descr: &rusb::DeviceDescriptor,
    lang: rusb::Language,
    timeout: time::Duration,
) -> anyhow::Result<String> {
    handle
        .read_manufacturer_string(lang, descr, timeout)
        .with_context(|| {
            let dev = handle.device();
            format!(
                "Unable to read manufacturer from USB device at address {} of bus {}",
                dev.address(),
                dev.bus_number()
            )
        })
}

pub fn product<T: rusb::UsbContext>(
    handle: &rusb::DeviceHandle<T>,
    descr: &rusb::DeviceDescriptor,
    lang: rusb::Language,
    timeout: time::Duration,
) -> anyhow::Result<String> {
    handle
        .read_product_string(lang, descr, timeout)
        .with_context(|| {
            let dev = handle.device();
            format!(
                "Unable to read product from USB device at address {} of bus {}",
                dev.address(),
                dev.bus_number()
            )
        })
}

pub fn serial_number<T: rusb::UsbContext>(
    handle: &rusb::DeviceHandle<T>,
    descr: &rusb::DeviceDescriptor,
    lang: rusb::Language,
    timeout: time::Duration,
) -> anyhow::Result<String> {
    handle
        .read_serial_number_string(lang, descr, timeout)
        .with_context(|| {
            let dev = handle.device();
            format!(
                "Unable to read serial number from USB device at address {} of bus {}",
                dev.address(),
                dev.bus_number()
            )
        })
}
