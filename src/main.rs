use std::time::Duration;

use sdl2::{Error, Sdl};
use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::video::Window;

use crate::player::Player;

mod player;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_NAME: &str = "my-game";
const SLEEP_TIME_NANOS: u32 = 17_000_000;
const SLEEP_TIME_S: u64 = 2;
const INIT_COLOR: Color = Color::RGB(81, 131, 232);
const BARD_PATH: &str = "assets/bard.png";
const INIT_HERO_X: i32 = 0;
const INIT_HERO_Y: i32 = 0;
const HERO_SPRITE_WIDTH: u32 = 26;
const HERO_SPRITE_HEIGHT: u32 = 36;

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init().expect("couldn't init sdl");
    let window = match init_window(&sdl_ctx) {
        Ok(window) => window,
        _ => panic!("couldn't init window")
    };
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)
        .expect("couldn't init image context");

    let mut canvas = window.into_canvas().build()
        .expect("couldn't init canvas");
    let player = Player::new(Point::new(INIT_HERO_X, INIT_HERO_Y),
                             Rect::new(INIT_HERO_X, INIT_HERO_Y, HERO_SPRITE_WIDTH, HERO_SPRITE_HEIGHT));

    drawing(&sdl_ctx, &mut canvas, player);

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

fn drawing(sdl_ctx: &Sdl, canvas: &mut WindowCanvas, player: Player) {
    let texture_builder = canvas.texture_creator();
    let texture = texture_builder.load_texture(BARD_PATH)
        .expect("couldn't load texture");
    let mut even_queue = sdl_ctx.event_pump().expect("couldn't init event pump");
    loop {
        for event in even_queue.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return;
                }
                _ => {}
            }
        }
        let _ = render(canvas, INIT_COLOR, &texture, &player);

        ::std::thread::sleep(Duration::new(SLEEP_TIME_S, SLEEP_TIME_NANOS))
    }
}

fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture, player: &Player) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();
    let (width, height) = canvas.output_size()?;
    let position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let rect = Rect::from_center(position, player.sprite.width(), player.sprite.height());
    canvas.copy(texture, player.sprite, rect)?;
    canvas.present();

    Ok(())
}

