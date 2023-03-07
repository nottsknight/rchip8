// This file is part of rchip8.
//
// rchip8 is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
//
// rchip8 is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with rchip8.
// If not, see <https://www.gnu.org/licenses/>.

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
    use rstest::*;

    #[rstest]
    #[case(0x00, 0x00)]
    #[case(0x00, 0xff)]
    #[case(0xff, 0x00)]
    #[case(0xef, 0x01)]
    #[case(0x01, 0xef)]
    fn test_add_no_overflow(#[case] n: u8, #[case] m: u8) {
        let (sum, carry) = u8::add_carry(n, m);
        assert_eq!(n+m, sum);
        assert!(!carry);
    }

    #[rstest]
    #[case(0xff, 0x01)]
    #[case(0x01, 0xff)]
    fn test_add_overflow(#[case] n: u8, #[case] m: u8) {
        let (_sum, carry) = u8::add_carry(n, m);
        assert!(carry);
    }
}

pub trait SubBorrow: Sized {
    fn sub_borrow(x: Self, y: Self) -> (Self, bool);
    fn sub_no_borrow(x: Self, y: Self) -> Self {
        Self::sub_borrow(x, y).0
    }
}

impl SubBorrow for u8 {
    fn sub_borrow(x: Self, y: Self) -> (Self, bool) {
        if x < y {
            (0, false)
        } else {
            (x - y, true)
        }
    }
}

#[cfg(test)]
mod sub_borrow_u8_tests {
    use super::SubBorrow;

    #[test]
    fn sub_no_borrow() {
        assert_eq!(u8::sub_borrow(20, 15), (5, true));
    }

    #[test]
    fn sub_borrow() {
        assert_eq!(u8::sub_borrow(15, 20), (0, false));
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

#[cfg(test)]
mod shift_overflow_u8_tests {
    use super::ShiftOverflow;
    use rstest::*;

    #[rstest]
    #[case(0x00)]
    #[case(0x7f)]
    #[case(0x80)]
    #[case(0xff)]
    fn test_shift_left(#[case] n: u8) {
        let (n1, overflow) = u8::shift_left(n, 1);
        if n > 0x7f {
            assert_eq!((n & 0x7f) << 1, n1);
            assert!(overflow);
        } else {
            assert_eq!(n << 1, n1);
            assert!(!overflow);
        }
    }

    #[rstest]
    #[case(0x00)]
    #[case(0x01)]
    #[case(0x0f)]
    #[case(0xf0)]
    #[case(0xff)]
    fn test_shift_right(#[case] n: u8) {
        let (n1, underflow) = u8::shift_right(n, 1);
        if n % 2 == 1 {
            assert_eq!((n & 0xfe) >> 1, n1);
            assert!(underflow);
        } else {
            assert_eq!(n >> 1, n1);
            assert!(!underflow);
        }
    }
}
