
use std::collections::VecDeque;
use piston_window::*;
use crate::immovable_objects::*;
use crate::movable_objects::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::time::{Duration, Instant};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    InvalidFile,
    PlayerDead,
    Victory,
}

const OPENGL: OpenGL = OpenGL::V3_2;
pub const BLOCK_SIZE: usize = 30;
const MAX_FPS: u64 = 18;
pub const PLAYER_SPEED: f64 = BLOCK_SIZE as f64;
pub const MONSTER_SPEED: f64 = 0.5;
pub const BAG_SPEED: f64 = 0.5;
pub const SHOT_SPEED: f64 = 0.8;
const MAX_MONSTERS: usize = 3;
const SCORE_PER_EMERALD: usize = 20;
const BACKGROUND_PATH: &str = "background.png";
const PLAYER_PATH: &str = "right.png";
const BAG_PATH: &str = "bag.png";
const BROKEN_BAG_PATH: &str = "broken.png";
const MONSTER_PATH: &str = "monster.png";
const FALLING_BAG_PATH: &str = "falling.png";
const EMERALD_PATH: &str = "emerald.png";
const DEATH_PATH: &str = "rip.png";
const SHOT_PATH: &str = "shot.png";
const WALL_PATH: &str = "wall.png";
const TIME_TO_FALL: u64 = 3;
const TIME_TO_SPAWN: u64 = 3;

