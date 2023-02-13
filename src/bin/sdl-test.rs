use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("sdl_test", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    
    let r = Rect::new(25, 25, 10, 10);
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(r).unwrap();
    canvas.present();

    let mut events = sdl_context.event_pump().unwrap();
    let freq = Duration::from_nanos(1_666_667);
    'running: loop {
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
}
