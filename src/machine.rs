use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;

use crate::carry_borrow::{AddCarry, ShiftOverflow, SubBorrow};
use crate::hilo::HiLo;
use crate::insts::Chip8Inst;

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

const DISPLAY_ROWS: usize = 32;

const DISPLAY_COLS: usize = 64;

pub struct VirtualMachine {
    memory: [u8; 4096],
    display: [[bool; DISPLAY_COLS]; DISPLAY_ROWS],
    prog_counter: usize,
    index_reg: usize,
    stack: Vec<usize>,
    delay_timer: Arc<AtomicU8>,
    sound_timer: Arc<AtomicU8>,
    registers: [u8; 16],
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        let mut memory_array = [0; 4096];
        memory_array[0x050..0x0a0].copy_from_slice(&FONT[..]);

        VirtualMachine {
            memory: memory_array,
            display: [[false; DISPLAY_COLS]; DISPLAY_ROWS],
            prog_counter: 0x200,
            index_reg: 0,
            stack: Vec::new(),
            delay_timer: Arc::new(AtomicU8::new(0)),
            sound_timer: Arc::new(AtomicU8::new(0)),
            registers: [0; 16],
        }
    }

    fn print_display(&self) {
        print!("{}", cursor::Goto(1, 1));

        let mut display_str = String::from("");
        for row in self.display {
            for col in row {
                if col {
                    display_str.push('\u{2588}');
                } else {
                    display_str.push(' ');
                }
            }
            display_str.push('\n');
        }
        println!("{}", display_str);
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.prog_counter] as u16;
        let lo = self.memory[self.prog_counter + 1] as u16;
        self.prog_counter += 2;
        (hi << 8) | lo
    }

    fn decode(&self, code: u16) -> Result<Chip8Inst, String> {
        let (a, b) = code.hi().split();
        let (c, d) = code.lo().split();
        match a {
            0x0 => match (b, c, d) {
                (0x0, 0xe, 0x0) => Ok(Chip8Inst::ClearScreen),
                (0x0, 0xe, 0xe) => Ok(Chip8Inst::SubReturn),
                _ => Err(self.bad_instruction(code)),
            },
            0x1 => {
                let n = make_usize(b, c, d);
                Ok(Chip8Inst::Jump(n))
            }
            0x2 => {
                let n = make_usize(b, c, d);
                Ok(Chip8Inst::SubCall(n))
            }
            0x3 => {
                let n = (c << 4) | d;
                Ok(Chip8Inst::SkipEqConst(b as usize, n))
            }
            0x4 => {
                let n = (c << 4) | d;
                Ok(Chip8Inst::SkipNeqConst(b as usize, n))
            }
            0x5 => {
                if d == 0 {
                    Ok(Chip8Inst::SkipEqReg(b as usize, c as usize))
                } else {
                    Err(self.bad_instruction(code))
                }
            }
            0x6 => {
                let n = (c << 4) | d;
                Ok(Chip8Inst::RegSet(b as usize, n))
            }
            0x7 => {
                let n = (c << 4) | d;
                Ok(Chip8Inst::RegAddNoCarry(b as usize, n))
            }
            0x8 => match d {
                0x0 => Ok(Chip8Inst::Assign(b as usize, c as usize)),
                0x1 => Ok(Chip8Inst::BinOr(b as usize, c as usize)),
                0x2 => Ok(Chip8Inst::BinAnd(b as usize, c as usize)),
                0x3 => Ok(Chip8Inst::BinXor(b as usize, c as usize)),
                0x4 => Ok(Chip8Inst::ArithAdd(b as usize, c as usize)),
                0x5 => Ok(Chip8Inst::ArithSub(b as usize, c as usize)),
                0x6 => Ok(Chip8Inst::ShiftRight(b as usize, c as usize)),
                0x7 => Ok(Chip8Inst::ArithSubReverse(b as usize, c as usize)),
                0xe => Ok(Chip8Inst::ShiftLeft(b as usize, c as usize)),
                _ => Err(self.bad_instruction(code)),
            },
            0x9 => {
                if d == 0 {
                    Ok(Chip8Inst::SkipNeqReg(b as usize, c as usize))
                } else {
                    Err(self.bad_instruction(code))
                }
            }
            0xa => {
                let n = make_usize(b, c, d);
                Ok(Chip8Inst::SetIndex(n))
            }
            0xc => {
                let n = (c << 4) | d;
                Ok(Chip8Inst::Random(b as usize, n))
            }
            0xd => Ok(Chip8Inst::Display(b as usize, c as usize, d)),
            0xe => match (c, d) {
                (0x9, 0xe) => Ok(Chip8Inst::SkipEqKey(b as usize)),
                (0xa, 0x1) => Ok(Chip8Inst::SkipNeqKey(b as usize)),
                (0x0, 0xa) => Ok(Chip8Inst::GetKey(b as usize)),
                _ => Err(self.bad_instruction(code)),
            }
            0xf => match (c, d) {
                (0x0, 0x7) => Ok(Chip8Inst::ReadDelay(b as usize)),
                (0x1, 0x5) => Ok(Chip8Inst::SetDelay(b as usize)),
                (0x1, 0x8) => Ok(Chip8Inst::SetSound(b as usize)),
                (0x1, 0xe) => Ok(Chip8Inst::AddIndex(b as usize)),
                _ => Err(self.bad_instruction(code)),
            },
            _ => Err(self.bad_instruction(code)),
        }
    }

    fn bad_instruction(&self, code: u16) -> String {
        format!(
            "Invalid instruction at {:#010x}: {:#06x}",
            self.prog_counter - 2,
            code
        )
    }

    fn get_keydown(&self) -> Option<u8> {
        let mut keys = std::io::stdin().keys();
        let k = keys.nth(0).unwrap().unwrap();
        return match k {
            Key::Char('1') => Some(0x1),
            Key::Char('2') => Some(0x2),
            Key::Char('3') => Some(0x3),
            Key::Char('4') => Some(0xc),
            Key::Char('q') => Some(0x4),
            Key::Char('w') => Some(0x5),
            Key::Char('e') => Some(0x6),
            Key::Char('r') => Some(0xd),
            Key::Char('a') => Some(0x7),
            Key::Char('s') => Some(0x8),
            Key::Char('d') => Some(0x9),
            Key::Char('f') => Some(0xe),
            Key::Char('z') => Some(0xa),
            Key::Char('x') => Some(0x0),
            Key::Char('c') => Some(0xb),
            Key::Char('v') => Some(0xf),
            _ => None,
        };
    }

    fn execute(&mut self, inst: Chip8Inst) {
        match inst {
            Chip8Inst::MachineInst => (),
            Chip8Inst::ClearScreen => {
                for mut row in self.display {
                    row.fill(false);
                }
                self.print_display();
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
                let n = self.delay_timer.load(Ordering::Acquire);
                self.registers[x] = n;
            }
            Chip8Inst::SetDelay(x) => {
                self.delay_timer.store(self.registers[x], Ordering::Release);
            }
            Chip8Inst::SetSound(x) => {
                self.sound_timer.store(self.registers[x], Ordering::Release);
            }
            Chip8Inst::Display(x_reg, y_reg, n) => {
                let mut x = (self.registers[x_reg] & 63) as usize;
                let mut y = (self.registers[y_reg] & 31) as usize;
                self.registers[0xf] = 0;

                for i in 0..n {
                    let b = self.memory[self.index_reg + i as usize];
                    let bs = [
                        b & 0x80,
                        b & 0x40,
                        b & 0x20,
                        b & 0x10,
                        b & 0x8,
                        b & 0x4,
                        b & 0x2,
                        b & 0x1,
                    ];
                    for j in 0..8 {
                        if self.display[y - 1][x - 1] && bs[j] != 0 {
                            self.display[y - 1][x - 1] = false;
                        } else if !self.display[y - 1][x - 1] && bs[j] != 0 {
                            self.display[y - 1][x - 1] = true;
                        }

                        x += 1;
                        if x - 2 >= DISPLAY_COLS {
                            break;
                        }
                    }
                    y += 1;
                    if y - 2 >= DISPLAY_ROWS {
                        break;
                    }
                    x = (self.registers[x_reg] & 63) as usize;
                }
                self.print_display();
            }
            Chip8Inst::Random(x, n) => {
                let r = rand::random::<u8>();
                self.registers[x] = n & r;
            }
            Chip8Inst::ShiftLeft(x, y) => {
                let n = self.registers[y];
                let (n1, overflow) = u8::shift_left(n, 1);
                self.registers[x] = n1;
                self.registers[0xf] = if overflow { 1 } else { 0 };
            }
            Chip8Inst::ShiftRight(x, y) => {
                let n = self.registers[y];
                let (n1, underflow) = u8::shift_right(n, 1);
                self.registers[x] = n1;
                self.registers[0xf] = if underflow { 1 } else { 0 };
            }
            Chip8Inst::SkipEqKey(x) => {
                match self.get_keydown() {
                    None => (),
                    Some(k) => {
                        if self.registers[x] == k {
                            self.prog_counter += 2;
                        }
                    }
                }
            }
            Chip8Inst::SkipNeqKey(x) => {
                match self.get_keydown() {
                    None => (),
                    Some(k) => {
                        if self.registers[x] != k {
                            self.prog_counter += 2;
                        }
                    }
                }
            }
            Chip8Inst::GetKey(x) => {
                loop {
                    match self.get_keydown() {
                        None => (),
                        Some(k1) => {
                            self.registers[x] = k1;
                            break;
                        }
                    }
                }
            }
        }
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
            let freq = Duration::from_nanos(16667);

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

        // fetch-decode-execute loop
        let cpu_freq = Duration::from_nanos(250);
        loop {
            let code = self.fetch();
            match self.decode(code) {
                Ok(inst) => self.execute(inst),
                Err(e) => {
                    print!("{}", cursor::Goto(1, 1));
                    panic!("{}", e);
                }
            }
            thread::sleep(cpu_freq);
        }
    }
}

fn make_usize(x: u8, y: u8, z: u8) -> usize {
    ((x as usize) << 8) | ((y as usize) << 4) | z as usize
}
