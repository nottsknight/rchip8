#[derive(Debug, PartialEq, Eq)]
pub enum Chip8Inst {
    // Display
    ClearScreen,
    Display(usize, usize, u8),
    // Subroutines and jumps
    MachineInst(usize),
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
    ShiftLeft(usize, usize),
    ShiftRight(usize, usize),
    // Timers 
    ReadDelay(usize),
    SetDelay(usize),
    SetSound(usize),
    // Index reg
    SetIndex(usize),
    AddIndex(usize),
    // Random
    Random(usize, u8),
    // Keys 
    SkipEqKey(usize),
    SkipNeqKey(usize),
    GetKey(usize),
    // Memory
    LoadFont(usize),
    BCDConvert(usize),
    StoreMem(usize),
    LoadMem(usize),
}
