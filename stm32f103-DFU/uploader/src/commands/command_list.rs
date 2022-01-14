use crate::driver::{device::Device, device_detector::DefaultDeviceDetector, driver::Driver};

use super::command_executor::CommandExecutor;

pub struct CommandExecutorList {
    driver: Driver,
}

impl CommandExecutorList {
    pub fn new(driver: Driver) -> Self {
        Self { driver }
    }

    pub fn boxed(self) -> Box<dyn CommandExecutor> {
        Box::new(self)
    }

    fn list_devices(&self) -> anyhow::Result<Vec<Device>> {
        self.driver.list_devices(&DefaultDeviceDetector::boxed())
    }

    fn print_devices(&self, devices: Vec<Device>) {
        for device in devices {
            let meta = device.metadata();
            log::info!(
                "[{}] {} - [{}] {} : {}",
                meta.vid,
                meta.vendor,
                meta.pid,
                meta.product,
                meta.serial,
            );
        }
    }
}

impl CommandExecutor for CommandExecutorList {
    fn exec(&self) -> anyhow::Result<()> {
        let devices = self.list_devices()?;
        if devices.is_empty() {
            log::info!("No connected devices found");
        } else {
            log::info!("Connected devices");
            self.print_devices(devices);
        }
        Ok(())
    }
}
