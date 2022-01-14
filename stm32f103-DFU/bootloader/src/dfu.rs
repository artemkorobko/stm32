use stm32f1xx_hal::flash;

const FLASH_SIZE: u32 = crate::FLASH_SIZE as u32 * flash::SZ_1K as u32;
const DFU_FLAGS_MAGIC_NUMBER: u32 = 0xDEADBEEF;
const DFU_FLAGS_RESERVED_SIZE: u32 = 1024;
const DFU_FLAGS_OFFSET: u32 = FLASH_SIZE - DFU_FLAGS_RESERVED_SIZE;

#[derive(Default)]
pub struct Flags {
    pub magic: u32,
    pub flash_count: u32,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            magic: DFU_FLAGS_MAGIC_NUMBER,
            ..Default::default()
        }
    }

    pub fn read(writer: &mut flash::FlashWriter) -> flash::Result<&'static Self> {
        let len = core::mem::size_of::<Self>();
        let data = writer.read(DFU_FLAGS_OFFSET, len)?;
        let flags = Self::from_slice(data);
        if flags.magic == DFU_FLAGS_MAGIC_NUMBER {
            Ok(flags)
        } else {
            Err(flash::Error::VerifyError)
        }
    }

    pub fn write(&self, writer: &mut flash::FlashWriter) -> flash::Result<()> {
        let data = self.as_slice();
        writer.erase(DFU_FLAGS_OFFSET, data.len())?;
        writer.write(DFU_FLAGS_OFFSET, data)
    }

    fn as_slice(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                core::mem::size_of::<Self>(),
            )
        }
    }

    fn from_slice(data: &[u8]) -> &'static Self {
        unsafe { &*(data.as_ptr() as *const Self) }
    }
}
