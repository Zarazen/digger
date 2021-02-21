use crate::game::{PLAYER_SPEED, MONSTER_SPEED, SHOT_SPEED, BLOCK_SIZE, BAG_SPEED};
use crate::immovable_objects::*;
use std::time::{Instant};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Movable {
    pub type_object: MovableType,
    pub x: f64,
    pub y: f64,
    pub dir: Direction,
    pub activation_time: std::time::Instant,
    pub previous_dir: Direction,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MovableType {
    Player,
    Monster,
    FallingBag,
    Shot,
    Crashed,
    Converted,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

pub fn move_object(input: Vec<Movable>) -> Vec<Movable> {
    let mut output = Vec::<Movable>::new();
    for i in 0..input.len() {
        output.push(input[i]);
        match output[i].type_object {
            MovableType::Player => {
                match output[i].dir {
                    Direction::Up => output[i].x -= PLAYER_SPEED,
                    Direction::Down => output[i].x += PLAYER_SPEED,
                    Direction::Left => output[i].y -= PLAYER_SPEED,
                    Direction::Right => output[i].y += PLAYER_SPEED,
                    _ => {}
                }
            },
            MovableType::Monster => {
                match output[i].dir {
                    Direction::Up => output[i].x -= MONSTER_SPEED,
                    Direction::Down => output[i].x += MONSTER_SPEED,
                    Direction::Left => output[i].y -= MONSTER_SPEED,
                    Direction::Right => output[i].y += MONSTER_SPEED,
                    _ => {}
                }
            }
            MovableType::FallingBag => {
                if output[i].dir == Direction::Down {
                    output[i].x += BAG_SPEED;
                };
            }
            MovableType::Shot => {
                match output[i].dir {
                    Direction::Up => output[i].x -= SHOT_SPEED,
                    Direction::Down => output[i].x += SHOT_SPEED,
                    Direction::Left => output[i].y -= SHOT_SPEED,
                    Direction::Right => output[i].y += SHOT_SPEED,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    output
}



pub fn overlaps(obj1: Movable, obj2: Movable) -> bool {

    if obj1.x - obj2.x < 0.1 
    && obj1.x - obj2.x > -0.1 
    && obj1.y - obj2.y < BLOCK_SIZE as f64 
    && obj1.y - obj2.y > -(BLOCK_SIZE as f64) {
        return true;
    }

    if obj1.y - obj2.y < 0.1 
    && obj1.y - obj2.y > -0.1 
    && obj1.x - obj2.x < BLOCK_SIZE as f64 
    && obj1.x - obj2.x > -(BLOCK_SIZE as f64) {
        return true;
    }
    
    false
}

pub fn bag_or_shot_crashed(obj: Movable, walls: Vec<Vec<Immovable>>) -> bool {
    let obj_X = (obj.x / (BLOCK_SIZE as f64)) as usize;
    let obj_Y = (obj.y / (BLOCK_SIZE as f64)) as usize;

    match obj.dir {
        Direction::Up => {
            if obj_X == 0 && obj.x < 0.1 {
                return true;
            }
            if obj_X > 0 && 
            (walls[obj_X - 1][obj_Y].type_object == ImmovableType::Bag
            || walls[obj_X - 1][obj_Y].type_object == ImmovableType::Emerald
            || walls[obj_X - 1][obj_Y].type_object == ImmovableType::Wall)
            && obj.x % (BLOCK_SIZE as f64) < 1.0 {
                return true;
            }
            return false;
        },
        Direction::Down => {
            if obj_X >= walls.len() - 1 {
                return true;
            }
            if obj_X < walls.len() - 2 && walls[obj_X + 1][obj_Y].type_object == ImmovableType::Background {
                return false;
            }
            if obj_X < walls.len() - 2 {
                return overlaps(obj, Movable{
                    type_object: MovableType::Converted,
                    x: walls[obj_X + 1][obj_Y].x,
                    y: walls[obj_X + 1][obj_Y].y,
                    dir: Direction::None,
                    activation_time: Instant::now(),
                    previous_dir: Direction::None,
                });
            }
            return false;
        },
        Direction::Left => {
            if obj_Y == 0 && obj.y < 0.1 {
                return true;
            }
            if obj_Y > 0 && 
            (walls[obj_X][obj_Y - 1].type_object == ImmovableType::Bag
            || walls[obj_X][obj_Y - 1].type_object == ImmovableType::Emerald
            || walls[obj_X][obj_Y - 1].type_object == ImmovableType::Wall)
            && obj.y % (BLOCK_SIZE as f64) < 1.0 {
                return true;
            }
            return false;
        },
        Direction::Right => {
            if obj_Y >= walls[0].len() - 1 {
                return true;
            }
            if obj_Y < walls[0].len() - 2 && walls[obj_X][obj_Y + 1].type_object == ImmovableType::Background {
                return false;
            }
            if obj_Y < walls[0].len() - 2 {
                return overlaps(obj, Movable{
                    type_object: MovableType::Converted,
                    x: walls[obj_X][obj_Y + 1].x,
                    y: walls[obj_X][obj_Y + 1].y,
                    dir: Direction::None,
                    activation_time: Instant::now(),
                    previous_dir: Direction::None,
                });
            }
            return false;
        },
        _ => {}
    }
    false
}