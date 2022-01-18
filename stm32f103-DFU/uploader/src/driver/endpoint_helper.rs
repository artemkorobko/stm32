use anyhow::Context;

pub enum Direction {
    Input,
    Output,
}

pub struct EndPointLookup {
    pub configuration: u8,
    pub interface: u8,
    pub setting: u8,
    pub address: u8,
    pub transfer_type: rusb::TransferType,
}

pub fn find_endpoint<T>(
    device: &mut rusb::Device<T>,
    descriptor: &rusb::DeviceDescriptor,
    transfer_type: rusb::TransferType,
    direction: Direction,
) -> Option<EndPointLookup>
where
    T: rusb::UsbContext,
{
    fn has_direction(
        descriptor: &rusb::EndpointDescriptor,
        transfer_type: rusb::TransferType,
        direction: &Direction,
    ) -> bool {
        let has_direction = match direction {
            Direction::Input => descriptor.direction() == rusb::Direction::In,
            Direction::Output => descriptor.direction() == rusb::Direction::Out,
        };
        has_direction && descriptor.transfer_type() == transfer_type
    }

    for config_index in 0..descriptor.num_configurations() {
        if let Ok(config_descriptor) = device.config_descriptor(config_index) {
            for interface in config_descriptor.interfaces() {
                for interface_descriptor in interface.descriptors() {
                    for endpoint_descriptor in interface_descriptor.endpoint_descriptors() {
                        if has_direction(&endpoint_descriptor, transfer_type, &direction) {
                            return Some(EndPointLookup {
                                configuration: config_descriptor.number(),
                                interface: interface_descriptor.interface_number(),
                                setting: interface_descriptor.setting_number(),
                                address: endpoint_descriptor.address(),
                                transfer_type,
                            });
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn is_kernel_driver_active<T>(
    handle: &rusb::DeviceHandle<T>,
    interface: u8,
) -> anyhow::Result<bool>
where
    T: rusb::UsbContext,
{
    handle.kernel_driver_active(interface).with_context(|| {
        format!(
            "Unable to verify if kernel driver is attached fo interface {}",
            interface
        )
    })
}

pub fn detach_kernel_driver<T>(
    handle: &mut rusb::DeviceHandle<T>,
    interface: u8,
) -> anyhow::Result<()>
where
    T: rusb::UsbContext,
{
    handle.detach_kernel_driver(interface).with_context(|| {
        format!(
            "Unable to detach kernel driver from interface {}",
            interface
        )
    })
}

pub fn attach_kernel_driver<T>(
    handle: &mut rusb::DeviceHandle<T>,
    interface: u8,
) -> anyhow::Result<()>
where
    T: rusb::UsbContext,
{
    handle
        .attach_kernel_driver(interface)
        .with_context(|| format!("Unable to attach kernel driver to interface {}", interface))
}

pub fn claim_interface<T>(handle: &mut rusb::DeviceHandle<T>, interface: u8) -> anyhow::Result<()>
where
    T: rusb::UsbContext,
{
    handle
        .claim_interface(interface)
        .with_context(|| format!("Unable to claim interface {}", interface))
}

pub fn set_alternate_setting<T>(
    handle: &mut rusb::DeviceHandle<T>,
    interface: u8,
    setting: u8,
) -> anyhow::Result<()>
where
    T: rusb::UsbContext,
{
    handle
        .set_alternate_setting(interface, setting)
        .with_context(|| {
            format!(
                "Unable to set alternate setting {} to interface {}",
                setting, interface
            )
        })
}
