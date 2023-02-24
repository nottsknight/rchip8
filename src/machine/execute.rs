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

use super::carry_borrow::*;
use super::insts::Chip8Inst;
use super::Display;
use super::{Chip8Machine, Chip8Mode, DISPLAY_HEIGHT, DISPLAY_WIDTH, FONT_BASE};
use std::sync::atomic::Ordering;

impl Chip8Machine {
    /// Execute the given instruction.
    pub fn execute(&mut self, inst: Chip8Inst) {
        match inst {
            Chip8Inst::MachineInst(_) => (),
            Chip8Inst::ClearScreen => {
                let mut dsp = self.display.lock().unwrap();
                dsp.fill(false);
                self.redraw.store(true, Ordering::Release);
            }
            Chip8Inst::SubCall(n) => {
                self.stack.push(self.prog_counter);
                self.prog_counter = n;
            }
            Chip8Inst::SubReturn => match self.stack.pop() {
                Some(n) => self.prog_counter = n,
                None => panic!("Tried to pop an empty stack"),
            },
            Chip8Inst::Jump(n) => self.prog_counter = n,
            Chip8Inst::SetIndex(n) => self.index_reg = n,
            Chip8Inst::AddIndex(x) => {
                self.index_reg += self.registers[x] as usize;
                if self.index_reg >= 0x1000 {
                    self.registers[0xf] = 1;
                }
            }
            Chip8Inst::RegSet(x, n) => self.registers[x] = n,
            Chip8Inst::RegAddNoCarry(x, n) => {
                let m = self.registers[x];
                self.registers[x] = u8::add_no_carry(m, n);
            }
            Chip8Inst::SkipEqConst(x, n) => {
                if self.registers[x] == n {
                    self.prog_counter += 2;
                }
            }
            Chip8Inst::SkipNeqConst(x, n) => {
                if self.registers[x] != n {
                    self.prog_counter += 2;
                }
            }
            Chip8Inst::SkipEqReg(x, y) => {
                if self.registers[x] == self.registers[y] {
                    self.prog_counter += 2;
                }
            }
            Chip8Inst::SkipNeqReg(x, y) => {
                if self.registers[x] != self.registers[y] {
                    self.prog_counter += 2;
                }
            }
            Chip8Inst::Assign(x, y) => self.registers[x] = self.registers[y],
            Chip8Inst::BinOr(x, y) => self.registers[x] |= self.registers[y],
            Chip8Inst::BinAnd(x, y) => self.registers[x] &= self.registers[y],
            Chip8Inst::BinXor(x, y) => self.registers[x] ^= self.registers[y],
            Chip8Inst::ArithAdd(x, y) => {
                let (sum, carry) = u8::add_carry(self.registers[x], self.registers[y]);
                self.registers[x] = sum;
                self.registers[0xf] = if carry { 1 } else { 0 }
            }
            Chip8Inst::ArithSub(x, y) => {
                let (diff, borrow) = u8::sub_borrow(self.registers[x], self.registers[y]);
                self.registers[x] = diff;
                self.registers[0xf] = if borrow { 1 } else { 0 }
            }
            Chip8Inst::ArithSubReverse(x, y) => {
                let (diff, borrow) = u8::sub_borrow(self.registers[y], self.registers[x]);
                self.registers[x] = diff;
                self.registers[0xf] = if borrow { 1 } else { 0 }
            }
            Chip8Inst::ReadDelay(x) => {
                let delay = self.delay_timer.load(Ordering::Acquire);
                self.registers[x] = delay;
            }
            Chip8Inst::SetDelay(x) => {
                let delay = self.registers[x];
                self.delay_timer.store(delay, Ordering::Release);
            }
            Chip8Inst::SetSound(x) => {
                let sound = self.registers[x];
                self.sound_timer.store(sound, Ordering::Release);
            }
            Chip8Inst::Display(x_reg, y_reg, n) => {
                let mut x = (self.registers[x_reg] & 63) as usize;
                let mut y = (self.registers[y_reg] & 31) as usize;
                self.registers[0xf] = 0;

                let mut dsp = self.display.lock().unwrap();
                'rows: for i in 0..n {
                    let b = self.memory[self.index_reg + i as usize];
                    'cols: for j in 0..8 {
                        let px = b & (0x1 << (7 - j));
                        if set_display_pixel(&mut dsp, x as usize, y as usize, px != 0) {
                            self.registers[0xf] = 1;
                        }

                        x += 1;
                        if x >= DISPLAY_WIDTH {
                            break 'cols;
                        }
                    }

                    y += 1;
                    if y >= DISPLAY_HEIGHT {
                        break 'rows;
                    }
                    x = (self.registers[x_reg] & 63) as usize;
                }
                self.redraw.store(true, Ordering::Release);
            }
            Chip8Inst::Random(x, n) => {
                let r = rand::random::<u8>();
                self.registers[x] = n & r;
            }
            Chip8Inst::ShiftLeft(x, y) => {
                if self.mode == Chip8Mode::Original {
                    self.registers[x] = self.registers[y];
                }
                let (n1, overflow) = u8::shift_left(self.registers[x], 1);
                self.registers[x] = n1;
                self.registers[0xf] = if overflow { 1 } else { 0 };
            }
            Chip8Inst::ShiftRight(x, y) => {
                if self.mode == Chip8Mode::Original {
                    self.registers[x] = self.registers[y];
                }
                let (n1, underflow) = u8::shift_right(self.registers[x], 1);
                self.registers[x] = n1;
                self.registers[0xf] = if underflow { 1 } else { 0 };
            }
            Chip8Inst::SkipEqKey(x) => {
                let (lock, _) = &*self.current_key;
                let key = lock.lock().unwrap();
                if key.is_some() && self.registers[x] == key.unwrap() {
                    self.prog_counter += 2;
                }
            }
            Chip8Inst::SkipNeqKey(x) => {
                let (lock, _) = &*self.current_key;
                let key = lock.lock().unwrap();
                if key.is_some() && self.registers[x] != key.unwrap() {
                    self.prog_counter += 2;
                }
            }
            Chip8Inst::GetKey(x) => {
                let (lock, cvar) = &*self.current_key;
                let mut key = lock.lock().unwrap();
                key = cvar.wait(key).unwrap();
                if key.is_some() {
                    self.registers[x] = key.unwrap();
                }
            }
            Chip8Inst::LoadFont(x) => {
                let c = self.registers[x];
                self.index_reg = FONT_BASE + (5 * c) as usize;
            }
            Chip8Inst::BCDConvert(x) => {
                let n = self.registers[x];
                self.memory[self.index_reg] = n / 100;
                self.memory[self.index_reg + 1] = (n % 100) / 10;
                self.memory[self.index_reg + 2] = n % 10;
            }
            Chip8Inst::StoreMem(x) => match self.mode {
                Chip8Mode::Modern => {
                    for i in 0..x + 1 {
                        self.memory[self.index_reg + i] = self.registers[i];
                    }
                }
                Chip8Mode::Original => {
                    for i in 0..x + 1 {
                        self.memory[self.index_reg] = self.registers[i];
                        self.index_reg += 1;
                    }
                }
            },
            Chip8Inst::LoadMem(x) => match self.mode {
                Chip8Mode::Modern => {
                    for i in 0..x + 1 {
                        self.registers[i] = self.memory[self.index_reg + i];
                    }
                }
                Chip8Mode::Original => {
                    for i in 0..x + 1 {
                        self.registers[i] = self.memory[self.index_reg];
                        self.index_reg += 1;
                    }
                }
            },
        }

        let (lock, _) = &*self.current_key;
        let mut key = lock.lock().unwrap();
        *key = None;
    }
}

#[inline]
fn set_display_pixel(display: &mut Display, x: usize, y: usize, px: bool) -> bool {
    let px0 = display[y * DISPLAY_WIDTH + x];
    display[y * DISPLAY_WIDTH + x] = px0 ^ px;
    px0 && (px ^ px0)
}
