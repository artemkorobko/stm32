use std::time;

use anyhow::Context;

// pub struct Writer<'a> {
//     address: u8,
//     transfer_type: rusb::TransferType,
//     handle: &'a rusb::DeviceHandle<T>,
// }

// impl<'a> Writer<'a> {
//     pub fn write<T>(&mut self, buf: &[u8], timeout: time::Duration) -> anyhow::Result<usize>
//     where
//         T: rusb::UsbContext,
//     {
//         match self.transfer_type {
//             rusb::TransferType::Bulk => handle
//                 .write_bulk(self.address, buf, timeout)
//                 .with_context(|| format!("Unable to bulk write to address {}", self.address)),
//             rusb::TransferType::Interrupt => handle
//                 .write_interrupt(self.address, buf, timeout)
//                 .with_context(|| format!("Unable to interrupt write to address {}", self.address)),
//             _ => Err(anyhow::anyhow!(
//                 "Unsupported transfer type {:?} at address {}",
//                 self.transfer_type,
//                 self.address
//             )),
//         }
//     }
// }
