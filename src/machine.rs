use crate::add_carry::AddCarry;
use crate::hilo::HiLo;
use crate::insts::Chip8Inst;
use std::sync::atomic::AtomicU8;
use std::sync::Arc;

pub struct VirtualMachine {
    memory: [u8; 4096],
    display: [bool; 64 * 32],
    prog_counter: usize,
    index_reg: usize,
    stack: Vec<u8>,
    delay_timer: Arc<AtomicU8>,
    sound_timer: Arc<AtomicU8>,
    registers: [u8; 16],
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        let fonts: [u8; 80] = [
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

        let mut memory_array = [0; 4096];
        for i in 0x050..0x09f {
            memory_array[i] = fonts[i - 0x0f0];
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

    fn decode(&mut self, code: u16) -> Chip8Inst {
        let (a, b) = code.hi().split();
        let (c, d) = code.lo().split();
        match a {
            0x0 => match (b, c, d) {
                (0x0, 0xe, 0x0) => Chip8Inst::ClearScreen,
                _ => panic!("Invalid instruction"),
            },
            0x1 => {
                let n = make_usize(b, c, d);
                Chip8Inst::Jump(n)
            }
            0x6 => {
                let n = (c << 4) | d;
                Chip8Inst::SetRegister(b as usize, n)
            }
            0x7 => {
                let n = (c << 4) | d;
                Chip8Inst::AddRegister(b as usize, n)
            }
            0xa => {
                let n = make_usize(b, c, d);
                Chip8Inst::SetIndex(n)
            }
            _ => panic!("Invalid instruction"),
        }
    }

    fn execute(&mut self, inst: Chip8Inst) {
        match inst {
            Chip8Inst::MachineInst => (),
            Chip8Inst::ClearScreen => self.display.fill(false),
            Chip8Inst::Jump(n) => self.prog_counter = n,
            Chip8Inst::SetIndex(n) => self.index_reg = n,
            Chip8Inst::SetRegister(x, n) => self.registers[x] = n,
            Chip8Inst::AddRegister(x, n) => {
                let m = self.registers[x];
                self.registers[x] = u8::add_no_carry(m, n);
            }
            _ => (),
        }
    }
}

fn make_usize(x: u8, y: u8, z: u8) -> usize {
    ((x as usize) << 8) | ((y as usize) << 4) | z as usize
}
