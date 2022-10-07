use std::fs::File;
use std::io::Read;

use crate::instruction::Instruction;

const MEM_SIZE: u16 = 4096;

pub struct Chip8Interpreter {
    memory: [u8; MEM_SIZE as usize],
    display: [[bool; 32]; 64],
    program_counter: u16,
    index_register: u16,
    stack_pointer: u16,
    registers: [u8; 16],
}

impl Chip8Interpreter {
    pub fn new() -> Chip8Interpreter {
        Chip8Interpreter {
            memory: [0u8; MEM_SIZE as usize],
            display: [[false; 32]; 64],
            program_counter: 0x200,
            index_register: 0,
            stack_pointer: MEM_SIZE - 3,
            registers: [0; 16],
        }
    }

    pub fn load_program(&mut self, filename: &str) -> std::io::Result<()> {
        let mut f = File::open(filename)?;
        f.read(&mut self.memory[0x200..])?;
        Ok(())
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            let code = self.fetch();
            let inst = self.decode(code);
            if !self.execute(inst) {
                break;
            }
        }
        Ok(())
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.program_counter as usize] as u16;
        let lo = self.memory[(self.program_counter + 1) as usize] as u16;
        self.program_counter += 2;
        return (hi << 8) | lo;
    }

    fn decode(&self, code: u16) -> Instruction {
        Instruction::from(code)
    }

    fn execute(&mut self, Instruction(a, b, c, d): Instruction) -> bool {
        match a {
            0x6 => {
                self.registers[b as usize] = (c << 4) | d;
                return true;
            }
            0x8 => match d {
                0x0 => {
                    self.registers[b as usize] = self.registers[c as usize];
                    return true;
                }
                0x1 => {
                    self.registers[b as usize] |= self.registers[c as usize];
                    return true;
                }
                0x2 => {
                    self.registers[b as usize] &= self.registers[c as usize];
                    return true;
                }
                0x3 => {
                    self.registers[b as usize] ^= self.registers[c as usize];
                    return true;
                }
                _ => return false,
            },
            _ => return false,
        }
    }
}
