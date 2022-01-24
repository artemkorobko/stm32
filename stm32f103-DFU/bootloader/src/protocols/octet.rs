pub trait OctetLo {
    fn octet_1(&self) -> u8;
    fn octet_2(&self) -> u8;
}

impl OctetLo for u16 {
    fn octet_1(&self) -> u8 {
        *self as u8
    }

    fn octet_2(&self) -> u8 {
        (*self >> 8) as u8
    }
}

impl OctetLo for u32 {
    fn octet_1(&self) -> u8 {
        *self as u8
    }

    fn octet_2(&self) -> u8 {
        (*self >> 8) as u8
    }
}

pub trait OctetHi {
    fn octet_3(&self) -> u8;
    fn octet_4(&self) -> u8;
}

impl OctetHi for u32 {
    fn octet_3(&self) -> u8 {
        (*self >> 16) as u8
    }

    fn octet_4(&self) -> u8 {
        (*self >> 24) as u8
    }
}
