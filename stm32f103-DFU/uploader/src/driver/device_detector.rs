pub trait DeviceDetector {
    fn validate_vid(&self, vid: u16) -> bool;
    fn validate_pid(&self, pid: u16) -> bool;
    fn validate_ids(&self, vid: u16, pid: u16) -> bool {
        self.validate_vid(vid) && self.validate_pid(pid)
    }
    fn validate_vendor(&self, vendor: &str) -> bool;
    fn validate_product(&self, product: &str) -> bool;
    fn validate_metadata(&self, vendor: &str, product: &str) -> bool {
        self.validate_vendor(vendor) && self.validate_product(product)
    }
}

pub struct DefaultDeviceDetector;

impl DefaultDeviceDetector {
    pub fn boxed() -> Box<dyn DeviceDetector> {
        Box::new(DefaultDeviceDetector {})
    }
}

impl DeviceDetector for DefaultDeviceDetector {
    fn validate_vid(&self, vid: u16) -> bool {
        vid == 1155
    }

    fn validate_pid(&self, pid: u16) -> bool {
        pid == 22336
    }

    fn validate_vendor(&self, vendor: &str) -> bool {
        vendor == "ST Microelectronics"
    }

    fn validate_product(&self, product: &str) -> bool {
        product == "STM32 Virtual ComPort"
    }
}
