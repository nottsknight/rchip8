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

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::{
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Arc, RwLock,
    },
    time::Duration,
};

const DELAY_60HZ: u64 = 1_000_000_000 / 60;

const DELAY_1MHZ: u64 = 1_000_000_000 / 1000;

const DISPLAY_WIDTH: usize = 64;

const DISPLAY_HEIGHT: usize = 32;

const FONT_BASE: usize = 0x050;

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

#[derive(PartialEq, Eq)]
pub enum Chip8Mode {
    Original,
    Modern,
}

pub struct Chip8Machine {
    mode: Chip8Mode,
    // memory
    memory: [u8; 4096],
    stack: Vec<usize>,
    prog_counter: usize,
    // registers
    registers: [u8; 16],
    index_reg: usize,
    // timers
    delay_timer: Arc<AtomicU8>,
    sound_timer: Arc<AtomicU8>,
    // display
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    current_key: Arc<AtomicU8>,
    redraw: Arc<AtomicBool>,
}

impl Chip8Machine {
    fn new(mode: Chip8Mode) -> Chip8Machine {
        let mut vm = Chip8Machine {
            mode,
            memory: [0; 4096],
            stack: Vec::new(),
            prog_counter: 0x200,
            registers: [0; 16],
            index_reg: 0,
            delay_timer: Arc::new(AtomicU8::new(0)),
            sound_timer: Arc::new(AtomicU8::new(0)),
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            current_key: Arc::new(AtomicU8::new(0xff)),
            redraw: Arc::new(AtomicBool::new(false)),
        };

        vm.memory[FONT_BASE..FONT_BASE + 80].copy_from_slice(&FONT[..]);
        vm
    }

    pub fn start_vm(mode: Chip8Mode, rom_file: &str) -> std::io::Result<()> {
        let mut vm = Arc::new(RwLock::new(Chip8Machine::new(mode)));

        let mut rom = File::open(rom_file)?;
        let mut vm_guard = vm.write().unwrap();
        rom.read(&mut vm_guard.memory[0x200..])?;
        drop(vm_guard);

        // set up display
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "rCHIP-8",
                (DISPLAY_WIDTH * 10) as u32,
                (DISPLAY_HEIGHT * 10) as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let black_col = Color::RGB(0, 0, 0);
        let white_col = Color::RGB(255, 255, 255);

        // launch vm thread
        let vm_local = Arc::clone(&mut vm);

        let vm_running = Arc::new(AtomicBool::new(true));
        let vm_running_clone = Arc::clone(&vm_running);

        let vm_thread = thread::spawn(move || {
            let vm_freq = Duration::from_nanos(DELAY_1MHZ);
            let timer_freq = Duration::from_nanos(DELAY_60HZ);

            // start timer thread
            let vm_guard = vm_local.read().unwrap();
            let delay_timer_clone = Arc::clone(&vm_guard.delay_timer);
            let sound_timer_clone = Arc::clone(&vm_guard.sound_timer);
            drop(vm_guard);

            let timer_running = Arc::new(AtomicBool::new(true));
            let timer_running_clone = Arc::clone(&timer_running);

            let timer_thread = thread::spawn(move || {
                while timer_running_clone.load(Ordering::Acquire) {
                    let delay = delay_timer_clone.load(Ordering::Acquire);
                    if delay > 0 {
                        delay_timer_clone.store(delay - 1, Ordering::Release);
                    }

                    let sound = sound_timer_clone.load(Ordering::Acquire);
                    if sound > 0 {
                        sound_timer_clone.store(sound - 1, Ordering::Release);
                    }

                    thread::sleep(timer_freq);
                }
            });

            while vm_running_clone.load(Ordering::Acquire) {
                let mut vm_guard = vm_local.write().unwrap();
                let code = vm_guard.fetch();
                drop(vm_guard);

                let vm_guard = vm_local.read().unwrap();
                let inst = vm_guard.decode(code).unwrap();
                drop(vm_guard);

                let mut vm_guard = vm_local.write().unwrap();
                vm_guard.execute(inst);
                drop(vm_guard);

                thread::sleep(vm_freq);
            }

            timer_running.store(false, Ordering::Release);
            timer_thread.join().unwrap();
        });

        // run window
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(black_col);
        canvas.clear();

        let mut events = sdl_context.event_pump().unwrap();
        let freq = Duration::from_nanos(DELAY_60HZ);

        'main: loop {
            let vm_guard = vm.read().unwrap();
            if vm_guard.redraw.load(Ordering::Acquire) {
                canvas.set_draw_color(black_col);
                canvas.clear();

                vm_guard.redraw.store(false, Ordering::Release);
                canvas.set_draw_color(white_col);
                for y in 0..DISPLAY_HEIGHT {
                    for x in 0..DISPLAY_WIDTH {
                        if vm_guard.display[y * DISPLAY_WIDTH + x] {
                            let px_rect = Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10);
                            canvas.fill_rect(px_rect).unwrap();
                        }
                    }
                }
            }
            drop(vm_guard);
            canvas.present();

