use crate::machine::insts::Chip8Inst;

pub fn disassemble(pc: Option<usize>, inst: Chip8Inst) -> String {
    let s = match inst {
        Chip8Inst::ClearScreen => "clr".to_string(),
        Chip8Inst::Display(x, y, height) => format!("draw    {:X}h, {:X}h, {:X}h", x, y, height),
        Chip8Inst::MachineInst(nnn) => format!("mc      {:X}h", nnn),
        Chip8Inst::Jump(nnn) => format!("jmp     {:X}h", nnn),
        Chip8Inst::JumpReg(nnn) => format!("jmpv   , {:X}h", nnn),
        Chip8Inst::SubCall(nnn) => format!("call    {:X}h", nnn),
        Chip8Inst::SubReturn => "retn".to_string(),
        Chip8Inst::SkipEqConst(x, nn) => format!("skipeq  V{:X}, {:X}h", x, nn),
        Chip8Inst::SkipNeqConst(x, nn) => format!("skipne  V{:X}, {:X}h", x, nn),
        Chip8Inst::SkipEqReg(x, y) => format!("skipeq  V{:X}, V{:X}", x, y),
        Chip8Inst::SkipNeqReg(x, y) => format!("skipne  V{:X}, V{:X}", x, y),
        Chip8Inst::RegSet(x, nn) => format!("mov     V{:X}, {:X}h", x, nn),
        Chip8Inst::RegAddNoCarry(x, nn) => format!("add     V{:X}, {:X}h", x, nn),
        Chip8Inst::Assign(x, y) => format!("mov     V{:X}, V{:X}", x, y),
        Chip8Inst::BinOr(x, y) => format!("or      V{:X}, V{:X}", x, y),
        Chip8Inst::BinAnd(x, y) => format!("and     V{:X}, V{:X}", x, y),
        Chip8Inst::BinXor(x, y) => format!("xor     V{:X}, V{:X}", x, y),
        Chip8Inst::ArithAdd(x, y) => format!("add     V{:X}, V{:X}", x, y),
        Chip8Inst::ArithSub(x, y) => format!("sub     {:X}, V{:X}", x, y),
        Chip8Inst::ArithSubReverse(x, y) => format!("subr    V{:X}, V{:X}", x, y),
        Chip8Inst::ShiftLeft(x, y) => format!("shl     V{:X}, V{:X}", x, y),
        Chip8Inst::ShiftRight(x, y) => format!("shr     V{:X}, V{:X}", x, y),
        Chip8Inst::ReadDelay(x) => format!("mov     V{:X}, D", x),
        Chip8Inst::SetDelay(x) => format!("mov     D, V{:X}", x),
        Chip8Inst::SetSound(x) => format!("mov     S, V{:X}", x),
        Chip8Inst::SetIndex(nnn) => format!("mov     I, {:X}h", nnn),
        Chip8Inst::AddIndex(nnn) => format!("add     I, {:X}h", nnn),
        Chip8Inst::Random(x, nn) => format!("rand    V{:X}, {:X}h", x, nn),
        Chip8Inst::SkipEqKey(x) => format!("skipeqk V{:X}", x),
        Chip8Inst::SkipNeqKey(x) => format!("skipnek V{:X}", x),
        Chip8Inst::GetKey(x) => format!("read    V{:X}", x),
        Chip8Inst::LoadFont(x) => format!("font    V{:X}", x),
        Chip8Inst::BCDConvert(x) => format!("bcd     V{:X}", x),
        Chip8Inst::StoreMem(x) => format!("str     V{:X}", x),
        Chip8Inst::LoadMem(x) => format!("load    V{:X}", x),
    };

    if pc.is_some() {
        format!("{:#06x}: {}", pc.unwrap(), s)
    } else {
        format!("{}", s)
    }
}
