use core::time;

const DEFAULT_IO_TIMEOUT: time::Duration = time::Duration::from_secs(1);

pub trait DeviceIdentifier {
    fn validate_vid(&self, vid: u16) -> bool;
    fn validate_pid(&self, pid: u16) -> bool;
}

pub trait ProductIdentifier {
    fn validate_vendor(&self, vendor: &str) -> bool;
    fn validate_product(&self, product: &str) -> bool;
    fn validate_serial(&self, serial: &str) -> bool;
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

pub struct AnyDeviceIdentifier;

impl DeviceIdentifier for AnyDeviceIdentifier {
    fn validate_vid(&self, _: u16) -> bool {
        true
    }

    fn validate_pid(&self, _: u16) -> bool {
        true
    }
}

pub struct SerialProductIdentifier {
    serial: String,
}

impl From<&str> for SerialProductIdentifier {
    fn from(serial: &str) -> Self {
        Self {
            serial: String::from(serial),
        }
    }
}

impl ProductIdentifier for SerialProductIdentifier {
    fn validate_vendor(&self, _: &str) -> bool {
        true
    }

    fn validate_product(&self, _: &str) -> bool {
        true
    }

    fn validate_serial(&self, serial: &str) -> bool {
        self.serial == serial
    }

    fn timeout(&self) -> time::Duration {
        DEFAULT_IO_TIMEOUT
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

    fn validate_serial(&self, _: &str) -> bool {
        true
    }

    fn timeout(&self) -> time::Duration {
        DEFAULT_IO_TIMEOUT
    }
}

pub struct CompositeProductIdentifier {
    identifiers: Vec<Box<dyn ProductIdentifier>>,
}

impl CompositeProductIdentifier {
    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let identifiers = Vec::with_capacity(capacity);
        Self { identifiers }
    }

    pub fn from(identifier: Box<dyn ProductIdentifier>) -> Self {
        let mut instance = Self::new();
        instance.add(identifier);
        instance
    }

    pub fn add(&mut self, identifier: Box<dyn ProductIdentifier>) {
        self.identifiers.push(identifier)
    }
}

impl ProductIdentifier for CompositeProductIdentifier {
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

    fn validate_serial(&self, serial: &str) -> bool {
        for identifier in self.identifiers.iter() {
            if !identifier.validate_serial(serial) {
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
