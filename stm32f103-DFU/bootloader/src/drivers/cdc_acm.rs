use core::borrow::BorrowMut;

use stm32f1xx_hal::usb;
use usb_device::{class_prelude::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[derive(Default)]
pub struct Descriptor {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: &'static str,
    pub product: &'static str,
    pub serial_number: &'static str,
}

pub struct Device {
    device: UsbDevice<'static, usb::UsbBusType>,
    serial: SerialPort<'static, usb::UsbBusType>,
}

impl Device {
    pub fn new(peripheral: usb::Peripheral, descriptor: Descriptor) -> Self {
        let alloc = unsafe {
            static mut USB_BUS: Option<UsbBusAllocator<usb::UsbBusType>> = None;
            *USB_BUS.borrow_mut() = Some(usb::UsbBus::new(peripheral));
            USB_BUS.as_ref().unwrap()
        };

        let serial = SerialPort::new(alloc);
        let vid_pid = UsbVidPid(descriptor.vendor_id, descriptor.product_id);
        let device = UsbDeviceBuilder::new(alloc, vid_pid)
            .manufacturer(descriptor.manufacturer)
            .product(descriptor.product)
            .serial_number(descriptor.serial_number)
            .device_class(USB_CLASS_CDC)
            .build();

        Self { device, serial }
    }

    pub fn poll(&mut self) -> bool {
        self.device.poll(&mut [&mut self.serial])
    }

    pub fn read(&mut self, data: &mut [u8]) -> Result<usize, UsbError> {
        self.serial.read(data)
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, UsbError> {
        self.serial.write(data)
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<usize, UsbError> {
        let mut sent = 0;
        while sent < buf.len() {
            sent += self.write(&buf[sent..])?;
        }
        Ok(sent)
    }
}
