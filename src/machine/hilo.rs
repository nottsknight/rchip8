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
