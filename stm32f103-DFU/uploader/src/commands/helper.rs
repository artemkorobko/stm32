use crate::driver::prelude::*;

pub fn open_by_serial(driver: &Driver, serial: &str) -> anyhow::Result<OpenedDevice> {
    find_by_serial(driver, serial)?.open()
}

pub fn find_by_serial(driver: &Driver, serial: &str) -> anyhow::Result<IdentifiedDevice> {
    let i_device = DefaultDeviceIdentifier;
    let mut i_product = CompositeProductIdentifier::with_capacity(2);
    i_product.add(Box::new(DefaultProductIdentifier));
    i_product.add(Box::new(SerialProductIdentifier::from(serial)));
    for device in driver.devices()?.iter() {
        if let Identification::Identified(device) = device.identify(&i_device, &i_product)? {
            return Ok(device);
        }
    }
    anyhow::bail!("Can't find USB device with serial {}", serial)
}
