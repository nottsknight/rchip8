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

use super::{insts::Chip8Inst, Chip8Machine};

impl Chip8Machine {
    fn bad_instruction(&self, code: u16) -> String {
        format!(
            "Invalid instruction at {:#010x}: {:#06x}",
            self.prog_counter - 2,
            code
        )
    }

    pub fn decode_run(&self, code: u16) -> Result<Chip8Inst, String> {
        if let Ok(inst) = Chip8Machine::decode(code) {
            Ok(inst)
        } else {
            Err(self.bad_instruction(code))
        }
    }

    /// Convert the given opcode into the appropriate `Chip8Inst`.
    pub fn decode(code: u16) -> Result<Chip8Inst, ()> {
        let x = (code & 0x0f00) >> 8;
        let y = (code & 0x00f0) >> 4;
        let n = code & 0x000f;
        let nn = (code & 0x00ff) as u8;
        let nnn = (code & 0x0fff) as usize;

        match code & 0xf000 {
            0x0000 => match nnn {
                0x0e0 => Ok(Chip8Inst::ClearScreen),
                0x0ee => Ok(Chip8Inst::SubReturn),
                _ => Ok(Chip8Inst::MachineInst(nnn)),
            },
            0x1000 => Ok(Chip8Inst::Jump(nnn)),
            0x2000 => Ok(Chip8Inst::SubCall(nnn)),
            0x3000 => Ok(Chip8Inst::SkipEqConst(x as usize, nn)),
            0x4000 => Ok(Chip8Inst::SkipNeqConst(x as usize, nn)),
            0x5000 => {
                if n == 0 {
                    Ok(Chip8Inst::SkipEqReg(x as usize, y as usize))
                } else {
                    Err(())
                }
            }
            0x6000 => Ok(Chip8Inst::RegSet(x as usize, nn)),
            0x7000 => Ok(Chip8Inst::RegAddNoCarry(x as usize, nn)),
            0x8000 => match n {
                0x0 => Ok(Chip8Inst::Assign(x as usize, y as usize)),
                0x1 => Ok(Chip8Inst::BinOr(x as usize, y as usize)),
                0x2 => Ok(Chip8Inst::BinAnd(x as usize, y as usize)),
                0x3 => Ok(Chip8Inst::BinXor(x as usize, y as usize)),
                0x4 => Ok(Chip8Inst::ArithAdd(x as usize, y as usize)),
                0x5 => Ok(Chip8Inst::ArithSub(x as usize, y as usize)),
                0x6 => Ok(Chip8Inst::ShiftRight(x as usize, y as usize)),
                0x7 => Ok(Chip8Inst::ArithSubReverse(x as usize, y as usize)),
                0xe => Ok(Chip8Inst::ShiftLeft(x as usize, y as usize)),
                _ => Err(()),
            },
            0x9000 => {
                if n == 0 {
                    Ok(Chip8Inst::SkipNeqReg(x as usize, y as usize))
                } else {
                    Err(())
                }
            }
            0xa000 => Ok(Chip8Inst::SetIndex(nnn)),
            0xb000 => Ok(Chip8Inst::JumpReg(nnn)),
            0xc000 => Ok(Chip8Inst::Random(x as usize, nn)),
            0xd000 => Ok(Chip8Inst::Display(x as usize, y as usize, n as u8)),
            0xe000 => match nn {
                0x9e => Ok(Chip8Inst::SkipEqKey(x as usize)),
                0xa1 => Ok(Chip8Inst::SkipNeqKey(x as usize)),
                0x0a => Ok(Chip8Inst::GetKey(x as usize)),
                _ => Err(()),
            },
            0xf000 => match nn {
                0x07 => Ok(Chip8Inst::ReadDelay(x as usize)),
                0x0a => Ok(Chip8Inst::GetKey(x as usize)),
                0x15 => Ok(Chip8Inst::SetDelay(x as usize)),
                0x18 => Ok(Chip8Inst::SetSound(x as usize)),
                0x1e => Ok(Chip8Inst::AddIndex(x as usize)),
                0x29 => Ok(Chip8Inst::LoadFont(x as usize)),
                0x33 => Ok(Chip8Inst::BCDConvert(x as usize)),
                0x55 => Ok(Chip8Inst::StoreMem(x as usize)),
                0x65 => Ok(Chip8Inst::LoadMem(x as usize)),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod decode_tests {
    use super::*;
    use crate::machine::{Chip8Mode, DISPLAY_HEIGHT, DISPLAY_WIDTH};
    use rstest::*;
    use std::sync::atomic::AtomicBool;
    use std::sync::{Arc, Condvar, Mutex};

    #[fixture]
    fn vm() -> Chip8Machine {
        const NEW_BOOL: AtomicBool = AtomicBool::new(false);
        Chip8Machine::new(
            Chip8Mode::Modern,
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new([false; DISPLAY_WIDTH * DISPLAY_HEIGHT])),
            Arc::new([NEW_BOOL; DISPLAY_WIDTH * DISPLAY_HEIGHT]),
            Arc::new([NEW_BOOL; 16]),
            Arc::new((Mutex::new(None), Condvar::new())),
        )
    }

    #[rstest]
    #[case::machine_inst(0x0162, Chip8Inst::MachineInst(0x162))]
    #[case::clear_screen(0x00e0, Chip8Inst::ClearScreen)]
    #[case::sub_return(0x00ee, Chip8Inst::SubReturn)]
    #[case::jump(0x1af2, Chip8Inst::Jump(0xaf2))]
    #[case::subroutine(0x2cc3, Chip8Inst::SubCall(0xcc3))]
    #[case::skip_eq_const(0x3b27, Chip8Inst::SkipEqConst(0xb, 0x27))]
    #[case::skip_neq_const(0x4b27, Chip8Inst::SkipNeqConst(0xb, 0x27))]
    #[case::skip_eq_reg(0x5c40, Chip8Inst::SkipEqReg(0xc, 0x4))]
    #[case::set_reg_const(0x68f5, Chip8Inst::RegSet(0x8, 0xf5))]
    #[case::reg_add(0x7b43, Chip8Inst::RegAddNoCarry(0xb, 0x43))]
    #[case::set_reg_reg(0x83e0, Chip8Inst::Assign(0x3, 0xe))]
    #[case::set_reg_or(0x8d21, Chip8Inst::BinOr(0xd, 0x2))]
    #[case::set_reg_and(0x83e2, Chip8Inst::BinAnd(0x3, 0xe))]
    #[case::set_reg_xor(0x87a3, Chip8Inst::BinXor(0x7, 0xa))]
    #[case::arith_add(0x87a4, Chip8Inst::ArithAdd(0x7, 0xa))]
    #[case::arith_sub(0x87a5, Chip8Inst::ArithSub(0x7, 0xa))]
    #[case::shift_right(0x87a6, Chip8Inst::ShiftRight(0x7, 0xa))]
    #[case::arith_sub_reverse(0x87a7, Chip8Inst::ArithSubReverse(0x7, 0xa))]
    #[case::shift_left(0x87ae, Chip8Inst::ShiftLeft(0x7, 0xa))]
    #[case::skip_neq_reg(0x9b30, Chip8Inst::SkipNeqReg(0xb, 0x3))]
    #[case::set_index(0xa2c3, Chip8Inst::SetIndex(0x2c3))]
    #[case::jump_add(0xb2f1, Chip8Inst::JumpReg(0x2f1))]
    #[case::random(0xc243, Chip8Inst::Random(0x2, 0x43))]
    #[case::display(0xdf4b, Chip8Inst::Display(0xf, 0x4, 0xb))]
    #[case::skip_eq_key(0xe49e, Chip8Inst::SkipEqKey(0x4))]
    #[case::skip_neq_key(0xe5a1, Chip8Inst::SkipNeqKey(0x5))]
    #[case::get_delay(0xf207, Chip8Inst::ReadDelay(0x2))]
    #[case::get_key(0xf70a, Chip8Inst::GetKey(0x7))]
    #[case::set_delay(0xf915, Chip8Inst::SetDelay(0x9))]
    #[case::set_sound(0xf218, Chip8Inst::SetSound(0x2))]
    #[case::add_index(0xfa1e, Chip8Inst::AddIndex(0xa))]
    #[case::load_font(0xf729, Chip8Inst::LoadFont(0x7))]
    #[case::bcd(0xfb33, Chip8Inst::BCDConvert(0xb))]
    #[case::reg_store(0xf955, Chip8Inst::StoreMem(0x9))]
    #[case::reg_load(0xf965, Chip8Inst::LoadMem(0x9))]
    fn test_decode_success(#[case] input: u16, #[case] expected: Chip8Inst) {
        assert_eq!(expected, Chip8Machine::decode(input).unwrap());
    }

    #[rstest]
    fn test_decode_fail(
        #[values(
            0x5121, 0x5122, 0x5123, 0x5124, 0x5125, 0x5126, 0x5127, 0x5128, 0x5129, 0x512a, 0x512b,
            0x512c, 0x512d, 0x512e, 0x512f, 0x82e8, 0x82e9, 0x82ea, 0x82eb, 0x82ec, 0x82ed, 0x82ef,
            0x9b31, 0x9b32, 0x9b33, 0x9b34, 0x9b35, 0x9b36, 0x9b37, 0x9b38, 0x9b39, 0x9b3a, 0x9b3b,
            0x9b3c, 0x9b3d, 0x9b3e, 0x9b3f
        )]
        code: u16,
    ) {
        assert!(Chip8Machine::decode(code).is_err());
    }
}
