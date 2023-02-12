pub trait AddCarry: Sized {
    fn add_carry(x: Self, y: Self) -> (Self, bool);
    fn add_no_carry(x: Self, y: Self) -> Self {
        Self::add_carry(x, y).0
    }
}

impl AddCarry for u8 {
    fn add_carry(x: Self, y: Self) -> (Self, bool) {
        let n = x as u16 + y as u16;
        let c = (n & 0xff00) != 0;
        ((n & 0xff) as u8, c)
    }
}

#[cfg(test)]
mod add_carry_u8_tests {
    use super::AddCarry;

    #[test]
    fn add_no_overflow() {
        assert_eq!(u8::add_carry(22, 44), (66, false));
    }

    #[test]
    fn add_overflow_zero() {
        assert_eq!(u8::add_carry(0xfd, 0x3), (0, true));
    }

    #[test]
    fn add_overflow_nonzero() {
        assert_eq!(u8::add_carry(0xfd, 0xa), (7, true));
    }
}

pub trait SubBorrow : Sized {
    fn sub_borrow(x: Self, y: Self) -> (Self, bool);
    fn sub_no_borrow(x: Self, y: Self) -> Self {
        Self::sub_borrow(x, y).0
    }
}

impl SubBorrow for u8 {
    fn sub_borrow(x: Self, y: Self) -> (Self, bool) {
        if x < y {
            (0, true)
        } else {
            (x - y, false)
        }
    }
}

#[cfg(test)]
mod sub_borrow_u8_tests {
    use super::SubBorrow;

    #[test]
    fn sub_no_borrow() {
        assert_eq!(u8::sub_borrow(20, 15), (5, false));
    }

    #[test]
    fn sub_borrow() {
        assert_eq!(u8::sub_borrow(15, 20), (0, true));
    }
}

pub trait ShiftOverflow: Sized {
    fn shift_left(n: Self, x: usize) -> (Self, bool);
    fn shift_right(n: Self, x: usize) -> (Self, bool);
}

impl ShiftOverflow for u8 {
    fn shift_left(n: u8, x: usize) -> (Self, bool) {
        let overflow = (n & 0x80) != 0;
        (n << x, overflow)
    }

    fn shift_right(n: Self, x: usize) -> (Self, bool) {
       let underflow = (n & 0x1) != 0;
       (n >> x, underflow)
    }
}
