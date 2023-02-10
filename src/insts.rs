
pub enum Chip8Inst {
    ClearScreen,
    MachineInst,
    Jump(usize),
    SetRegister(usize, u8),
    AddRegister(usize, u8),
    SetIndex(usize),
    Display(u8, u8),
}
