use log::info;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::{
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Arc, Mutex,
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
    keypresses: Option<Receiver<u8>>,
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
            keypresses: None,
            redraw: Arc::new(AtomicBool::new(false)),
        };

        let _ = &vm.memory[FONT_BASE..FONT_BASE + 80].copy_from_slice(&FONT[..]);
        vm
    }

    pub fn start_vm(mode: Chip8Mode, rom_file: &str) -> std::io::Result<()> {
        info!("Main thread started");
        let mut vm = Arc::new(Mutex::new(Chip8Machine::new(mode)));

        {
            let mut vm_guard = vm.lock().unwrap();
            let mut rom = File::open(rom_file)?;
            rom.read(&mut vm_guard.memory[0x200..])?;
            drop(vm_guard);
        }

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

        let (key_send, key_receive) = channel::<u8>();
        let mut vm_guard = vm_local.lock().unwrap();
        vm_guard.keypresses = Some(key_receive);
        drop(vm_guard);

        let vm_thread = thread::Builder::new()
            .name(String::from("vm"))
            .spawn(move || {
                info!("VM thread started");
                let vm_freq = Duration::from_nanos(DELAY_1MHZ);
                let timer_freq = Duration::from_nanos(DELAY_60HZ);

                // start timer thread
                let vm_guard = vm_local.lock().unwrap();
                let delay_timer_clone = Arc::clone(&vm_guard.delay_timer);
                let sound_timer_clone = Arc::clone(&vm_guard.sound_timer);
                drop(vm_guard);

                let timer_running = Arc::new(AtomicBool::new(true));
                let timer_running_clone = Arc::clone(&timer_running);

                let timer_thread =
                    thread::Builder::new()
                        .name(String::from("timer"))
                        .spawn(move || {
                            info!("Timer thread started");
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
                        }).unwrap();

                while vm_running_clone.load(Ordering::Acquire) {
                    let mut vm_guard = vm_local.lock().unwrap();
                    let code = vm_guard.fetch();
                    let inst = vm_guard.decode(code).unwrap();
                    vm_guard.execute(inst);
                    drop(vm_guard);

                    thread::sleep(vm_freq);
                }

                timer_running.store(false, Ordering::Release);
                timer_thread.join().unwrap();
            }).unwrap();

        // run window
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(black_col);
        canvas.clear();

        let mut events = sdl_context.event_pump().unwrap();
        let freq = Duration::from_nanos(DELAY_60HZ);

        'main: loop {
            canvas.set_draw_color(black_col);
            canvas.clear();

            let vm_guard = vm.lock().unwrap();
            if vm_guard.redraw.load(Ordering::Acquire) {
                vm_guard.redraw.store(false, Ordering::Release);
                canvas.set_draw_color(white_col);
                for y in 0..DISPLAY_HEIGHT {
                    for x in 0..DISPLAY_WIDTH {
                        let px_rect = Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10);
                        canvas.fill_rect(px_rect).unwrap();
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
                    Event::KeyDown { scancode: sc, .. } => match sc {
                        Some(Scancode::Num1) => key_send.send(0x1).unwrap(),
                        Some(Scancode::Num2) => key_send.send(0x2).unwrap(),
                        Some(Scancode::Num3) => key_send.send(0x3).unwrap(),
                        Some(Scancode::Num4) => key_send.send(0xc).unwrap(),
                        Some(Scancode::Q) => key_send.send(0x4).unwrap(),
                        Some(Scancode::W) => key_send.send(0x5).unwrap(),
                        Some(Scancode::E) => key_send.send(0x6).unwrap(),
                        Some(Scancode::R) => key_send.send(0xd).unwrap(),
                        Some(Scancode::A) => key_send.send(0x7).unwrap(),
                        Some(Scancode::D) => key_send.send(0x8).unwrap(),
                        Some(Scancode::F) => key_send.send(0x9).unwrap(),
                        Some(Scancode::Z) => key_send.send(0xa).unwrap(),
                        Some(Scancode::X) => key_send.send(0x0).unwrap(),
                        Some(Scancode::C) => key_send.send(0xb).unwrap(),
                        Some(Scancode::V) => key_send.send(0xf).unwrap(),
                        _ => (),
                    },
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
