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
