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

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use super::FREQ_60HZ;

pub struct Chip8Timers {
    delay_timer: Arc<Mutex<u8>>,
    sound_timer: Arc<Mutex<u8>>,
}

impl Chip8Timers {
    pub fn init() -> Chip8Timers {
        Chip8Timers {
            delay_timer: Arc::new(Mutex::new(0)),
            sound_timer: Arc::new(Mutex::new(0)),
        }
    }

    pub fn set_delay(&mut self, value: u8) {
        let mut delay_val = self.delay_timer.lock().unwrap();
        *delay_val = value;
    }

    pub fn read_delay(&self) -> u8 {
        let delay_val = self.delay_timer.lock().unwrap();
        *delay_val
    }

    pub fn set_sound(&mut self, value: u8) {
        let mut sound_val = self.sound_timer.lock().unwrap();
        *sound_val = value;
    }

    pub fn start(&self) {
        let delay_clone = Arc::clone(&self.delay_timer);
        let sound_clone = Arc::clone(&self.sound_timer);
        thread::spawn(move || {
            let freq = Duration::from_nanos(FREQ_60HZ);
            loop {
                let mut delay_val = delay_clone.lock().unwrap();
                if *delay_val > 0 {
                    *delay_val -= 1;
                }
                drop(delay_val);

                let mut sound_val = sound_clone.lock().unwrap();
                if *sound_val > 0 {
                    *sound_val -= 1;
                }
                drop(sound_val);

                thread::sleep(freq);
            }
        });
    }
}
