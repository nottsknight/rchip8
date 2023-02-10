pub trait HiLo<T>: Sized {
    fn hi(&self) -> T;
    fn lo(&self) -> T;

    fn split(&self) -> (T,T) {
        (self.hi(), self.lo())
    }
}

impl HiLo<u8> for u8 {
    fn hi(&self) -> u8 {
        (self & 0xf0) >> 4
    }

    fn lo(&self) -> u8 {
        self & 0xf
    }
}

impl HiLo<u8> for u16 {
    fn hi(&self) -> u8 {
        ((self & 0xff00) >> 8) as u8
    }

    fn lo(&self) -> u8 {
        (self & 0xff) as u8
    }
}
