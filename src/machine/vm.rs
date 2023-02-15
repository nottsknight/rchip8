use crate::machine::components::{display::Chip8Display, timers::Chip8Timers};
use crate::machine::{FONT, FONT_BASE, FREQ_1MHZ};
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Eq)]
pub enum Chip8Mode {
    Original,
    Modern,
}

pub struct Chip8Machine {
    pub (in crate::machine) mode: Chip8Mode,
    pub (in crate::machine) memory: [u8; 4096],
    pub (in crate::machine) display: Chip8Display,
    pub (in crate::machine) prog_counter: usize,
    pub (in crate::machine) index_reg: usize,
    pub (in crate::machine) stack: Vec<usize>,
    pub (in crate::machine) timers: Chip8Timers,
    pub (in crate::machine) registers: [u8; 16],
}

impl Chip8Machine {
    pub fn new(mode: Chip8Mode) -> Chip8Machine {
        let mut memory = [0; 4096];
        memory[FONT_BASE..FONT_BASE + 80].copy_from_slice(&FONT[..]);

        Chip8Machine {
            mode,
            memory,
            display: Chip8Display::init(),
            prog_counter: 0x200,
            index_reg: 0,
            stack: Vec::new(),
            timers: Chip8Timers::init(),
            registers: [0; 16],
        }
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
        // start timers
        self.timers.start();

        // start display
        self.display.start();

        // fetch-decode-execute loop
        let cpu_freq = Duration::from_nanos(FREQ_1MHZ);
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
