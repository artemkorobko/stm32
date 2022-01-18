use crate::driver::{
    device::{
        generic::Identification,
        identifier::{DefaultDeviceIdentifier, DefaultProductIdentifier},
    },
    driver::Driver,
};

use super::command_executor::CommandExecutor;

pub struct CommandList {
    driver: Driver,
}

impl CommandList {
    pub fn new(driver: Driver) -> Self {
        Self { driver }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }
}

impl CommandExecutor for CommandList {
    fn exec(&self) -> anyhow::Result<()> {
        let i_device = DefaultDeviceIdentifier {};
        let i_product = DefaultProductIdentifier {};
        let mut devices = 0;
        for device in self.driver.devices()?.iter() {
            if let Identification::Identified(device) = device.identify(&i_device, &i_product)? {
                log::info!("----------");
                log::info!("Vendor/ID: {}/{}", device.vendor(), device.vendor_id());
                log::info!("Product/ID: {}/{}", device.product(), device.product_id());
                log::info!("Serial: {}", device.serial_number());
                devices += 1;
            }
        }

        if devices > 0 {
            log::info!("---------- Total devices: {}", devices);
        } else {
            log::info!("No devices found");
        }
        Ok(())
    }
}
