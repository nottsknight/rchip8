pub trait AddCarry: Sized {
    fn add_carry(x: Self, y: Self) -> (Self, bool);
    fn add_no_carry(x: Self, y: Self) -> Self {
        let (n, _) = Self::add_carry(x, y);
        n
    }
}

impl AddCarry for u8 {
    fn add_carry(x: Self, y: Self) -> (Self, bool) {
        let n = x as u16 + y as u16;
        let c = (n & 0xff00) != 0;
        ((n & 0xff) as u8, c)
    }
}
