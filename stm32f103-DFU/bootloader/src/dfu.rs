use stm32f1xx_hal::flash;

const FLASH_SIZE: u32 = crate::app::FLASH_SIZE as u32 * flash::SZ_1K as u32;
const DFU_FLAGS_MAGIC_NUMBER: u32 = 0xDEADBEEF;
const DFU_FLAGS_RESERVED_SIZE: u32 = flash::SZ_1K as u32;
const DFU_FLAGS_OFFSET: u32 = FLASH_SIZE - DFU_FLAGS_RESERVED_SIZE;
const FLAGS_SIZE: usize = 6; // Should be power of 2

#[derive(Default)]
pub struct Flags {
    pub magic: u32,
    pub writes: u8,
    pub flashed: bool,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            magic: DFU_FLAGS_MAGIC_NUMBER,
            ..Default::default()
        }
    }

    pub fn read(writer: &mut flash::FlashWriter) -> flash::Result<Option<Self>> {
        let data = writer.read(DFU_FLAGS_OFFSET, FLAGS_SIZE)?;
        let flags = Self::from_slice(data);
        if flags.magic == DFU_FLAGS_MAGIC_NUMBER {
            Ok(Some(flags))
        } else {
            Ok(None)
        }
    }

    pub fn write(&self, writer: &mut flash::FlashWriter) -> flash::Result<()> {
        let data = self.as_slice();
        writer.erase(DFU_FLAGS_OFFSET, FLAGS_SIZE)?;
        writer.write(DFU_FLAGS_OFFSET, &data)
    }

    fn as_slice(&self) -> [u8; FLAGS_SIZE] {
        let mut flags = 0u8;
        if self.flashed {
            flags |= 1;
        }

        [
            self.magic as u8,
            (self.magic >> 8) as u8,
            (self.magic >> 16) as u8,
            (self.magic >> 24) as u8,
            self.writes,
            flags,
        ]
    }

    fn from_slice(data: &[u8]) -> Self {
        let flags = data[5];
        let mut flashed = false;
        if flags & 1 == 1 {
            flashed = true;
        }

        Self {
            magic: data[0] as u32
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24,
            writes: data[4],
            flashed,
        }
    }
}
