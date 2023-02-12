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

#[cfg(test)]
mod hilo_u8_tests {
    use super::HiLo;

    #[test]
    fn test_hi() {
        assert_eq!(0xab_u8.hi(), 0xa);
    }

    #[test]
    fn test_lo() {
        assert_eq!(0xab_u8.lo(), 0xb);
    }

    #[test]
    fn test_split() {
        assert_eq!(0xab_u8.split(), (0xa, 0xb));
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

#[cfg(test)]
mod hilo_u16_tests {
    use super::HiLo;

    #[test]
    fn test_hi() {
        assert_eq!(0xabcd_u16.hi(), 0xab);
    }

    #[test]
    fn test_lo() {
        assert_eq!(0xabcd_u16.lo(), 0xcd);
    }

    #[test]
    fn test_split() {
        assert_eq!(0xabcd_u16.split(), (0xab, 0xcd));
    }
}
