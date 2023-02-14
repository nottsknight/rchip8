use crate::machine::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use termion::cursor;

pub struct Chip8Display {
    pixels: [bool; DISPLAY_HEIGHT * DISPLAY_WIDTH],
}

impl Chip8Display {
    pub fn init() -> Chip8Display {
        Chip8Display {
            pixels: [false; DISPLAY_HEIGHT * DISPLAY_WIDTH],
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.pixels.fill(false);
    }

    pub fn update_pixel(&mut self, x: usize, y: usize, px: bool) -> bool {
        let px0 = self.get_pixel(x, y);
        self.set_pixel(x, y, px0 ^ px);
        if px && px == px0 {
            true
        } else {
            false
        }
    }

    #[inline]
    fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y * DISPLAY_WIDTH + x]
    }

    #[inline]
    fn set_pixel(&mut self, x: usize, y: usize, px: bool) {
        self.pixels[y * DISPLAY_WIDTH + x] = px;
    }

    pub fn draw(&self) {
        let mut display_str = String::from("");
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let px = self.get_pixel(x, y);
                display_str.push(if px { '\u{2588}' } else { ' ' })
            }
            display_str.push('\n');
        }
        print!("{}{}", cursor::Goto(1, 1), display_str);
    }
}
