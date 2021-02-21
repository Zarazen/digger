mod game;
mod immovable_objects;
mod movable_objects;

use immovable_objects::*;
use movable_objects::*;
use game::*;
use piston_window::*;

const MAP_PATH: &str = "map.txt";

fn main() {

    let mut g = Game::new();
    g.initialize(MAP_PATH);
    g.print();
    return;

}

#[cfg(test)]
mod tests{
    #[test]
    fn test_overlapping() {

        assert_eq!(
            super::overlaps(super::Movable{
                type_object: super::MovableType::Player,
                x: 0.0,
                y: 0.0,
                dir: super::Direction::None,
                activation_time: std::time::Instant::now(),
                previous_dir: super::Direction::None,
            }, 
            super::Movable{
                type_object: super::MovableType::Player,
                x: 0.0,
                y: 1.0 + (super::BLOCK_SIZE as f64),
                dir: super::Direction::None,
                activation_time: std::time::Instant::now(),
                previous_dir: super::Direction::None,
            }), 
            false);

        assert_eq!(
            super::overlaps(super::Movable{
                type_object: super::MovableType::Player,
                x: 2.0 * (super::BLOCK_SIZE as f64) + 15.0,
                y: 3.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::None,
                activation_time: std::time::Instant::now(),
                previous_dir: super::Direction::None,
            }, 
            super::Movable{
                type_object: super::MovableType::Player,
                x: 4.0 * (super::BLOCK_SIZE as f64) + 1.0,
                y: 3.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::None,
                activation_time: std::time::Instant::now(),
                previous_dir: super::Direction::None,
            }), 
            false);

            assert_eq!(
                super::overlaps(super::Movable{
                type_object: super::MovableType::Player,
                x: 2.0 * (super::BLOCK_SIZE as f64) + 15.0,
                y: 7.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::None,
                activation_time: std::time::Instant::now(),
                previous_dir: super::Direction::None,
            }, 
                super::Movable{
                type_object: super::MovableType::Player,
                x: 3.0 * (super::BLOCK_SIZE as f64) + 1.0,
                y: 7.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::None,
                activation_time: std::time::Instant::now(),
                previous_dir: super::Direction::None,
            }), 
            true);

            assert_eq!(
                super::overlaps(super::Movable{
                type_object: super::MovableType::Player,
                x: 4.0 * (super::BLOCK_SIZE as f64),
                y: 7.0 * (super::BLOCK_SIZE as f64) + 6.0,
                dir: super::Direction::None,
                activation_time: std::time::Instant::now(),
                previous_dir: super::Direction::None,
            }, 
                super::Movable{
                type_object: super::MovableType::Player,
                x: 4.0 * (super::BLOCK_SIZE as f64),
                y: 7.0 * (super::BLOCK_SIZE as f64) - 11.0,
                dir: super::Direction::None,
                activation_time: std::time::Instant::now(),
                previous_dir: super::Direction::None,
            }), 
            true);
    }
    #[test]
    fn test_moving() {
        let input = vec![
            super::Movable {
                type_object: super::MovableType::Player,
                x: 4.0 * (super::BLOCK_SIZE as f64),
                y: 7.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::None,
                activation_time: std::time::Instant::now(),
                previous_dir: super::Direction::None,
            },
            super::Movable{
                type_object: super::MovableType::Monster,
                x: 5.0 * (super::BLOCK_SIZE as f64),
                y: 2.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::None,
                activation_time: std::time::Instant::now(),
                previous_dir: super::Direction::None,
            }
        ];
        assert_eq!(super::move_object(input.clone()),input);

        let now = std::time::Instant::now();
        let input1 = vec![
            super::Movable {
                type_object: super::MovableType::Player,
                x: 4.0 * (super::BLOCK_SIZE as f64),
                y: 7.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::Right,
                activation_time: now,
                previous_dir: super::Direction::None,
            },
            super::Movable{
                type_object: super::MovableType::Monster,
                x: 5.0 * (super::BLOCK_SIZE as f64),
                y: 2.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::Up,
                activation_time: now,
                previous_dir: super::Direction::None,
            }
        ];
        let output1 = vec![
            super::Movable {
                type_object: super::MovableType::Player,
                x: 4.0 * (super::BLOCK_SIZE as f64),
                y: 7.0 * (super::BLOCK_SIZE as f64) + (super::BLOCK_SIZE as f64),
                dir: super::Direction::Right,
                activation_time: now,
                previous_dir: super::Direction::None,
            },
            super::Movable{
                type_object: super::MovableType::Monster,
                x: 5.0 * (super::BLOCK_SIZE as f64) - super::MONSTER_SPEED,
                y: 2.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::Up,
                activation_time: now,
                previous_dir: super::Direction::None,
            }
        ];
        assert_eq!(super::move_object(input1),output1);

        let input2 = vec![
            super::Movable {
                type_object: super::MovableType::FallingBag,
                x: 13.0 * (super::BLOCK_SIZE as f64),
                y: 17.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::Down,
                activation_time: now,
                previous_dir: super::Direction::None,
            },
            super::Movable{
                type_object: super::MovableType::Shot,
                x: 15.0 * (super::BLOCK_SIZE as f64),
                y: 12.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::Left,
                activation_time: now,
                previous_dir: super::Direction::None,
            }
        ];
        let output2 = vec![
            super::Movable {
                type_object: super::MovableType::FallingBag,
                x: 13.0 * (super::BLOCK_SIZE as f64) + super::BAG_SPEED,
                y: 17.0 * (super::BLOCK_SIZE as f64),
                dir: super::Direction::Down,
                activation_time: now,
                previous_dir: super::Direction::None,
            },
            super::Movable{
                type_object: super::MovableType::Shot,
                x: 15.0 * (super::BLOCK_SIZE as f64),
                y: 12.0 * (super::BLOCK_SIZE as f64) - super::SHOT_SPEED,
                dir: super::Direction::Left,
                activation_time: now,
                previous_dir: super::Direction::None,
            }
        ];
        assert_eq!(super::move_object(input2),output2);
    }
}