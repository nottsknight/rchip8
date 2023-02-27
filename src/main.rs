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

pub mod compiler;
mod machine;

use clap::Parser;
use machine::{Chip8Machine, Chip8Mode, DELAY_1MHZ, DELAY_60HZ, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use sdl2::{
    event::Event,
    keyboard::Scancode,
    pixels::Color,
    rect::Rect,
};
use simple_logger::SimpleLogger;
use std::sync::{
    atomic::{AtomicBool, AtomicU8, Ordering},
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
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Chip8Args::parse();

    let mode = if args.original {
        Chip8Mode::Original
    } else {
        Chip8Mode::Modern
    };

    start_vm(mode, &args.rom_file);
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

    // Create VM and load rom
    let delay_timer = Arc::new(AtomicU8::new(0));
    let sound_timer = Arc::new(AtomicU8::new(0));
    let display = Arc::new(Mutex::new([false; DISPLAY_WIDTH * DISPLAY_HEIGHT]));
    let current_key = Arc::new((Mutex::new(0xff), Condvar::new()));
    let redraw = Arc::new(AtomicBool::new(false));

    let mut vm = Chip8Machine::new(
        mode,
        delay_timer.clone(),
        sound_timer.clone(),
        display.clone(),
        current_key.clone(),
        redraw.clone(),
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
            loop {
                let code = vm.fetch();
                let inst = vm.decode(code).unwrap();
                vm.execute(inst);
                thread::sleep(freq);
            }
        })
        .unwrap();

    // Main loop
    let mut events = sdl_context.event_pump().unwrap();
    let freq = Duration::from_nanos(DELAY_60HZ);
    'running: loop {
        // Decrement timers
        let delay = delay_timer.load(Ordering::Acquire);
        if delay > 0 {
            delay_timer.store(delay - 1, Ordering::Release);
        }

        let sound = sound_timer.load(Ordering::Acquire);
        if sound > 0 {
            sound_timer.store(sound - 1, Ordering::Release);
        }

        // Check for redraw
        if redraw.load(Ordering::Relaxed) {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            canvas.set_draw_color(Color::WHITE);
            let pixels = display.lock().unwrap();
            for x in 0..DISPLAY_WIDTH {
                for y in 0..DISPLAY_HEIGHT {
                    if pixels[y * DISPLAY_WIDTH + x] {
                        let r = Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10);
                        canvas.fill_rect(r).unwrap();
                    }
                }
            }

            redraw.store(false, Ordering::Relaxed);
            canvas.present();
        }

        // Respond to input events
        for e in events.poll_iter() {
            match e {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    scancode: Some(sc), ..
                } => {
                    let (lock, cvar) = &*current_key;
                    let mut key = lock.lock().unwrap();
                    match sc {
                        Scancode::Num1 => *key = 0x1,
                        Scancode::Num2 => *key = 0x2,
                        Scancode::Num3 => *key = 0x3,
                        Scancode::Num4 => *key = 0xc,
                        Scancode::Q => *key = 0x4,
                        Scancode::W => *key = 0x5,
                        Scancode::E => *key = 0x6,
                        Scancode::R => *key = 0xd,
                        Scancode::A => *key = 0x7,
                        Scancode::S => *key = 0x8,
                        Scancode::D => *key = 0x9,
                        Scancode::F => *key = 0xe,
                        Scancode::Z => *key = 0xa,
                        Scancode::X => *key = 0x0,
                        Scancode::C => *key = 0xb,
                        Scancode::V => *key = 0xf,
                        _ => *key = 0xff,
                    }
                    cvar.notify_one();
                    drop(key);
                }
                _ => (),
            }
        }

        thread::sleep(freq);
    }
}
