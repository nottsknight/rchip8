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

    pub fn decode(&self, code: u16) -> Result<Chip8Inst, String> {
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
                    Err(self.bad_instruction(code))
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
                _ => Err(self.bad_instruction(code)),
            },
            0x9000 => {
                if n == 0 {
                    Ok(Chip8Inst::SkipNeqReg(x as usize, y as usize))
                } else {
                    Err(self.bad_instruction(code))
                }
            }
            0xa000 => Ok(Chip8Inst::SetIndex(nnn)),
            0xc000 => Ok(Chip8Inst::Random(x as usize, nn)),
            0xd000 => Ok(Chip8Inst::Display(x as usize, y as usize, n as u8)),
            0xe000 => match nn {
                0x9e => Ok(Chip8Inst::SkipEqKey(x as usize)),
                0xa1 => Ok(Chip8Inst::SkipNeqKey(x as usize)),
                0x0a => Ok(Chip8Inst::GetKey(x as usize)),
                _ => Err(self.bad_instruction(code)),
            },
            0xf000 => match nn {
                0x07 => Ok(Chip8Inst::ReadDelay(x as usize)),
                0x15 => Ok(Chip8Inst::SetDelay(x as usize)),
                0x18 => Ok(Chip8Inst::SetSound(x as usize)),
                0x1e => Ok(Chip8Inst::AddIndex(x as usize)),
                0x29 => Ok(Chip8Inst::LoadFont(x as usize)),
                0x33 => Ok(Chip8Inst::BCDConvert(x as usize)),
                0x55 => Ok(Chip8Inst::StoreMem(x as usize)),
                0x65 => Ok(Chip8Inst::LoadMem(x as usize)),
                _ => Err(self.bad_instruction(code)),
            },
            _ => Err(self.bad_instruction(code)),
        }
    }
}
