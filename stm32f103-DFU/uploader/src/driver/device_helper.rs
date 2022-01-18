use std::time;

use anyhow::Context;

pub fn set_active_configuration<T>(
    handle: &mut rusb::DeviceHandle<T>,
    configuration: u8,
) -> anyhow::Result<()>
where
    T: rusb::UsbContext,
{
    handle
        .set_active_configuration(configuration)
        .with_context(|| format!("Unable to set active configuration {}", configuration))
}
