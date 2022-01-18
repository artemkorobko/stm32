use std::time;

use anyhow::Context;
use rusb::UsbContext;

use super::{
    device::{Device, Metadata},
    device_detector::DeviceDetector,
    device_helper,
};

pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

pub struct Driver {
    ctx: rusb::Context,
}

impl Driver {
    pub fn new() -> anyhow::Result<Self> {
        let ctx = rusb::Context::new().context("Unable to load USB driver")?;
        Ok(Self { ctx })
    }

    pub fn version() -> Version {
        let version = rusb::version();
        Version {
            major: version.major(),
            minor: version.minor(),
            patch: version.micro(),
        }
    }

    pub fn list_devices(&self, detector: &Box<dyn DeviceDetector>) -> anyhow::Result<Vec<Device>> {
        let mut devices = Vec::new();
        let usb_devices = self.ctx.devices().context("Unable to list USB devices")?;
        for usb_device in usb_devices.iter() {
            let descriptor = device_helper::read_descriptor(&usb_device)?;
            let vid = descriptor.vendor_id();
            let pid = descriptor.product_id();
            if detector.validate_ids(vid, pid) {
                let timeout = time::Duration::from_secs(1);
                let handle = device_helper::open(&usb_device)?;
                let language = device_helper::first_language(&handle, timeout)?;
                let vendor = device_helper::vendor(&handle, &descriptor, language, timeout)?;
                let product = device_helper::product(&handle, &descriptor, language, timeout)?;
                if detector.validate_metadata(&vendor, &product) {
                    let serial =
                        device_helper::serial_number(&handle, &descriptor, language, timeout)?;
                    let meta = Metadata {
                        vid,
                        pid,
                        vendor,
                        product,
                        serial,
                    };
                    devices.push(Device::new(usb_device, handle, descriptor, meta));
                }
            }
        }
        Ok(devices)
    }

    pub fn open_device(
        &self,
        detector: &Box<dyn DeviceDetector>,
        serial: &str,
    ) -> anyhow::Result<Option<Device>> {
        let device = self
            .list_devices(detector)?
            .into_iter()
            .find(|device| device.metadata().serial == serial);
        Ok(device)
    }
}
