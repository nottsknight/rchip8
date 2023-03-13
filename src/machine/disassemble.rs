use crate::machine::insts::Chip8Inst;

pub fn disassemble(pc: Option<usize>, inst: Chip8Inst) -> String {
    let s = match inst {
        Chip8Inst::ClearScreen => "clear".to_string(),
        Chip8Inst::Display(x, y, height) => format!("draw {:x}, {:x}, {:x}", x, y, height),
        Chip8Inst::MachineInst(nnn) => format!("machine_code {:x}", nnn),
        Chip8Inst::Jump(nnn) => format!("jump {:x}", nnn),
        Chip8Inst::JumpReg(nnn) => format!("jump_v0 {:x}", nnn),
        Chip8Inst::SubCall(nnn) => format!("call {:x}", nnn),
        Chip8Inst::SubReturn => "return".to_string(),
        Chip8Inst::SkipEqConst(x, nn) => format!("skip_eq V{:x}, {:x}", x, nn),
        Chip8Inst::SkipNeqConst(x, nn) => format!("skip_neq V{:x}, {:x}", x, nn),
        Chip8Inst::SkipEqReg(x, y) => format!("skip_eq V{:x}, V{:x}", x, y),
        Chip8Inst::SkipNeqReg(x, y) => format!("skip_neq V{:x}, V{:x}", x, y),
        Chip8Inst::RegSet(x, nn) => format!("set V{:x}, {:x}", x, nn),
        Chip8Inst::RegAddNoCarry(x, nn) => format!("inc V{:x}, {:x}", x, nn),
        Chip8Inst::Assign(x, y) => format!("set V{:x}, V{:x}", x, y),
        Chip8Inst::BinOr(x, y) => format!("or V{:x}, V{:x}", x, y),
        Chip8Inst::BinAnd(x, y) => format!("and V{:x}, V{:x}", x, y),
        Chip8Inst::BinXor(x, y) => format!("xor V{:x}, V{:x}", x, y),
        Chip8Inst::ArithAdd(x, y) => format!("add V{:x}, V{:x}", x, y),
        Chip8Inst::ArithSub(x, y) => format!("sub {:x}, V{:x}", x, y),
        Chip8Inst::ArithSubReverse(x, y) => format!("sub_reverse V{:x}, V{:x}", x, y),
        Chip8Inst::ShiftLeft(x, y) => format!("lshift V{:x}, V{:x}", x, y),
        Chip8Inst::ShiftRight(x, y) => format!("rshift V{:x}, V{:x}", x, y),
        Chip8Inst::ReadDelay(x) => format!("read_delay V{:x}", x),
        Chip8Inst::SetDelay(x) => format!("set_delay V{:x}", x),
        Chip8Inst::SetSound(x) => format!("set_sound V{:x}", x),
        Chip8Inst::SetIndex(nnn) => format!("set_index {:x}", nnn),
        Chip8Inst::AddIndex(nnn) => format!("add_index {:x}", nnn),
        Chip8Inst::Random(x, nn) => format!("random V{:x}, {:x}", x, nn),
        Chip8Inst::SkipEqKey(x) => format!("skip_key_eq V{:x}", x),
        Chip8Inst::SkipNeqKey(x) => format!("skip_key_neq V{:x}", x),
        Chip8Inst::GetKey(x) => format!("get_key V{:x}", x),
        Chip8Inst::LoadFont(x) => format!("load_font V{:x}", x),
        Chip8Inst::BCDConvert(x) => format!("bcd V{:x}", x),
        Chip8Inst::StoreMem(x) => format!("store V{:x}", x),
        Chip8Inst::LoadMem(x) => format!("load V{:x}", x),
    };

    if pc.is_some() {
        format!("{:#06x}: {}", pc.unwrap(), s)
    } else {
        format!("{}", s)
    }
}
