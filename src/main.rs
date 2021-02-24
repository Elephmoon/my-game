use std::time::Duration;

use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::Sdl;

use crate::direction::Direction;
use crate::player::Player;
use crate::window::{handle_event, init_window};

mod player;
mod direction;
mod window;

const SLEEP_TIME_NANOS: u32 = 17_000_000;
const SLEEP_TIME_S: u64 = 0;
const INIT_COLOR: Color = Color::RGB(81, 131, 232);
const BARD_PATH: &str = "assets/bard.png";

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
    let player = Player::new();

    drawing(&sdl_ctx, &mut canvas, player);

    Ok(())
}

fn drawing(sdl_ctx: &Sdl, canvas: &mut WindowCanvas, mut player: Player) {
    let texture_builder = canvas.texture_creator();
    let texture = texture_builder.load_texture(BARD_PATH)
        .expect("couldn't load texture");
    let mut even_queue = sdl_ctx.event_pump().expect("couldn't init event pump");
    loop {
        for event in even_queue.poll_iter() {
            let exit = handle_event(event, &mut player);
            if exit {
                return;
            }
        }
        Player::update_player(&mut player);
        let _ = render(canvas, INIT_COLOR, &texture, &player);

        ::std::thread::sleep(Duration::new(SLEEP_TIME_S, SLEEP_TIME_NANOS))
    }
}

fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture, player: &Player) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();
    let (width, height) = canvas.output_size()?;
    let (frame_width, frame_height) = player.sprite.size();
    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y() + frame_height as i32 * Direction::direction_sprite_row(player.direction),
        frame_width,
        frame_height,
    );
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
    canvas.copy(texture, current_frame, screen_rect)?;
    canvas.present();

    Ok(())
}

