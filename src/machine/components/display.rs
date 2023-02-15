use crate::machine::{DISPLAY_HEIGHT, DISPLAY_WIDTH, FREQ_60HZ};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Chip8Display {
    pixels: Arc<Mutex<[bool; DISPLAY_HEIGHT * DISPLAY_WIDTH]>>,
    redraw: Arc<AtomicBool>,
}

impl Chip8Display {
    pub fn init() -> Chip8Display {
        Chip8Display {
            pixels: Arc::new(Mutex::new([false; DISPLAY_HEIGHT * DISPLAY_WIDTH])),
            redraw: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self) {
        let pixel_clone = Arc::clone(&self.pixels);
        let redraw_clone = Arc::clone(&self.redraw);

        thread::spawn(move || {
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

            let BLACK = Color::RGB(0, 0, 0);
            let WHITE = Color::RGB(255, 255, 255);

            let mut canvas = window.into_canvas().build().unwrap();
            canvas.set_draw_color(BLACK);
            canvas.clear();

            let mut events = sdl_context.event_pump().unwrap();
            let freq = Duration::from_nanos(FREQ_60HZ);

            'running: loop {
                if redraw_clone.load(Ordering::Acquire) {
                    canvas.set_draw_color(BLACK);
                    canvas.clear();

                    canvas.set_draw_color(WHITE);
                    let pixels = pixel_clone.lock().unwrap();
                    for y in 0..DISPLAY_HEIGHT {
                        for x in 0..DISPLAY_WIDTH {
                            let px = pixels[y * DISPLAY_WIDTH + x];
                            if px {
                                let px_rect = Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10);
                                canvas.fill_rect(px_rect).unwrap();
                            }
                        }
                    }
                    drop(pixels);

                    canvas.present();
                    redraw_clone.store(false, Ordering::Release);
                }

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
                thread::sleep(freq);
            }
        });
    }

    pub fn redraw(&mut self) {
        self.redraw.store(true, Ordering::Release);
    }

    #[inline]
    pub fn clear(&mut self) {
        let mut pixels_val = self.pixels.lock().unwrap();
        pixels_val.fill(false);
    }

    pub fn update_pixel(&mut self, x: usize, y: usize, px: bool) -> bool {
        let mut pixels_val = self.pixels.lock().unwrap();
        let px0 = pixels_val[y * DISPLAY_WIDTH + x];
        pixels_val[y * DISPLAY_WIDTH + x] = px0 ^ px;
        drop(pixels_val);

        if px && px == px0 {
            true
        } else {
            false
        }
    }
}
