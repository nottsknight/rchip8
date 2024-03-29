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

extern crate lalrpop_util;

use clap::Parser;
use rchip8::machine::{
    disassemble::disassemble, Chip8Machine, Chip8Mode, DELAY_1MHZ, DELAY_60HZ, DISPLAY_HEIGHT,
    DISPLAY_WIDTH,
};
// use rodio::{source::SineWave, OutputStream, Sink, Source};
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, keyboard::Scancode, pixels::Color, rect::Rect};
use simple_logger::SimpleLogger;
use std::fs::File;
use std::io::Read;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex,
};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Chip8Args {
    /// Path to the ROM file to run
    rom_file: String,
    /// Run in original mode
    #[arg(long, short)]
    original: bool,
    /// Output addresses when disassembling (starting at 0x200)
    #[arg(short, long)]
    addresses: bool,
    /// Disassemble the ROM instead of executing it
    #[arg(long, short)]
    disassemble: bool,
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Chip8Args::parse();

    if args.disassemble {
        run_disassemble(&args.rom_file, args.addresses);
    } else {
        let mode = if args.original {
            Chip8Mode::Original
        } else {
            Chip8Mode::Modern
        };

        start_vm(mode, &args.rom_file);
    }
}

fn run_disassemble(rom_file: &str, addresses: bool) {
    if let Ok(mut f) = File::open(rom_file) {
        let mut buf = [0u8; 2];
        let mut pc = 0x200;
        while let Ok(2) = f.read(&mut buf) {
            let code = (buf[0] as u16) << 8 | (buf[1] as u16);
            match Chip8Machine::decode(code) {
                Ok(inst) => {
                    if addresses {
                        println!("{}", disassemble(Some(pc), inst));
                    } else {
                        println!("{}", disassemble(None, inst));
                    }
                }
                Err(_) => {
                    if addresses {
                        println!("{:#06x} .data   {:02X} {:02X}", pc, buf[0], buf[1]);
                    } else {
                        println!(".data   {:02X} {:02X}", buf[0], buf[1]);
                    }
                }
            }
            pc += 2;
        }
    }
}

fn start_vm(mode: Chip8Mode, rom_file: &str) {
    // Initialise and display window
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();

    let window = video_subsys
        .window(
            "rCHIP-8",
            (DISPLAY_WIDTH * 10) as u32,
            (DISPLAY_HEIGHT * 10) as u32,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    // Create VM and load ROM
    let delay_timer = Arc::new(Mutex::new(0));
    let sound_timer = Arc::new(Mutex::new(0));
    let display = Arc::new(Mutex::new([false; DISPLAY_WIDTH * DISPLAY_HEIGHT]));
    const NEW_BOOL: AtomicBool = AtomicBool::new(false);
    let redraw = Arc::new([NEW_BOOL; DISPLAY_WIDTH * DISPLAY_HEIGHT]);
    let key_state = Arc::new([NEW_BOOL; 16]);
    let current_key = Arc::new((Mutex::new(None), Condvar::new()));

    let mut vm = Chip8Machine::new(
        mode,
        delay_timer.clone(),
        sound_timer.clone(),
        display.clone(),
        redraw.clone(),
        key_state.clone(),
        current_key.clone(),
    );

    match vm.load_rom(rom_file) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    // Launch VM thread
    thread::Builder::new()
        .name("vm".to_string())
        .spawn(move || {
            let freq = Duration::from_nanos(DELAY_1MHZ);
            vm.run_program(freq);
        })
        .unwrap();

    // Main loop
    // let (_, audio_stream) = OutputStream::try_default().unwrap();
    // let audio_sink = Sink::try_new(&audio_stream).unwrap();
    // let source = SineWave::new(261.63).take_duration(Duration::from_secs(600));
    // audio_sink.append(source);

    let mut events = sdl_context.event_pump().unwrap();
    let freq = Duration::from_nanos(DELAY_60HZ);
    'running: loop {
        // Decrement timers
        if let Ok(mut delay) = delay_timer.lock() {
            if *delay > 0 {
                *delay -= 1;
            }
        }

        if let Ok(mut sound) = sound_timer.lock() {
            if *sound > 0 {
                // audio_sink.play();
                *sound -= 1;
            } else {
                // audio_sink.pause();
            }
        }

        // Check for redraw
        for x in 0..DISPLAY_WIDTH {
            for y in 0..DISPLAY_HEIGHT {
                let idx = y * DISPLAY_WIDTH + x;
                if redraw[idx].load(Ordering::Acquire) {
                    redraw[idx].store(false, Ordering::Release);
                    let pixels = display.lock().unwrap();
                    let r = Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10);
                    let color = if pixels[idx] {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    };
                    canvas.set_draw_color(color);
                    canvas.fill_rect(r).unwrap();
                }
            }
        }
        canvas.present();

        // Respond to input events
        for e in events.poll_iter() {
            match e {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    scancode: Some(sc), ..
                } => {
                    if let Some(idx) = scancode_to_index(sc) {
                        key_state[idx].store(true, Ordering::Release);
                        let (lock, cvar) = &*current_key;
                        if let Ok(mut curr_key) = lock.lock() {
                            *curr_key = Some(idx as u8);
                            cvar.notify_all();
                        }
                    }
                }
                Event::KeyUp {
                    scancode: Some(sc), ..
                } => {
                    if let Some(idx) = scancode_to_index(sc) {
                        key_state[idx].store(false, Ordering::Release);
                    }
                    let (lock, _cvar) = &*current_key;
                    if let Ok(mut curr_key) = lock.lock() {
                        *curr_key = None;
                    }
                }
                _ => (),
            }
        }

        thread::sleep(freq);
    }
}

#[inline]
fn scancode_to_index(sc: Scancode) -> Option<usize> {
    match sc {
        Scancode::Num1 => Some(0x1),
        Scancode::Num2 => Some(0x2),
        Scancode::Num3 => Some(0x3),
        Scancode::Num4 => Some(0xc),
        Scancode::Q => Some(0x4),
        Scancode::W => Some(0x5),
        Scancode::E => Some(0x6),
        Scancode::R => Some(0xd),
        Scancode::A => Some(0x7),
        Scancode::S => Some(0x8),
        Scancode::D => Some(0x9),
        Scancode::F => Some(0xe),
        Scancode::Z => Some(0xa),
        Scancode::X => Some(0x0),
        Scancode::C => Some(0xb),
        Scancode::V => Some(0xf),
        _ => None,
    }
}
