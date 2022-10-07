use crate::hilo::*;

pub struct Instruction(pub u8, pub u8, pub u8, pub u8);

impl From<u16> for Instruction {
    fn from(code: u16) -> Instruction {
        let hi = code.hi();
        let lo = code.lo();
        Instruction(hi.hi(), hi.lo(), lo.hi(), lo.lo())
    }
}
