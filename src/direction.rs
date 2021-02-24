const UP_ROW: i32 = 3;
const DOWN_ROW: i32 = 0;
const LEFT_ROW: i32 = 1;
const RIGHT_ROW: i32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn direction_sprite_row(direction: Direction) -> i32 {
        use self::Direction::*;
        return match direction {
            Up => UP_ROW,
            Down => DOWN_ROW,
            Left => LEFT_ROW,
            Right => RIGHT_ROW,
        };
    }
}