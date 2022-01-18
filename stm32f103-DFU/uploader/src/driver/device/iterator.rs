use super::generic::GenericDevice;

pub struct GenericDeviceList {
    inner: rusb::DeviceList<rusb::Context>,
}

impl From<rusb::DeviceList<rusb::Context>> for GenericDeviceList {
    fn from(inner: rusb::DeviceList<rusb::Context>) -> Self {
        Self { inner }
    }
}

impl GenericDeviceList {
    pub fn iter(&self) -> GenericDeviceIterator {
        GenericDeviceIterator::from(self.inner.iter())
    }
}

pub struct GenericDeviceIterator<'a> {
    inner: rusb::Devices<'a, rusb::Context>,
}
impl<'a> From<rusb::Devices<'a, rusb::Context>> for GenericDeviceIterator<'a> {
    fn from(inner: rusb::Devices<'a, rusb::Context>) -> Self {
        Self { inner }
    }
}

impl<'a> Iterator for GenericDeviceIterator<'a> {
    type Item = GenericDevice;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|device| GenericDevice::from(device))
    }
}
