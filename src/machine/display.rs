use super::{Chip8Machine, DISPLAY_COLS, DISPLAY_ROWS};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

impl Chip8Machine {
    pub(super) fn run_display(&self) {
        let display_clone = Arc::clone(&self.display);
        let display_freq = Duration::from_nanos(16_666_667);

        thread::spawn(move || {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem
                .window("rCHIP-8", 640, 320)
                .position_centered()
                .build()
                .unwrap();

            let mut canvas = window.into_canvas().build().unwrap();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            let mut events = sdl_context.event_pump().unwrap();

            'running: loop {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
                canvas.set_draw_color(Color::RGB(255, 255, 255));

                let display_data = display_clone.lock().unwrap();
                for x in 0..DISPLAY_COLS {
                    for y in 0..DISPLAY_ROWS {
                        if display_data[y][x] {
                            let r = Rect::new(x as i32, y as i32, 10, 10);
                            canvas.fill_rect(r).unwrap();
                        }
                    }
                }
                drop(display_data);

                for e in events.poll_iter() {
                    match e {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'running,
                        _ => (),
                    }
                }

                thread::sleep(display_freq);
            }
        });
        println!("Display thread started");
    }
}
