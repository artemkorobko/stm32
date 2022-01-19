use core::time;

pub trait DeviceIdentifier {
    fn validate_vid(&self, vid: u16) -> bool;
    fn validate_pid(&self, pid: u16) -> bool;
}

pub trait ProductIdentifier {
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

impl ProductIdentifier for DefaultProductIdentifier {
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

pub struct MultiProductIdentifier {
    identifiers: Vec<Box<dyn ProductIdentifier>>,
}

impl MultiProductIdentifier {
    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let identifiers = Vec::with_capacity(capacity);
        Self { identifiers }
    }

    pub fn add(&mut self, identifier: Box<dyn ProductIdentifier>) {
        self.identifiers.push(identifier)
    }
}

impl From<Box<dyn ProductIdentifier>> for MultiProductIdentifier {
    fn from(identifier: Box<dyn ProductIdentifier>) -> Self {
        let mut instance = Self::new();
        instance.add(identifier);
        instance
    }
}

impl ProductIdentifier for MultiProductIdentifier {
    fn validate_vendor(&self, vendor: &str) -> bool {
        for identifier in self.identifiers.iter() {
            if !identifier.validate_vendor(vendor) {
                return false;
            }
        }

        true
    }

    fn validate_product(&self, product: &str) -> bool {
        for identifier in self.identifiers.iter() {
            if !identifier.validate_product(product) {
                return false;
            }
        }

        true
    }

    fn timeout(&self) -> time::Duration {
        self.identifiers
            .iter()
            .map(|identifier| identifier.timeout())
            .reduce(time::Duration::max)
            .unwrap_or(time::Duration::from_secs(1))
    }
}
