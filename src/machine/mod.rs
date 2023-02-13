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
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use termion::clear;
use termion::cursor;

mod carry_borrow;
mod decode;
mod execute;
mod hilo;
mod insts;
mod display;

const FONT: [u8; 80] = [
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

const FONT_BASE: usize = 0x050;

const DISPLAY_ROWS: usize = 32;

const DISPLAY_COLS: usize = 64;

#[derive(PartialEq, Eq)]
pub enum Chip8Mode {
    Original,
    Modern
}

pub struct Chip8Machine {
    mode: Chip8Mode,
    memory: [u8; 4096],
    display: Arc<Mutex<[[bool; DISPLAY_COLS]; DISPLAY_ROWS]>>,
    prog_counter: usize,
    index_reg: usize,
    stack: Vec<usize>,
    delay_timer: Arc<AtomicU8>,
    sound_timer: Arc<AtomicU8>,
    registers: [u8; 16],
}

impl Chip8Machine {
    pub fn new(mode: Chip8Mode) -> Chip8Machine {
        let mut memory = [0; 4096];
        memory[FONT_BASE..FONT_BASE + 80].copy_from_slice(&FONT[..]);

        Chip8Machine {
            mode,
            memory,
            display: Arc::new(Mutex::new([[false; DISPLAY_COLS]; DISPLAY_ROWS])),
            prog_counter: 0x200,
            index_reg: 0,
            stack: Vec::new(),
            delay_timer: Arc::new(AtomicU8::new(0)),
            sound_timer: Arc::new(AtomicU8::new(0)),
            registers: [0; 16],
        }
    }

    fn print_display(&self) {
        let mut display_str = String::from("");
        let grid = self.display.lock().unwrap();
        for row in *grid {
            for col in row {
                if col {
                    display_str.push('\u{2588}');
                } else {
                    display_str.push(' ');
                }
            }
            display_str.push('\n');
        }
        print!("{}{}", cursor::Goto(1, 1), display_str);
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.prog_counter] as u16;
        let lo = self.memory[self.prog_counter + 1] as u16;
        self.prog_counter += 2;
        (hi << 8) | lo
    }

    pub fn load_program(&mut self, fname: &str) -> std::io::Result<()> {
        let mut f = File::open(fname)?;
        f.read(&mut self.memory[0x200..])?;
        Ok(())
    }

    pub fn run(&mut self) {
        print!("{}{}", clear::All, cursor::Goto(1, 1));

        // start timers
        let delay_clone = Arc::clone(&self.delay_timer);
        let sound_clone = Arc::clone(&self.sound_timer);

        thread::spawn(move || {
            let freq = Duration::from_nanos(16_666_667);

            loop {
                let d = delay_clone.load(Ordering::Acquire);
                if d > 0 {
                    delay_clone.store(d - 1, Ordering::Release);
                }

                let d = sound_clone.load(Ordering::Acquire);
                if d > 0 {
                    sound_clone.store(d - 1, Ordering::Release);
                }

                thread::sleep(freq);
            }
        });

        self.run_display();

        // fetch-decode-execute loop
        let cpu_freq = Duration::from_nanos(1_428_571);
        loop {
            let code = self.fetch();
            match self.decode(code) {
                Ok(inst) => self.execute(inst),
                Err(e) => panic!("{}", e),
            }
            thread::sleep(cpu_freq);
        }
    }
}
