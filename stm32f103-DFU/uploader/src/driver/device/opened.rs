use super::identified::IdentifiedDevice;

pub struct OpenedDevice {
    inner: IdentifiedDevice,
}

impl OpenedDevice {
    pub fn new(inner: IdentifiedDevice) -> Self {
        Self { inner }
    }
}
