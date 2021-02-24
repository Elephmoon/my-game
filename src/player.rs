use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub struct Player {
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
}

impl Player {
    pub fn new(position: Point, sprite: Rect, speed: i32) -> Player {
        Player {
            position,
            sprite,
            speed,
        }
    }
}