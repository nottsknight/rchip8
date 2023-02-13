use super::hilo::HiLo;
use super::insts::Chip8Inst;
use super::Chip8Machine;

#[inline]
fn make_usize(x: u8, y: u8, z: u8) -> usize {
    ((x as usize) << 8) | ((y as usize) << 4) | z as usize
}

#[cfg(test)]
mod make_usize_tests {
    use super::make_usize;

    #[test]
    fn test_make_usize() {
        assert_eq!(make_usize(0xa, 0xf, 0xb), 0x0afb);
    }
}

impl Chip8Machine {
    fn bad_instruction(&self, code: u16) -> String {
        format!(
            "Invalid instruction at {:#010x}: {:#06x}",
            self.prog_counter - 2,
            code
        )
    }

    pub(super) fn decode(&self, code: u16) -> Result<Chip8Inst, String> {
        let (a, b) = code.hi().split();
        let (c, d) = code.lo().split();
        match a {
            0x0 => match (b, c, d) {
                (0x0, 0xe, 0x0) => Ok(Chip8Inst::ClearScreen),
                (0x0, 0xe, 0xe) => Ok(Chip8Inst::SubReturn),
                _ => {
                    let n = make_usize(b, c, d);
                    Ok(Chip8Inst::MachineInst(n))
                }
            },
            0x1 => {
                let n = make_usize(b, c, d);
                Ok(Chip8Inst::Jump(n))
            }
            0x2 => {
                let n = make_usize(b, c, d);
                Ok(Chip8Inst::SubCall(n))
            }
            0x3 => {
                let n = (c << 4) | d;
                Ok(Chip8Inst::SkipEqConst(b as usize, n))
            }
            0x4 => {
                let n = (c << 4) | d;
                Ok(Chip8Inst::SkipNeqConst(b as usize, n))
            }
            0x5 => {
                if d == 0 {
                    Ok(Chip8Inst::SkipEqReg(b as usize, c as usize))
                } else {
                    Err(self.bad_instruction(code))
                }
            }
            0x6 => {
                let n = (c << 4) | d;
                Ok(Chip8Inst::RegSet(b as usize, n))
            }
            0x7 => {
                let n = (c << 4) | d;
                Ok(Chip8Inst::RegAddNoCarry(b as usize, n))
            }
            0x8 => match d {
                0x0 => Ok(Chip8Inst::Assign(b as usize, c as usize)),
                0x1 => Ok(Chip8Inst::BinOr(b as usize, c as usize)),
                0x2 => Ok(Chip8Inst::BinAnd(b as usize, c as usize)),
                0x3 => Ok(Chip8Inst::BinXor(b as usize, c as usize)),
                0x4 => Ok(Chip8Inst::ArithAdd(b as usize, c as usize)),
                0x5 => Ok(Chip8Inst::ArithSub(b as usize, c as usize)),
                0x6 => Ok(Chip8Inst::ShiftRight(b as usize, c as usize)),
                0x7 => Ok(Chip8Inst::ArithSubReverse(b as usize, c as usize)),
                0xe => Ok(Chip8Inst::ShiftLeft(b as usize, c as usize)),
                _ => Err(self.bad_instruction(code)),
            },
            0x9 => {
                if d == 0 {
                    Ok(Chip8Inst::SkipNeqReg(b as usize, c as usize))
                } else {
                    Err(self.bad_instruction(code))
                }
            }
            0xa => {
                let n = make_usize(b, c, d);
                Ok(Chip8Inst::SetIndex(n))
            }
            0xc => {
                let n = (c << 4) | d;
                Ok(Chip8Inst::Random(b as usize, n))
            }
            0xd => Ok(Chip8Inst::Display(b as usize, c as usize, d)),
            0xe => match (c, d) {
                (0x9, 0xe) => Ok(Chip8Inst::SkipEqKey(b as usize)),
                (0xa, 0x1) => Ok(Chip8Inst::SkipNeqKey(b as usize)),
                (0x0, 0xa) => Ok(Chip8Inst::GetKey(b as usize)),
                _ => Err(self.bad_instruction(code)),
            },
            0xf => match (c, d) {
                (0x0, 0x7) => Ok(Chip8Inst::ReadDelay(b as usize)),
                (0x1, 0x5) => Ok(Chip8Inst::SetDelay(b as usize)),
                (0x1, 0x8) => Ok(Chip8Inst::SetSound(b as usize)),
                (0x1, 0xe) => Ok(Chip8Inst::AddIndex(b as usize)),
                (0x2, 0x9) => Ok(Chip8Inst::LoadFont(b as usize)),
                (0x3, 0x3) => Ok(Chip8Inst::BCDConvert(b as usize)),
                (0x5, 0x5) => Ok(Chip8Inst::StoreMem(b as usize)),
                (0x6, 0x5) => Ok(Chip8Inst::LoadMem(b as usize)),
                _ => Err(self.bad_instruction(code)),
            },
            _ => Err(self.bad_instruction(code)),
        }
    }
}

#[cfg(test)]
mod decode_tests {
    use super::super::insts::Chip8Inst;
    use super::super::Chip8Machine;

    macro_rules! assert_decode {
        ($code:literal, $expected:expr) => {
            let vm = Chip8Machine::new();
            match vm.decode($code) {
                Err(_) => assert!(false),
                Ok(inst) => assert_eq!(inst, $expected),
            }
        };
    }

    macro_rules! assert_decode_fail {
        ($code:literal) => {
            let vm = Chip8Machine::new();
            match vm.decode($code) {
                Err(_) => assert!(true),
                Ok(_) => assert!(false),
            }
        };
    }

    #[test]
    fn test_decode_clear() {
        assert_decode!(0x0e0, Chip8Inst::ClearScreen);
    }

    #[test]
    fn test_decode_subreturn() {
        assert_decode!(0x00ee, Chip8Inst::SubReturn);
    }

    #[test]
    fn test_decode_machine_inst() {
        assert_decode!(0x0211, Chip8Inst::MachineInst(0x211));
    }

    #[test]
    fn test_decode_jump() {
        assert_decode!(0x1d2a, Chip8Inst::Jump(0xd2a));
    }

    #[test]
    fn test_decode_sub_call() {
        assert_decode!(0x2987, Chip8Inst::SubCall(0x987));
    }

    #[test]
    fn test_decode_skip_eq_const() {
        assert_decode!(0x3b3a, Chip8Inst::SkipEqConst(0xb, 0x3a));
    }
}
