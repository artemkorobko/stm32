use anyhow::Context;
use rusb::UsbContext;

use super::device::iterator::GenericDeviceList;

pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

pub struct Driver {
    context: rusb::Context,
}

impl Driver {
    pub fn new() -> anyhow::Result<Self> {
        let context = rusb::Context::new().context("Can't load USB driver")?;
        Ok(Self { context })
    }

    pub fn version() -> Version {
        let version = rusb::version();
        Version {
            major: version.major(),
            minor: version.minor(),
            patch: version.micro(),
        }
    }

    pub fn devices(&self) -> anyhow::Result<GenericDeviceList> {
        let devices = self
            .context
            .devices()
            .context("Can't list attached USB devices")?;
        Ok(GenericDeviceList::from(devices))
    }
}
