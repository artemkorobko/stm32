pub use super::device::{
    generic::{GenericDevice, Identification},
    identified::IdentifiedDevice,
    identifier::{
        CompositeProductIdentifier, DefaultDeviceIdentifier, DefaultProductIdentifier,
        SerialProductIdentifier,
    },
    opened::OpenedDevice,
};
pub use super::driver::Driver;
pub use super::protocol::common::{CommonProtocol, DeviceMode};
pub use super::protocol::dfu::DfuProtocol;
