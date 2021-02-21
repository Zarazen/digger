use crate::game::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    //#[allow(dead_code)]
    pub fn move_to_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 2,
            Direction::Down => self.y += 2,
            Direction::Left => self.x -= 2,
            Direction::Right => self.x += 2,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

