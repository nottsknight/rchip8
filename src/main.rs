use std::fs::File;
use std::io::Read;
use std::process::exit;

const MEM_SIZE: u16 = 4096;

struct Chip8Interpreter {
    memory: [u8; MEM_SIZE as usize],
    display: [[bool; 32]; 64],
    program_counter: u16,
    index_register: u16,
    stack_pointer: u16,
    registers: [u8; 16],
}

impl Chip8Interpreter {
    fn new() -> Chip8Interpreter {
        Chip8Interpreter {
            memory: [0u8; MEM_SIZE as usize],
            display: [[false; 32]; 64],
            program_counter: 0x200,
            index_register: 0,
            stack_pointer: MEM_SIZE - 3,
            registers: [0; 16],
        }
    }

    fn load_program(&mut self, filename: &str) -> std::io::Result<()> {
        let mut f = File::open(filename)?;
        f.read(&mut self.memory[0x200..])?;
        Ok(())
    }
}

fn main() {
    let mut interpreter = Chip8Interpreter::new();
}
