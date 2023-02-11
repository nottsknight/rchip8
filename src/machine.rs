use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::add_carry::AddCarry;
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

const DEBUG: bool = true;

pub struct VirtualMachine {
    memory: [u8; 4096],
    display: [bool; 64 * 32],
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
        for i in 0x050..0x09f {
            memory_array[i] = FONT[i - 0x050];
        }

        VirtualMachine {
            memory: memory_array,
            display: [false; 64 * 32],
            prog_counter: 0x200,
            index_reg: 0,
            stack: Vec::new(),
            delay_timer: Arc::new(AtomicU8::new(0)),
            sound_timer: Arc::new(AtomicU8::new(0)),
            registers: [0; 16],
        }
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.prog_counter] as u16;
        let lo = self.memory[self.prog_counter + 1] as u16;
        self.prog_counter += 2;
        (hi << 8) | lo
    }

    fn decode(&mut self, code: u16) -> Result<Chip8Inst, String> {
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
                0x7 => Ok(Chip8Inst::ArithSubReverse(b as usize, c as usize)),
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
            0xf => match (c, d) {
                (0x0, 0x7) => Ok(Chip8Inst::ReadDelay(b as usize)),
                (0x1, 0x5) => Ok(Chip8Inst::SetDelay(b as usize)),
                (0x1, 0x8) => Ok(Chip8Inst::SetSound(b as usize)),
                _ => Err(self.bad_instruction(code)),
            },
            _ => Err(self.bad_instruction(code)),
        }
    }

    fn bad_instruction(&self, code: u16) -> String {
        format!(
            "Invalid instruction at {:#010x}: {:#06x}",
            self.prog_counter - 2, code
        )
    }

    fn execute(&mut self, inst: Chip8Inst) {
        if DEBUG {
            println!("{:?}", inst);
        }

        match inst {
            Chip8Inst::MachineInst => (),
            Chip8Inst::ClearScreen => self.display.fill(false),
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
                let n = self.registers[x];
                let m = self.registers[y];
                if n > m {
                    self.registers[x] = 0;
                    self.registers[0xf] = 1;
                } else {
                    self.registers[x] = n - m;
                    self.registers[0xf] = 0;
                }
            }
            Chip8Inst::ArithSubReverse(x, y) => {
                let n = self.registers[y];
                let m = self.registers[x];
                if n > m {
                    self.registers[x] = 0;
                    self.registers[0xf] = 1;
                } else {
                    self.registers[x] = n - m;
                    self.registers[0xf] = 0;
                }
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
            _ => (),
        }

        if DEBUG {
            println!("{:?}", self.registers);
        }
    }

    pub fn load_program(&mut self, fname: &str) -> std::io::Result<()> {
        let mut f = File::open(fname)?;
        f.read(&mut self.memory[0x200..])?;
        Ok(())
    }

    pub fn run(&mut self) {
        // start timers
        let delay_clone = Arc::clone(&self.delay_timer);
        let sound_clone = Arc::clone(&self.sound_timer);

        let run_timers = Arc::new(AtomicBool::new(true));
        let run_timers_clone = Arc::clone(&run_timers);

        let timer_thread = thread::spawn(move || {
            let freq = Duration::from_nanos(16667);

            while run_timers_clone.load(Ordering::Acquire) {
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
        for _ in 1..100 {
            let code = self.fetch();
            match self.decode(code) {
                Ok(inst) => self.execute(inst),
                Err(e) => panic!("{}", e),
            }
        }

        run_timers.store(false, Ordering::Release);
        match timer_thread.join() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }
}

fn make_usize(x: u8, y: u8, z: u8) -> usize {
    ((x as usize) << 8) | ((y as usize) << 4) | z as usize
}
