use core::time;

pub trait DeviceIdentifier {
    fn validate_vid(&self, vid: u16) -> bool;
    fn validate_pid(&self, pid: u16) -> bool;
}

pub trait ProductDetector {
    fn validate_vendor(&self, vendor: &str) -> bool;
    fn validate_product(&self, product: &str) -> bool;
    fn timeout(&self) -> time::Duration;
}

pub struct DefaultDeviceIdentifier;

impl DeviceIdentifier for DefaultDeviceIdentifier {
    fn validate_vid(&self, vid: u16) -> bool {
        vid == 1155
    }

    fn validate_pid(&self, pid: u16) -> bool {
        pid == 22336
    }
}

pub struct DefaultProductIdentifier;

impl ProductDetector for DefaultProductIdentifier {
    fn validate_vendor(&self, vendor: &str) -> bool {
        vendor == "STMicroelectronics"
    }

    fn validate_product(&self, product: &str) -> bool {
        product == "STM32 Virtual ComPort"
    }

    fn timeout(&self) -> time::Duration {
        time::Duration::from_secs(1)
    }
}
