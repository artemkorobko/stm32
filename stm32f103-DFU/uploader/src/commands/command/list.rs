use crate::driver::prelude::*;

use super::executor::CommandExecutor;

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
        let mut devices = 1;
        for device in self.driver.devices()?.iter() {
            if let Identification::Identified(device) = device.identify(&i_device, &i_product)? {
                log::info!("----------------- Device {}", devices);
                log::info!("Vendor / ID:\t{} / {}", device.vendor(), device.vendor_id());
                log::info!(
                    "Product / ID:\t{} / {}",
                    device.product(),
                    device.product_id()
                );
                log::info!("Serial:\t\t{}", device.serial_number());
                devices += 1;
            }
        }

        if devices > 0 {
            log::info!("----------------- Total devices: {}", devices - 1);
        } else {
            log::info!("No attached devices found");
        }
        Ok(())
    }
}
