use crate::machine::insts::Chip8Inst;

impl From<Chip8Inst> for u16 {
    fn from(inst: Chip8Inst) -> u16 {
        match inst {
            Chip8Inst::ClearScreen => 0x00e0,
            Chip8Inst::SubReturn => 0x00ee,
            Chip8Inst::Jump(nnn) => 0x1000 | nnn as u16,
            Chip8Inst::SubCall(nnn) => 0x2000 | nnn as u16,
            Chip8Inst::SkipEqConst(x, nn) => 0x3000 | (x as u16) << 8 | nn as u16,
            Chip8Inst::SkipNeqConst(x, nn) => 0x4000 | (x as u16) << 8 | nn as u16,
            Chip8Inst::SkipEqReg(x, y) => 0x5000 | (x as u16) << 8 | (y as u16) << 4,
            _ => 0 
        }
    }
}