pub struct Game {
    pub immovable: Vec<Vec<Immovable>>,
    pub immovable_texture: Vec<Vec<G2dTexture>>,
    pub movable: Vec<Movable>,
    pub movable_texture: Vec<G2dTexture>,
    line_len: usize,
    num_lines: usize,
    win: PistonWindow,
    spawn_x: f64,
    spawn_y: f64,
    previous: Vec<i32>,
    max_score: usize,
    current_score: usize,
    shots: usize,
    last_spawned: std::time::Instant,
    current_number_monsters: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            immovable: Vec::<Vec<Immovable>>::new(),
            immovable_texture: Vec::<Vec<G2dTexture>>::new(),
            movable: Vec::<Movable>::new(),
            movable_texture: Vec::<G2dTexture>::new(),
            line_len: 0,
            num_lines: 0,
            win: WindowSettings::new("DIGGER", [10, 10])
            .exit_on_esc(true)
            .graphics_api(OPENGL)
            .build()
            .unwrap(),
            spawn_x: 0.0,
            spawn_y: 0.0,
            previous: Vec::<i32>::new(),
            max_score: 0,
            current_score: 0,
            shots: 0,
            last_spawned: Instant::now(),
            current_number_monsters: 0,
        }
    }

    pub fn initialize(&mut self, path: &str) {
        let file = File::open(path);
        let f;
        match file {
            Ok(x) => {
                f = BufReader::new(x);
            }
            Err(e) => {
                println!("{:?}", e);
                return
            },
        }
        for line in f.lines() {
            let unwrapped = line.unwrap();
            let splitted = Game::split_row(&unwrapped);
            let splitted_as_vec;
            match splitted {
                Some(x) => splitted_as_vec = x,
                None => return,
            }
            
            let added = self.add_row(splitted_as_vec);
            match added {
                Ok(()) => {},
                Err(e) => {
                    println!("{:?}", e);
                    return;
                }
            }
            self.num_lines += 1;
        }

        self.win.set_size(Size {width: (self.line_len * BLOCK_SIZE) as f64, height: (self.num_lines * BLOCK_SIZE) as f64});
        self.win.set_max_fps(MAX_FPS);
        self.win.set_ups(MAX_FPS);
        self.win.set_ups(MAX_FPS);
        self.previous.resize(self.line_len * self.num_lines, -1);
    }

    fn split_row(input: &str) -> Option<Vec<&str>> {
        let mut output = Vec::<&str>::new();
        let mut flag = false;

        let mut curr = input;
        while curr != "" {
            let splitted = Game::take_and_skip(curr, ',');
            match splitted {
                Some((x, y)) => {
                    output.push(x);
                    curr = y;
                },
                None => {
                    flag = true;
                    break;
                },
            }
        }

        match flag {
            false => Some(output),
            true => None,
        }
    }

    fn add_row(&mut self, input: Vec<&str>) -> Result<(), RuntimeError> {

        if self.line_len == 0 {
            self.line_len = input.len();
        };
        if self.line_len != input.len() {
            return Err(RuntimeError::InvalidFile);
        }

        let current_x = self.immovable.len();
        let mut new_immovable_line = Vec::<Immovable>::new();
        let mut new_immovable_texture_line = Vec::<G2dTexture>::new();

        for i in 0..input.len() {
            match input[i] {
                "space" => {
                    new_immovable_line.push(Immovable{
                        type_object: ImmovableType::Background,
                        x: (current_x * BLOCK_SIZE) as f64,
                        y: (i * BLOCK_SIZE) as f64,
                    });
                    new_immovable_texture_line.push(Texture::from_path(
                        &mut self.win.create_texture_context(),
                        BACKGROUND_PATH,
                        Flip::None,
                        &TextureSettings::new()
                    ).unwrap());
                } ,
                "wall" => {
                    new_immovable_line.push(Immovable{
                        type_object: ImmovableType::Wall,
                        x: (current_x * BLOCK_SIZE) as f64,
                        y: (i * BLOCK_SIZE) as f64,
                    });
                    new_immovable_texture_line.push(Texture::from_path(
                        &mut self.win.create_texture_context(),
                        WALL_PATH,
                        Flip::None,
                        &TextureSettings::new()
                    ).unwrap());
                } ,
                "gold" => {
                    new_immovable_line.push(Immovable{
                        type_object: ImmovableType::Bag,
                        x: (current_x * BLOCK_SIZE) as f64,
                        y: (i * BLOCK_SIZE) as f64,
                    });
                    new_immovable_texture_line.push(Texture::from_path(
                        &mut self.win.create_texture_context(),
                        BAG_PATH,
                        Flip::None,
                        &TextureSettings::new()
                    ).unwrap());
                } ,
                "diamond" => {
                    new_immovable_line.push(Immovable{
                        type_object: ImmovableType::Emerald,
                        x: (current_x * BLOCK_SIZE) as f64,
                        y: (i * BLOCK_SIZE) as f64,
                    });
                    new_immovable_texture_line.push(Texture::from_path(
                        &mut self.win.create_texture_context(),
                        EMERALD_PATH,
                        Flip::None,
                        &TextureSettings::new()
                    ).unwrap());
                    self.max_score += SCORE_PER_EMERALD;
                } ,
                "player" => {
                    new_immovable_line.push(Immovable{
                        type_object: ImmovableType::Background,
                        x: (current_x * BLOCK_SIZE) as f64,
                        y: (i * BLOCK_SIZE) as f64,
                    });
                    new_immovable_texture_line.push(Texture::from_path(
                        &mut self.win.create_texture_context(),
                        BACKGROUND_PATH,
                        Flip::None,
                        &TextureSettings::new()
                    ).unwrap());
                    self.movable.push(Movable{
                        type_object: MovableType::Player,
                        x: (current_x * BLOCK_SIZE) as f64,
                        y: (i * BLOCK_SIZE) as f64,
                        dir: Direction::None,
                        activation_time: Instant::now(),
                        previous_dir: Direction::Right,
                    });
                    self.movable_texture.push(Texture::from_path(
                        &mut self.win.create_texture_context(),
                        PLAYER_PATH,
                        Flip::None,
                        &TextureSettings::new()
                    ).unwrap());
                }
                "spawn" => {
                    new_immovable_line.push(Immovable{
                        type_object: ImmovableType::Background,
                        x: (current_x * BLOCK_SIZE) as f64,
                        y: (i * BLOCK_SIZE) as f64,
                    });
                    new_immovable_texture_line.push(Texture::from_path(
                        &mut self.win.create_texture_context(),
                        BACKGROUND_PATH,
                        Flip::None,
                        &TextureSettings::new()
                    ).unwrap());
                    self.spawn_x = (current_x * BLOCK_SIZE) as f64;
                    self.spawn_y = (i * BLOCK_SIZE) as f64;
                    self.current_number_monsters += 1;
                    self.movable.push(Movable{
                        type_object: MovableType::Monster,
                        x: (current_x * BLOCK_SIZE) as f64,
                        y: (i * BLOCK_SIZE) as f64,
                        dir: Direction::None,
                        activation_time: Instant::now(),
                        previous_dir: Direction::None,
                    });
                    self.movable_texture.push(Texture::from_path(
                        &mut self.win.create_texture_context(),
                        MONSTER_PATH,
                        Flip::None,
                        &TextureSettings::new()
                    ).unwrap());
                },
                _ => break,
            }
        }

        self.immovable.push(new_immovable_line);
        self.immovable_texture.push(new_immovable_texture_line);
        self.last_spawned = Instant::now();

        Ok(())
    }

    fn skip_next(input: &str, target: char) -> Option<&str> {
        let mut chars = input.chars();
        match chars.next() {
            Some(ch) if ch == target => Some(chars.as_str()),
            _ => None,
        }
    }
    
    fn take_until(input: &str, target: char) -> (&str, &str) {
        match input.find(target) {
            Some(pos) => (&input[..pos], &input[pos..]),
            None => (input, ""),
        }
    }
    
    fn take_and_skip(input: &str, target: char) -> Option<(&str, &str)> {
        let (first, second) = Game::take_until(input, target);
        let second = Game::skip_next(second, target);
        match second {
            Some(x) => Some((first, x)),
            None => None
        }
    }

    pub fn print(&mut self) -> Result<(), RuntimeError> {
        while let Some(e) = self.win.next() {
            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.key_down(key);
            }

            let mut cp = self.immovable_texture.clone();
            let mut ci = self.immovable.clone();
            let mut cpm = self.movable_texture.clone();
            let mut cm = self.movable.clone();

            self.win.draw_2d(&e, |c, g, _| {
                for i in 0..cp.len() {
                    for j in 0..cp[i].len() {
                        image(&cp[i][j], c.transform.trans(ci[i][j].y, ci[i][j].x), g);
                    }
                }
                for i in 0..cpm.len() {
                    image(&cpm[i], c.transform.trans(cm[i].y, cm[i].x), g);
                }
            });
            cm = move_object(cm);
            match self.check_for_colision(cm.clone(), cpm.clone(), ci.clone()) {
                Err(e) => {
                    return Err(e);
                },
                Ok((x,y)) => {
                    cm = x.clone();
                    cpm = y .clone();
                }
            }
            match self.update_game(&mut ci, &mut cp, &mut cm, &mut cpm) {
                Err(e) => {
                    return Err(e);
                },
                _ => {}
            }
        }
        Ok(())
    }

    fn key_down(&mut self, key: keyboard::Key) {

        let player_position = self.player_pos();
        let X = (self.movable[player_position].x / (BLOCK_SIZE as f64)) as usize;
        let Y = (self.movable[player_position].y / (BLOCK_SIZE as f64)) as usize;
        match key {
            Key::A | Key::Left => {
                if Y > 0 && self.immovable[X][Y - 1].type_object != ImmovableType::Bag {
                    self.movable[player_position].dir = Direction::Left;
                };
            },
            Key::W | Key::Up => {
                if X > 0 && self.immovable[X - 1][Y].type_object != ImmovableType::Bag {
                    self.movable[player_position].dir = Direction::Up;
                };
            },
            Key::D | Key::Right => {
                if Y < self.line_len - 1 && self.immovable[X][Y + 1].type_object != ImmovableType::Bag {
                    self.movable[player_position].dir = Direction::Right;
                };
            },
            Key::S | Key::Down => {
                if X < self.num_lines - 1 && self.immovable[X + 1][Y].type_object != ImmovableType::Bag {
                    self.movable[player_position].dir = Direction::Down;
                };
            },
            Key::Space => {
                if self.shots < 1 {
                    return;
                }
                self.shots -= 1;
                let player_position = self.player_pos();
                match self.movable[player_position].previous_dir {
                    Direction::Right => {
                        self.movable.push(Movable{
                            type_object: MovableType::Shot,
                            x: self.movable[player_position].x,
                            y: self.movable[player_position].y + (BLOCK_SIZE as f64),
                            dir: Direction::Right,
                            activation_time: Instant::now(),
                            previous_dir: Direction::Right,
                        });
                        self.movable_texture.push(Texture::from_path(
                            &mut self.win.create_texture_context(),
                            SHOT_PATH,
                            Flip::None,
                            &TextureSettings::new()
                        ).unwrap());
                    },
                    Direction::Left => {
                        self.movable.push(Movable{
                            type_object: MovableType::Shot,
                            x: self.movable[player_position].x,
                            y: self.movable[player_position].y - (BLOCK_SIZE as f64),
                            dir: Direction::Left,
                            activation_time: Instant::now(),
                            previous_dir: Direction::Left,
                        });
                        self.movable_texture.push(Texture::from_path(
                            &mut self.win.create_texture_context(),
                            SHOT_PATH,
                            Flip::None,
                            &TextureSettings::new()
                        ).unwrap());
                    },
                    Direction::Up => {
                        self.movable.push(Movable{
                            type_object: MovableType::Shot,
                            x: self.movable[player_position].x - (BLOCK_SIZE as f64),
                            y: self.movable[player_position].y,
                            dir: Direction::Up,
                            activation_time: Instant::now(),
                            previous_dir: Direction::Up,
                        });
                        self.movable_texture.push(Texture::from_path(
                            &mut self.win.create_texture_context(),
                            SHOT_PATH,
                            Flip::None,
                            &TextureSettings::new()
                        ).unwrap());
                    },
                    Direction::Down => {
                        self.movable.push(Movable{
                            type_object: MovableType::Shot,
                            x: self.movable[player_position].x + (BLOCK_SIZE as f64),
                            y: self.movable[player_position].y,
                            dir: Direction::Down,
                            activation_time: Instant::now(),
                            previous_dir: Direction::Down,
                        });
                        self.movable_texture.push(Texture::from_path(
                            &mut self.win.create_texture_context(),
                            SHOT_PATH,
                            Flip::None,
                            &TextureSettings::new()
                        ).unwrap());
                    },
                    _ => {}
                }
            }
            _ => {
                self.movable[player_position].dir = Direction::None;
            }
        }
    }

    fn player_pos(&mut self) -> usize {
        for i in 0..self.movable.len() {
            if self.movable[i].type_object == MovableType::Player {
                return i;
            };
        }
        0
    }

    fn stop_player_movement(&mut self) {
        let player_position = self.player_pos();
        if self.movable[player_position].dir != Direction::None {
            self.movable[player_position].previous_dir = self.movable[player_position].dir;
        };
        self.movable[player_position].dir = Direction::None;
    }

    fn update_game(&mut self, 
        cp_immovable: &mut Vec<Vec<Immovable>>, 
        cp_immovable_texture: &mut Vec<Vec<G2dTexture>>,
        cp_movable: &mut Vec<Movable>,
        cp_movable_texture: &mut Vec<G2dTexture>) -> Result<(), RuntimeError> {

            let player_position = self.player_pos();
            let X = (self.movable[player_position].x / (BLOCK_SIZE as f64)) as usize;
            let Y = (self.movable[player_position].y / (BLOCK_SIZE as f64)) as usize;
            if cp_immovable[X][Y].type_object == ImmovableType::Emerald {
                self.current_score += SCORE_PER_EMERALD;
            };
            if self.current_score >= self.max_score {
                return Err(RuntimeError::Victory);
            };
            cp_immovable[X][Y].type_object = ImmovableType::Background;
            cp_immovable_texture[X][Y] = 
                Texture::from_path(
                &mut self.win.create_texture_context(),
                BACKGROUND_PATH,
                Flip::None,
                &TextureSettings::new()
            ).unwrap();

            self.immovable_texture = cp_immovable_texture.clone();
            self.immovable = cp_immovable.clone();
            self.movable_texture = cp_movable_texture.clone();
            self.movable = cp_movable.clone();
            self.activate_bags();
            self.drop_bags();
            self.stop_player_movement();
            self.find_shortest_paths();
            self.spawn_monster();
            self.redirect_monsters();
            
            Ok(())
    }

    fn find_shortest_paths(&mut self) {
        let player_pososition = self.player_pos();
        let player_X = (self.movable[player_pososition].x / (BLOCK_SIZE as f64)) as usize;
        let player_Y = (self.movable[player_pososition].y / (BLOCK_SIZE as f64)) as usize;
        let mut visited = Vec::<bool>::new();
        for i in 0..self.previous.len() {
            self.previous[i] = -1;
            visited.push(false);
        }
        let mut queue = VecDeque::<usize>::new();
        queue.push_back(player_X * self.line_len + player_Y);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            visited[current] = true;
            let current_X = current / self.line_len;
            let current_Y = current % self.line_len;

            if current_Y < self.line_len - 1
            && self.immovable[current_X][current_Y + 1].type_object == ImmovableType::Background 
            && !visited[current + 1] {
                queue.push_back(current + 1);
                self.previous[current + 1] = current as i32;
            };

            if current_Y > 0 
            && self.immovable[current_X][current_Y - 1].type_object == ImmovableType::Background 
            && !visited[current - 1] {
                queue.push_back(current - 1);
                self.previous[current - 1] = current as i32;
            };

            if current_X < self.num_lines - 1 
            && self.immovable[current_X + 1][current_Y].type_object == ImmovableType::Background 
            && !visited[current + self.line_len]{
                queue.push_back(current + self.line_len);
                self.previous[current + self.line_len] = current as i32;
            };

            if current_X > 0 
            && self.immovable[current_X - 1][current_Y].type_object == ImmovableType::Background 
            && !visited[current - self.line_len]{
                queue.push_back(current - self.line_len);
                self.previous[current - self.line_len] = current as i32;
            };
        }

    }

    fn redirect_monsters(&mut self) {
        for i in 0..self.movable.len() {
            if self.movable[i].type_object == MovableType::Monster {
                let monster_X = (self.movable[i].x / (BLOCK_SIZE as f64)) as i32;
                let monster_Y = (self.movable[i].y / (BLOCK_SIZE as f64)) as i32;

                let previous_position = self.previous[(monster_X as usize) * self.line_len + (monster_Y as usize)] as i32;

                let previous_X = previous_position / (self.line_len as i32);
                let previous_Y = previous_position % (self.line_len as i32);
                
                if previous_X == monster_X - 1  {
                    self.movable[i].dir = Direction::Up;
                    self.movable[i].y = (monster_Y * (BLOCK_SIZE as i32)) as f64;
                    continue;
                }

                if previous_X == monster_X + 1  {
                    self.movable[i].dir = Direction::Down;
                    self.movable[i].y = (monster_Y * (BLOCK_SIZE as i32)) as f64;
                    continue;
                }

                if previous_Y == monster_Y - 1 {
                    self.movable[i].dir = Direction::Left;
                    self.movable[i].x = (monster_X * (BLOCK_SIZE as i32)) as f64;
                    continue;
                }

                if previous_Y == monster_Y + 1 {
                    self.movable[i].dir = Direction::Right;
                    self.movable[i].x = (monster_X * (BLOCK_SIZE as i32)) as f64;
                    continue;
                }
            }
        }
    }

    fn activate_bags(&mut self) {
        let player_position = self.player_pos();
        let player_X = (self.movable[player_position].x / (BLOCK_SIZE as f64)) as usize;
        let player_Y = (self.movable[player_position].y / (BLOCK_SIZE as f64)) as usize;

        if player_X > 0 && self.immovable[player_X - 1][player_Y].type_object == ImmovableType::Bag {
            self.immovable[player_X - 1][player_Y].type_object = ImmovableType::Background;
            self.immovable_texture[player_X - 1][player_Y] = Texture::from_path(
                &mut self.win.create_texture_context(),
                BACKGROUND_PATH,
                Flip::None,
                &TextureSettings::new()
            ).unwrap();
            self.movable.push(Movable{
                type_object: MovableType::FallingBag,
                x: ((player_X - 1) * BLOCK_SIZE) as f64,
                y: (player_Y * BLOCK_SIZE) as f64,
                dir: Direction::None,
                activation_time: Instant::now(),
                previous_dir: Direction::None,
            });
            self.movable_texture.push(Texture::from_path(
                &mut self.win.create_texture_context(),
                BAG_PATH,
                Flip::None,
                &TextureSettings::new()
            ).unwrap());
        };
    }

    fn drop_bags(&mut self) {
        for i in 0..self.movable.len() {
            if self.movable[i].type_object == MovableType::FallingBag 
            && self.movable[i].dir == Direction::None 
            && self.movable[i].activation_time.elapsed() >= Duration::from_secs(TIME_TO_FALL) {
                self.movable[i].dir = Direction::Down;
                self.movable_texture[i] = Texture::from_path(
                    &mut self.win.create_texture_context(),
                    FALLING_BAG_PATH,
                    Flip::None,
                    &TextureSettings::new()
                ).unwrap();
            }
        }
    }

    fn check_for_colision(&mut self, 
        input: Vec<Movable>, 
        input_texture: Vec<G2dTexture>,
        input_immovable: Vec<Vec<Immovable>>) 
        -> Result<(Vec<Movable>, Vec<G2dTexture>), RuntimeError> {
        
            let mut output = Vec::<Movable>::new();
            let mut output_texture = Vec::<G2dTexture>::new();
            let mut to_remove = Vec::<usize>::new();
    
            for i in 0..input.len(){
                match input[i].type_object {
                    MovableType::Player => {
                        for j in 0..input.len() {
                            match input[j].type_object {
                                MovableType::Monster | MovableType::FallingBag => {
                                    if overlaps(input[i], input[j]) {
                                        return Err(RuntimeError::PlayerDead);
                                    }
                                }
                                MovableType::Crashed => {
                                    if overlaps(input[i], input[j]) {
                                        self.shots += 1;
                                        to_remove.push(j);
                                    };
                                },
                                _ => {},
                            }
                        }
                        output.push(input[i]);
                        output_texture.push(input_texture[i].clone());
                    }
                    MovableType::Monster => {
                        let mut monster_dead = false;
                        for j in 0..input.len() {
                            if input[j].type_object == MovableType::Shot 
                            || input[j].type_object == MovableType::FallingBag {
                                if overlaps(input[i], input[j]) {
                                    self.current_number_monsters -= 1;
                                    monster_dead = true;
                                    to_remove.push(j);
                                    break;
                                }
                            }
                        }
                        if !monster_dead {
                            output.push(input[i]);
                            output_texture.push(input_texture[i].clone());
                        };
                    }
                    MovableType::FallingBag => {
                        if to_remove.iter().any(|&k| k == i) {
                            continue;
                        }
                        if bag_or_shot_crashed(input[i], input_immovable.clone()) {
                            output.push(Movable{
                                type_object: MovableType::Crashed,
                                x: input[i].x,
                                y: input[i].y,
                                dir: Direction::None,
                                activation_time: Instant::now(),
                                previous_dir: Direction::None,
                            });
                            output_texture.push(Texture::from_path(
                                &mut self.win.create_texture_context(),
                                BROKEN_BAG_PATH,
                                Flip::None,
                                &TextureSettings::new()
                            ).unwrap());
                            continue;
                        }
                        output.push(input[i]);
                        output_texture.push(input_texture[i].clone());
                    }
                    MovableType::Shot => {
                        if !to_remove.iter().any(|&k| k == i) && !bag_or_shot_crashed(input[i], input_immovable.clone()) {
                            output.push(input[i]);
                            output_texture.push(input_texture[i].clone());
                        }
                    },
                    MovableType::Crashed => {
                        if !to_remove.iter().any(|&k| k == i) {
                            output.push(input[i]);
                            output_texture.push(input_texture[i].clone());
                        }
                    }
                    _ => {}
                }
            }
    
            Ok((output, output_texture))
    }

    fn spawn_monster(&mut self) {
        if self.current_number_monsters < MAX_MONSTERS
        && self.last_spawned.elapsed() >= Duration::from_secs(TIME_TO_SPAWN) {
            self.movable.push(Movable{
                type_object: MovableType::Monster,
                x: self.spawn_x,
                y: self.spawn_y,
                dir: Direction::None,
                activation_time: Instant::now(),
                previous_dir: Direction::None,
            });
            self.movable_texture.push(Texture::from_path(
                &mut self.win.create_texture_context(),
                MONSTER_PATH,
                Flip::None,
                &TextureSettings::new()
            ).unwrap());
            self.last_spawned = Instant::now();
            self.current_number_monsters += 1;
        }
    }
}