use sdl2::rect::{Point, Rect};

use crate::direction::Direction;
use crate::window::{WINDOW_HEIGHT, WINDOW_WIDTH};

const INIT_HERO_SPEED: i32 = 0;
const INIT_HERO_X: i32 = 0;
const INIT_HERO_Y: i32 = 0;
const HERO_SPRITE_WIDTH: u32 = 26;
const HERO_SPRITE_HEIGHT: u32 = 36;
pub const DEFAULT_PLAYER_SPEED: i32 = 20;
const LEFT_BORDER: i32 = 0 - (WINDOW_WIDTH / 2 + 50) as i32;
const RIGHT_BORDER: i32 = (WINDOW_WIDTH / 2 + 50) as i32;
const UP_BORDER: i32 = 0 - (WINDOW_HEIGHT / 2 + 50) as i32;
const DOWN_BORDER: i32 = (WINDOW_HEIGHT / 2 + 50) as i32;
const INIT_CURRENT_FRAME: i32 = 0;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub direction: Direction,
    pub current_frame: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Point::new(INIT_HERO_X, INIT_HERO_Y),
            sprite: Rect::new(INIT_HERO_X, INIT_HERO_Y, HERO_SPRITE_WIDTH, HERO_SPRITE_HEIGHT),
            speed: INIT_HERO_SPEED,
            direction: Direction::Right,
            current_frame: INIT_CURRENT_FRAME,
        }
    }

    pub fn update_player(player: &mut Player) {
        use self::Direction::*;
        match player.direction {
            Left => {
                if player.position.x() <= LEFT_BORDER {
                    player.position = Point::new(RIGHT_BORDER, player.position.y());
                }
                player.position = player.position.offset(-player.speed, 0);
            }
            Right => {
                if player.position.x() >= RIGHT_BORDER {
                    player.position = Point::new(LEFT_BORDER, player.position.y());
                }
                player.position = player.position.offset(player.speed, 0);
            }
            Up => {
                if player.position.y() <= UP_BORDER {
                    player.position = Point::new(player.position.x(), DOWN_BORDER);
                }
                player.position = player.position.offset(0, -player.speed);
            }
            Down => {
                if player.position.y() >= DOWN_BORDER {
                    player.position = Point::new(player.position.x(), UP_BORDER);
                }
                player.position = player.position.offset(0, player.speed);
            }
        }
    }
}