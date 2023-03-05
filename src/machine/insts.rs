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
    /// Set all display bits to false.
    ClearScreen,
    /// Draw some rows of the currently loaded character on screen.
    Display(usize, usize, u8),

    /// Call the machine language routine at the specified address.
    MachineInst(usize),
    /// Jump to the specified address.
    Jump(usize),
    /// Jump to the address found by adding the given value to V0.
    JumpReg(usize),
    /// Call the subroutine that begins at the specified address.
    SubCall(usize),
    /// Return from the current subroutine.
    SubReturn,

    /// Skip the next instruction if a register is equal to a constant value.
    SkipEqConst(usize, u8),
    /// Skip the next instruction if a register is not equal to a constant value.
    SkipNeqConst(usize, u8),
    /// Skip the next instruction if two registers are equal.
    SkipEqReg(usize, usize),
    /// Skip the next instruction if two registers are not equal.
    SkipNeqReg(usize, usize),

    /// Set a register to a constant value.
    RegSet(usize, u8),
    /// Add a constant to the value of a register, ignoring any carries.
    RegAddNoCarry(usize, u8),

    /// Set one register to another register.
    Assign(usize, usize),
    /// Set one register to the bitwise-OR of two registers.
    BinOr(usize, usize),
    /// Set one register to the bitwise-AND of two registers.
    BinAnd(usize, usize),
    /// Set one register to the bitwise-XOR of two registers.
    BinXor(usize, usize),
    /// Add two registers together, respecting carrying.
    ArithAdd(usize, usize),
    /// Subtract two registers, respecting borrowing.
    ArithSub(usize, usize),
    /// Subtract two registers in reverse order.
    ArithSubReverse(usize, usize),
    /// Logically shift a register left.
    ShiftLeft(usize, usize),
    /// Logically shift a register right.
    ShiftRight(usize, usize),

    /// Get the current value of the delay timer.
    ReadDelay(usize),
    /// Set the value of the delay timer.
    SetDelay(usize),
    /// Set the value of the sound timer.
    SetSound(usize),

    /// Set the index register.
    SetIndex(usize),
    /// Add a constant value to the index register.
    AddIndex(usize),

    /// Set a register to a random value bitwise-ANDed with a register.
    Random(usize, u8),

    /// Skip the next instruction if a specified key is being pressed.
    SkipEqKey(usize),
    /// Skip the next instruction if a specified key is not being pressed.
    SkipNeqKey(usize),
    /// Block until a key is pressed.
    GetKey(usize),

    /// Load a font character.
    LoadFont(usize),
    /// Write the binary-coded decimal representation of a register into memory.
    BCDConvert(usize),
    /// Write the contents of several registers to sequential locations in memory.
    StoreMem(usize),
    /// Read several locations in memory into sequential registers.
    LoadMem(usize),
}
