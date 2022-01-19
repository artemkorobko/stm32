use crate::driver::{
    device::{
        generic::Identification,
        identified::IdentifiedDevice,
        identifier::{
            DefaultDeviceIdentifier, DefaultProductIdentifier, MultiProductIdentifier,
            SerialProductIdentifier,
        },
    },
    driver::Driver,
};

use super::command_executor::CommandExecutor;

pub struct CommandFv {
    driver: Driver,
    serial: String,
}

impl CommandFv {
    pub fn new(driver: Driver, serial: String) -> Self {
        Self { driver, serial }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }

    // fn read_firmware_version(&self, device: Device) -> anyhow::Result<(u8, u8, u8)> {
    //     let device = device.open()?;
    //     Ok((0, 0, 0))
    // }

    fn find_by_serial(&self, serial: &str) -> anyhow::Result<Option<IdentifiedDevice>> {
        let i_device = DefaultDeviceIdentifier;
        let mut i_product = MultiProductIdentifier::with_capacity(2);
        i_product.add(Box::new(DefaultProductIdentifier));
        i_product.add(Box::new(SerialProductIdentifier::from(serial)));
        for device in self.driver.devices()?.iter() {
            if let Identification::Identified(device) = device.identify(&i_device, &i_product)? {
                return Ok(Some(device));
            }
        }
        Ok(None)
    }
}

impl CommandExecutor for CommandFv {
    fn exec(&self) -> anyhow::Result<()> {
        if let Some(device) = self.find_by_serial(&self.serial)? {
            log::info!("Found USB device with serial {}", self.serial);
        } else {
            log::error!("Can't find USB device with serial {}", self.serial);
        }
        Ok(())
    }
}
