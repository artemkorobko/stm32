use std::time;

use crate::driver::endpoint::Endpoint;

pub struct OpenedDevice {
    handle: rusb::DeviceHandle<rusb::Context>,
    read_ep: Endpoint,
    write_ep: Endpoint,
}

impl OpenedDevice {
    pub fn new(
        handle: rusb::DeviceHandle<rusb::Context>,
        read_ep: Endpoint,
        write_ep: Endpoint,
    ) -> Self {
        Self {
            handle,
            read_ep,
            write_ep,
        }
    }

    pub fn write(&mut self, buf: &[u8], timeout: time::Duration) -> anyhow::Result<usize> {
        self.write_ep.write(&self.handle, buf, timeout)
    }

    pub fn write_all(&mut self, buf: &[u8]) -> anyhow::Result<usize> {
        self.try_write_all(buf, time::Duration::MAX, usize::MAX)
    }

    pub fn try_write_all(
        &mut self,
        buf: &[u8],
        timeout: time::Duration,
        mut retries: usize,
    ) -> anyhow::Result<usize> {
        let bytes_total = buf.len();
        let mut bytes_written = self.write(buf, timeout)?;
        while bytes_written < bytes_total && retries > 0 {
            bytes_written += self.write(&buf[bytes_written..], timeout)?;
            retries -= 1;
        }
        Ok(bytes_written)
    }

    pub fn read(&mut self, buf: &mut [u8], timeout: time::Duration) -> anyhow::Result<usize> {
        self.read_ep.read(&self.handle, buf, timeout)
    }

    pub fn read_all(&mut self, buf: &mut [u8]) -> anyhow::Result<usize> {
        self.try_read_all(buf, time::Duration::MAX, usize::MAX)
    }

    pub fn try_read_all(
        &mut self,
        buf: &mut [u8],
        timeout: time::Duration,
        mut retries: usize,
    ) -> anyhow::Result<usize> {
        let bytes_total = buf.len();
        let mut bytes_read = self.read(buf, timeout)?;
        while bytes_read < bytes_total && retries > 0 {
            println!("123");
            bytes_read += self.read(&mut buf[bytes_read..], timeout)?;
            retries -= 1;
        }
        Ok(bytes_read)
    }
}

impl Drop for OpenedDevice {
    fn drop(&mut self) {
        self.read_ep.close(&mut self.handle).ok();
        self.write_ep.close(&mut self.handle).ok();
    }
}
