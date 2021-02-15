use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub struct Player {
    pub position: Point,
    pub sprite: Rect,
}

impl Player {
    pub fn new(position: Point, sprite: Rect) -> Player {
        Player {
            position,
            sprite,
        }
    }
}