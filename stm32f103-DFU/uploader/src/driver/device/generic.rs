use std::time;

use anyhow::Context;

use super::{
    identified::IdentifiedDevice,
    identifier::{DeviceIdentifier, ProductIdentifier},
};

pub struct GenericDevice {
    inner: rusb::Device<rusb::Context>,
}

impl From<rusb::Device<rusb::Context>> for GenericDevice {
    fn from(device: rusb::Device<rusb::Context>) -> Self {
        Self { inner: device }
    }
}

pub enum Identification {
    Identified(IdentifiedDevice),
    Unknown(GenericDevice),
}

impl GenericDevice {
    pub fn identify(
        self,
        i_device: &impl DeviceIdentifier,
        i_product: &impl ProductIdentifier,
    ) -> anyhow::Result<Identification> {
        let descriptor = Self::read_descriptor(&self.inner)?;
        if Self::is_device_supported(&descriptor, i_device) {
            let handle = Self::open_device(&self.inner)?;
            let language = Self::detect_language(&handle, i_product.timeout())?;
            let vendor = Self::read_vendor(&handle, language, &descriptor, i_product.timeout())?;
            let product = Self::read_product(&handle, language, &descriptor, i_product.timeout())?;
            let serial = Self::read_serial(&handle, language, &descriptor, i_product.timeout())?;
            if Self::is_product_supported(&vendor, &product, &serial, i_product) {
                Ok(Identification::Identified(IdentifiedDevice::new(
                    self.inner, descriptor, handle, vendor, product, serial,
                )))
            } else {
                Ok(Identification::Unknown(self))
            }
        } else {
            Ok(Identification::Unknown(self))
        }
    }

    fn is_device_supported(
        descriptor: &rusb::DeviceDescriptor,
        identifier: &impl DeviceIdentifier,
    ) -> bool {
        let vid = descriptor.vendor_id();
        let pid = descriptor.product_id();
        identifier.validate_vid(vid) && identifier.validate_pid(pid)
    }

    fn is_product_supported(
        vendor: &str,
        product: &str,
        serial: &str,
        identifier: &impl ProductIdentifier,
    ) -> bool {
        identifier.validate_vendor(vendor)
            && identifier.validate_product(product)
            && identifier.validate_serial(serial)
    }

    fn read_descriptor(
        device: &rusb::Device<rusb::Context>,
    ) -> anyhow::Result<rusb::DeviceDescriptor> {
        device.device_descriptor().with_context(|| {
            format!(
                "Can't read USB device descriptor on bus {} and address {}",
                device.bus_number(),
                device.address(),
            )
        })
    }

    fn open_device(
        device: &rusb::Device<rusb::Context>,
    ) -> anyhow::Result<rusb::DeviceHandle<rusb::Context>> {
        device.open().with_context(|| {
            format!(
                "Can't open USB device on bus {} and address {}",
                device.bus_number(),
                device.address(),
            )
        })
    }

    fn detect_language(
        handle: &rusb::DeviceHandle<rusb::Context>,
        timeout: time::Duration,
    ) -> anyhow::Result<rusb::Language> {
        let languages = handle.read_languages(timeout).with_context(|| {
            let device = handle.device();
            format!(
                "Can't read USB device languages on bus {} and address {}",
                device.bus_number(),
                device.address(),
            )
        })?;
        languages.first().cloned().ok_or_else(|| {
            let device = handle.device();
            anyhow::anyhow!(
                "USB device on bus {} and address {} does not have any language",
                device.bus_number(),
                device.address(),
            )
        })
    }

    fn read_vendor(
        handle: &rusb::DeviceHandle<rusb::Context>,
        language: rusb::Language,
        descriptor: &rusb::DeviceDescriptor,
        timeout: time::Duration,
    ) -> anyhow::Result<String> {
        handle
            .read_manufacturer_string(language, descriptor, timeout)
            .with_context(|| {
                let device = handle.device();
                format!(
                    "Can't read USB device manufacturer on bus {} and address {}",
                    device.bus_number(),
                    device.address(),
                )
            })
    }

    fn read_product(
        handle: &rusb::DeviceHandle<rusb::Context>,
        language: rusb::Language,
        descriptor: &rusb::DeviceDescriptor,
        timeout: time::Duration,
    ) -> anyhow::Result<String> {
        handle
            .read_product_string(language, descriptor, timeout)
            .with_context(|| {
                let dev = handle.device();
                format!(
                    "Can't read USB device product on bus {} and address {}",
                    dev.bus_number(),
                    dev.address(),
                )
            })
    }

    pub fn read_serial(
        handle: &rusb::DeviceHandle<rusb::Context>,
        language: rusb::Language,
        descriptor: &rusb::DeviceDescriptor,
        timeout: time::Duration,
    ) -> anyhow::Result<String> {
        handle
            .read_serial_number_string(language, descriptor, timeout)
            .with_context(|| {
                let dev = handle.device();
                format!(
                    "Can't read USB device serial number on bus {} and address {}",
                    dev.bus_number(),
                    dev.address(),
                )
            })
    }
}