            for e in events.poll_iter() {
                match e {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main,
                    Event::KeyDown { scancode: sc, .. } => {
                        let vm_guard = vm.write().unwrap();
                        match sc {
                            Some(Scancode::Num1) => {
                                vm_guard.current_key.store(0x1, Ordering::Release)
                            }
                            Some(Scancode::Num2) => {
                                vm_guard.current_key.store(0x2, Ordering::Release)
                            }
                            Some(Scancode::Num3) => {
                                vm_guard.current_key.store(0x3, Ordering::Release)
                            }
                            Some(Scancode::Num4) => {
                                vm_guard.current_key.store(0xc, Ordering::Release)
                            }
                            Some(Scancode::Q) => vm_guard.current_key.store(0x4, Ordering::Release),
                            Some(Scancode::W) => vm_guard.current_key.store(0x5, Ordering::Release),
                            Some(Scancode::E) => vm_guard.current_key.store(0x6, Ordering::Release),
                            Some(Scancode::R) => vm_guard.current_key.store(0xd, Ordering::Release),
                            Some(Scancode::A) => vm_guard.current_key.store(0x7, Ordering::Release),
                            Some(Scancode::S) => vm_guard.current_key.store(0x8, Ordering::Release),
                            Some(Scancode::D) => vm_guard.current_key.store(0x9, Ordering::Release),
                            Some(Scancode::F) => vm_guard.current_key.store(0xe, Ordering::Release),
                            Some(Scancode::Z) => vm_guard.current_key.store(0xa, Ordering::Release),
                            Some(Scancode::X) => vm_guard.current_key.store(0x0, Ordering::Release),
                            Some(Scancode::C) => vm_guard.current_key.store(0xb, Ordering::Release),
                            Some(Scancode::V) => vm_guard.current_key.store(0xf, Ordering::Release),
                            _ => vm_guard.current_key.store(0xff, Ordering::Release),
                        }
                        drop(vm_guard);
                    }
                    _ => (),
                }
            }

            thread::sleep(freq);
        }

        // shutdown other threads
        vm_running.store(false, Ordering::Release);
        vm_thread.join().unwrap();

        Ok(())
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.prog_counter];
        let lo = self.memory[self.prog_counter + 1];
        self.prog_counter += 2;
        (hi as u16) << 8 | lo as u16
    }

    fn read_delay_timer(&self) -> u8 {
        self.delay_timer.load(Ordering::Acquire)
    }

    fn write_delay_timer(&mut self, value: u8) {
        self.delay_timer.store(value, Ordering::Release)
    }

    fn write_sound_timer(&mut self, value: u8) {
        self.sound_timer.store(value, Ordering::Release)
    }
}

mod decode;
mod execute;
mod insts;
mod utils;
