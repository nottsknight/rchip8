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

use std::fs::File;
use std::io::Read;
use std::sync::{
    atomic::{AtomicBool, AtomicU8},
    Arc, Condvar, Mutex,
};
use std::thread;
use std::time::Duration;

pub const DELAY_60HZ: u64 = 1_000_000_000 / 60;

pub const DELAY_1MHZ: u64 = 1_000_000_000 / 1000;

pub const DISPLAY_WIDTH: usize = 64;

pub const DISPLAY_HEIGHT: usize = 32;

pub const FONT_BASE: usize = 0x050;

pub const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(PartialEq, Eq)]
pub enum Chip8Mode {
    Original,
    Modern,
}

/// Type representing the display pixels.
type Display = [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT];

pub struct Chip8Machine {
    /// Flags whether to run in original or modern mode.
    mode: Chip8Mode,

    /// Total memory available to the machine.
    memory: [u8; 4096],
    /// Address stack for subroutines.
    stack: Vec<usize>,
    /// Address of next instruction to run.
    prog_counter: usize,

    /// 8-bit registers.
    registers: [u8; 16],
    /// 16-bit register.
    index_reg: usize,

    /// Current value of the delay timer.
    delay_timer: Arc<AtomicU8>,
    /// Current value of the sound timer.
    sound_timer: Arc<AtomicU8>,

    /// The current state of the display pixels.
    display: Arc<Mutex<Display>>,
    /// The current key being pressed along with its condition variable.
    current_key: Arc<(Mutex<Option<u8>>, Condvar)>,
    /// Flags whether the display needs to be redrawn or not.
    redraw: Arc<AtomicBool>,
}

impl Chip8Machine {
    pub fn new(
        mode: Chip8Mode,
        delay_timer: Arc<AtomicU8>,
        sound_timer: Arc<AtomicU8>,
        display: Arc<Mutex<Display>>,
        current_key: Arc<(Mutex<Option<u8>>, Condvar)>,
        redraw: Arc<AtomicBool>,
    ) -> Chip8Machine {
        let mut vm = Chip8Machine {
            mode,
            memory: [0; 4096],
            stack: Vec::new(),
            prog_counter: 0x200,
            registers: [0; 16],
            index_reg: 0,
            delay_timer,
            sound_timer,
            display,
            current_key,
            redraw,
        };

        vm.memory[FONT_BASE..FONT_BASE + 80].copy_from_slice(&FONT[..]);
        vm
    }

    /// Load the contents of the specified file into the machine's memory.
    pub fn load_rom(&mut self, filename: &str) -> std::io::Result<()> {
        let mut f = File::open(filename)?;
        f.read(&mut self.memory[self.prog_counter..])?;
        Ok(())
    }

    /// Start the VM running its currently loaded program.
    pub fn run_program(&mut self, frequency: Duration) {
        loop {
            let opcode = self.fetch();
            let inst = self.decode(opcode).unwrap();
            self.execute(inst);
            thread::sleep(frequency);
        }
    }

    /// Get the 16-bit opcode starting from the address stored in the program counter.
    fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.prog_counter] as u16;
        let lo = self.memory[self.prog_counter + 1] as u16;
        self.prog_counter += 2;
        (hi << 8) | lo
    }
}

mod carry_borrow;
mod decode;
mod execute;
mod insts;
