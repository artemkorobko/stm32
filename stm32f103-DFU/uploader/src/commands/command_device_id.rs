use crate::driver::{prelude::*, protocol::common::CommonProtocol};

use super::command_executor::CommandExecutor;

pub struct CommandDeviceId {
    driver: Driver,
    serial: String,
}

impl CommandDeviceId {
    pub fn new(driver: Driver, serial: String) -> Self {
        Self { driver, serial }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }

    fn find_by_serial(&self, serial: &str) -> anyhow::Result<Option<IdentifiedDevice>> {
        let i_device = DefaultDeviceIdentifier;
        let mut i_product = CompositeProductIdentifier::with_capacity(2);
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

impl CommandExecutor for CommandDeviceId {
    fn exec(&self) -> anyhow::Result<()> {
        if let Some(device) = self.find_by_serial(&self.serial)? {
            log::info!("Found USB device with serial {}", self.serial);
            let device_id = device.open()?.device_id()?;
            log::info!(
                "USB device id {}-{}-{}-{}",
                device_id.id_0,
                device_id.id_1,
                device_id.id_2,
                device_id.id_3,
            );
        } else {
            log::error!("Can't find USB device with serial {}", self.serial);
        }
        Ok(())
    }
}
