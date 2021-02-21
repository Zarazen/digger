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
