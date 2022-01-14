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
        let mut devs = Vec::new();
        let usb_devs = self.ctx.devices().context("Unable to list USB devices")?;
        for usb_dev in usb_devs.iter() {
            let descr = device_helper::read_descriptor(&usb_dev)?;
            let vid = descr.vendor_id();
            let pid = descr.product_id();
            if detector.validate_ids(vid, pid) {
                let handle = device_helper::open(&usb_dev)?;
                let timeout = time::Duration::from_secs(1);
                let lang = device_helper::first_language(&handle, timeout)?;
                let vendor = device_helper::vendor(&handle, &descr, lang, timeout)?;
                let product = device_helper::product(&handle, &descr, lang, timeout)?;
                if detector.validate_metadata(&vendor, &product) {
                    let serial = device_helper::serial_number(&handle, &descr, lang, timeout)?;
                    let meta = Metadata {
                        vid,
                        pid,
                        vendor,
                        product,
                        serial,
                    };
                    devs.push(Device::new(usb_dev, handle, meta));
                }
            }
        }
        Ok(devs)
    }
}
