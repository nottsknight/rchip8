use crate::machine::insts::Chip8Inst;

pub fn disassemble(pc: Option<usize>, inst: Chip8Inst) -> String {
    let s = match inst {
        Chip8Inst::ClearScreen => "clr".to_string(),
        Chip8Inst::Display(x, y, height) => format!("draw    V{:x}, V{:x}, {:x}", x, y, height),
        Chip8Inst::MachineInst(nnn) => {
            let hi = (nnn & 0xff00) >> 8;
            let lo = nnn & 0xff;
            format!(".data   {:02x} {:02x}", hi, lo)
        }
        Chip8Inst::Jump(nnn) => format!("jmp     {:03x}", nnn),
        Chip8Inst::JumpReg(nnn) => format!("jmpv    {:03x}", nnn),
        Chip8Inst::SubCall(nnn) => format!("call    {:03x}", nnn),
        Chip8Inst::SubReturn => "retn".to_string(),
        Chip8Inst::SkipEqConst(x, nn) => format!("skipeq  V{:x}, {:02x}", x, nn),
        Chip8Inst::SkipNeqConst(x, nn) => format!("skipne  V{:x}, {:02x}", x, nn),
        Chip8Inst::SkipEqReg(x, y) => format!("skipeq  V{:x}, V{:x}", x, y),
        Chip8Inst::SkipNeqReg(x, y) => format!("skipne  V{:x}, V{:x}", x, y),
        Chip8Inst::RegSet(x, nn) => format!("mov     V{:x}, {:02x}", x, nn),
        Chip8Inst::RegAddNoCarry(x, nn) => format!("add     V{:x}, {:02x}", x, nn),
        Chip8Inst::Assign(x, y) => format!("mov     V{:x}, V{:x}", x, y),
        Chip8Inst::BinOr(x, y) => format!("or      V{:x}, V{:x}", x, y),
        Chip8Inst::BinAnd(x, y) => format!("and     V{:x}, V{:x}", x, y),
        Chip8Inst::BinXor(x, y) => format!("xor     V{:x}, V{:x}", x, y),
        Chip8Inst::ArithAdd(x, y) => format!("addc    V{:x}, V{:x}", x, y),
        Chip8Inst::ArithSub(x, y) => format!("sub     V{:x}, V{:x}", x, y),
        Chip8Inst::ArithSubReverse(x, y) => format!("subr    V{:x}, V{:x}", x, y),
        Chip8Inst::ShiftLeft(x, y) => format!("lshift  V{:x}, V{:x}", x, y),
        Chip8Inst::ShiftRight(x, y) => format!("rshift  V{:x}, V{:x}", x, y),
        Chip8Inst::ReadDelay(x) => format!("mov     V{:x}, D", x),
        Chip8Inst::SetDelay(x) => format!("mov     D, V{:x}", x),
        Chip8Inst::SetSound(x) => format!("mov     S, V{:x}", x),
        Chip8Inst::SetIndex(nnn) => format!("mov     I, {:03x}", nnn),
        Chip8Inst::AddIndex(nnn) => format!("add     I, {:03x}", nnn),
        Chip8Inst::Random(x, nn) => format!("rand    V{:x}, {:02x}", x, nn),
        Chip8Inst::SkipEqKey(x) => format!("skipeqk V{:x}", x),
        Chip8Inst::SkipNeqKey(x) => format!("skipnek V{:x}", x),
        Chip8Inst::GetKey(x) => format!("read    V{:x}", x),
        Chip8Inst::LoadFont(x) => format!("font    V{:x}", x),
        Chip8Inst::BCDConvert(x) => format!("bcd     V{:x}", x),
        Chip8Inst::StoreMem(x) => format!("str     V{:x}", x),
        Chip8Inst::LoadMem(x) => format!("load    V{:x}", x),
    };

    if let Some(pc_val) = pc {
        format!("{:#06x}    {}", pc_val, s)
    } else {
        format!("{}", s)
    }
}
