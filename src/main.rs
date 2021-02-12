use std::ops::Range;
use std::time::Duration;

use rand::Rng;
use sdl2::{Error, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;
const WINDOW_NAME: &str = "my-game";
const SLEEP_TIME_NANOS: u32 = 20000;
const SLEEP_TIME_S: u64 = 2;
const INIT_COLOR: Color = Color::RGB(0, 255, 255);
const COLORS_RANGE: Range<u8> = 1..255;

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init().expect("couldn't init sdl");
    let window = match init_window(&sdl_ctx) {
        Ok(window) => window,
        _ => panic!("couldn't init window")
    };

    let mut canvas = window.into_canvas().build()
        .expect("couldn't init canvas");

    drawing(&sdl_ctx, &mut canvas);

    Ok(())
}

fn init_window(sdl_ctx: &Sdl) -> Result<Window, Error> {
    let video = sdl_ctx.video().expect("couldn't init video");
    let window = video.window(WINDOW_NAME, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("couldn't init window");
    Ok(window)
}

fn drawing(sdl_ctx: &Sdl, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(INIT_COLOR);
    canvas.clear();
    canvas.present();
    let mut even_queue = sdl_ctx.event_pump().expect("couldn't init event pump");
    loop {
        let red= rand::thread_rng().gen_range(COLORS_RANGE);
        let green = rand::thread_rng().gen_range(COLORS_RANGE);
        let blue = rand::thread_rng().gen_range(COLORS_RANGE);
        canvas.set_draw_color(Color::RGB(red, green, blue));
        canvas.clear();
        for event in even_queue.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    return;
                }
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(SLEEP_TIME_S, SLEEP_TIME_NANOS))
    }
}
