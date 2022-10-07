pub trait HiLo {
    type Half;
    fn hi(&self) -> Self::Half;
    fn lo(&self) -> Self::Half;
}

impl HiLo for u16 {
    type Half = u8;

    fn hi(&self) -> u8 {
        ((self & 0xff00) >> 8) as u8
    }

    fn lo(&self) -> u8 {
        (self & 0xff) as u8
    }
}

impl HiLo for u8 {
    type Half = u8;

    fn hi(&self) -> u8 {
        (self & 0xf0) >> 4
    }

    fn lo(&self) -> u8 {
        self & 0xf
    }
}
