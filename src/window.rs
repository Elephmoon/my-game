use sdl2::{Error, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;

use crate::direction::Direction;
use crate::player::{DEFAULT_PLAYER_SPEED, Player};

pub const WINDOW_WIDTH: u32 = 1000;
pub const WINDOW_HEIGHT: u32 = 600;
const WINDOW_NAME: &str = "my-game";

pub fn init_window(sdl_ctx: &Sdl) -> Result<Window, Error> {
    let video = sdl_ctx.video().expect("couldn't init video");
    let window = video.window(WINDOW_NAME, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("couldn't init window");
    Ok(window)
}

pub fn handle_event(event: Event, player: &mut Player) -> bool {
    match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            return true;
        }
        Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
            player.speed = DEFAULT_PLAYER_SPEED;
            player.direction = Direction::Left;
        }
        Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
            player.speed = DEFAULT_PLAYER_SPEED;
            player.direction = Direction::Right;
        }
        Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
            player.speed = DEFAULT_PLAYER_SPEED;
            player.direction = Direction::Up;
        }
        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
            player.speed = DEFAULT_PLAYER_SPEED;
            player.direction = Direction::Down;
        }
        Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
        Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
        Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
        Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
            player.speed = 0;
        }
        _ => {}
    }
    return false;
}