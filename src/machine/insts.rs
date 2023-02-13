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
