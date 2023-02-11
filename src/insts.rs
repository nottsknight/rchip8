#[derive(Debug)]
pub enum Chip8Inst {
    // Display
    ClearScreen,
    Display(u8, u8),
    // Subroutines and jumps
    MachineInst,
    Jump(usize),
    SubCall(usize),
    SubReturn,
    // Skips
    SkipEqConst(usize, u8),
    SkipNeqConst(usize, u8),
    SkipEqReg(usize, usize),
    SkipNeqReg(usize, usize),
    // Register ops
    RegSet(usize, u8),
    RegAddNoCarry(usize, u8),
    // Arithmetic and logic
    Assign(usize, usize),
    BinOr(usize, usize),
    BinAnd(usize, usize),
    BinXor(usize, usize),
    ArithAdd(usize, usize),
    ArithSub(usize, usize),
    ArithSubReverse(usize, usize),
    // Timers 
    ReadDelay(usize),
    SetDelay(usize),
    SetSound(usize),
    SetIndex(usize),
}
