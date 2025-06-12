use std::{os::unix::fs::DirBuilderExt, thread::current};

use getch_rs::{Getch, Key};
use rand::{Rng, rngs::ThreadRng, seq::IndexedRandom};

const MAZE_WIDTH: usize = 8;
const MAZE_HEIGHT: usize = 8;

#[derive(Clone, Copy)]
enum DirectionEnum {
    North = 0,
    West,
    South,
    East,
    Max,
}

impl TryFrom<usize> for DirectionEnum {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == DirectionEnum::North as usize => Ok(DirectionEnum::North),
            x if x == DirectionEnum::West as usize => Ok(DirectionEnum::West),
            x if x == DirectionEnum::South as usize => Ok(DirectionEnum::South),
            x if x == DirectionEnum::East as usize => Ok(DirectionEnum::East),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
struct Tile {
    walls: [bool; DirectionEnum::Max as usize],
}

impl Tile {
    pub fn new() -> Self {
        Self {
            walls: [false; DirectionEnum::Max as usize],
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn is_inside_maze(&self) -> bool {
        self.x >= 0 && self.x < MAZE_WIDTH as isize && self.y >= 0 && self.y < MAZE_HEIGHT as isize
    }
    pub fn add(&mut self, other: &Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
    pub fn add_new(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

struct Character {
    position: Vec2,
    direction: DirectionEnum,
}

impl Character {
    pub fn new() -> Self {
        Self {
            position: Vec2::default(),
            direction: DirectionEnum::North,
        }
    }
    pub fn turn_left(&mut self) {
        self.direction = ((self.direction as usize + 1) % DirectionEnum::Max as usize)
            .try_into()
            .unwrap();
    }
    pub fn turn_back(&mut self) {
        self.direction = ((self.direction as usize + 2) % DirectionEnum::Max as usize)
            .try_into()
            .unwrap();
    }
    pub fn turn_right(&mut self) {
        self.direction = ((self.direction as usize + DirectionEnum::Max as usize - 1)
            % DirectionEnum::Max as usize)
            .try_into()
            .unwrap();
    }
}

struct Context {
    maze: [Tile; MAZE_HEIGHT * MAZE_WIDTH],
    directions: [Vec2; DirectionEnum::Max as usize],
    player: Character,
    g: Getch,
    rng: ThreadRng,
}

impl Context {
    pub fn new() -> Self {
        let directions = [
            Vec2::new(0, -1),
            Vec2::new(-1, 0),
            Vec2::new(0, 1),
            Vec2::new(1, 0),
        ];
        Self {
            maze: [Tile::new(); MAZE_HEIGHT * MAZE_WIDTH],
            player: Character::new(),
            directions,
            g: Getch::new(),
            rng: rand::rng(),
        }
    }
    pub fn draw_map(&self) {
        for y in 0..MAZE_HEIGHT {
            for x in 0..MAZE_WIDTH {
                print!(
                    "+{}+",
                    if self.maze[y * MAZE_WIDTH + x].walls[DirectionEnum::North as usize] {
                        '-'
                    } else {
                        ' '
                    }
                );
            }
            println!();
            for x in 0..MAZE_WIDTH {
                let mut floor_aa = ' ';
                if x == self.player.position.x as usize && y == self.player.position.y as usize {
                    const DIRECTION_AA: [char; DirectionEnum::Max as usize] = ['↑', '←', '↓', '→'];
                    floor_aa = DIRECTION_AA[self.player.direction as usize];
                }
                print!(
                    "{}{}{}",
                    if self.maze[y * MAZE_WIDTH + x].walls[DirectionEnum::West as usize] {
                        '-'
                    } else {
                        ' '
                    },
                    floor_aa,
                    if self.maze[y * MAZE_WIDTH + x].walls[DirectionEnum::East as usize] {
                        '-'
                    } else {
                        ' '
                    }
                );
            }
            println!();
            for x in 0..MAZE_WIDTH {
                print!(
                    "+{}+",
                    if self.maze[y * MAZE_WIDTH + x].walls[DirectionEnum::South as usize] {
                        '-'
                    } else {
                        ' '
                    }
                );
            }
            println!();
        }
    }
    pub fn init(&mut self) {
        self.generate_map();

        self.player.position = Vec2::new(0, 0);

        self.player.direction = DirectionEnum::North;
    }
    fn generate_map(&mut self) {
        for y in 0..MAZE_HEIGHT {
            for x in 0..MAZE_WIDTH {
                for i in 0..DirectionEnum::Max as usize {
                    self.maze[y * MAZE_WIDTH + x].walls[i] = true;
                }
            }
        }

        let mut current_position = Vec2::new(0, 0);
        let mut to_dig_wall_positions: Vec<Vec2> = Vec::new();

        to_dig_wall_positions.push(current_position);

        loop {
            let mut can_dig_directions = Vec::new();

            for i in 0..DirectionEnum::Max as usize {
                if self.can_dig_wall(&current_position, i.try_into().unwrap()) {
                    can_dig_directions.push(i);
                }
            }

            if can_dig_directions.len() > 0 {
                let dig_direction = *can_dig_directions.choose(&mut self.rng).unwrap();

                self.dig_wall(&current_position, dig_direction.try_into().unwrap());

                current_position.add(&self.directions[dig_direction]);

                to_dig_wall_positions.push(current_position);
            } else {
                to_dig_wall_positions.remove(0);

                if to_dig_wall_positions.len() <= 0 {
                    break;
                }

                current_position = to_dig_wall_positions[0];
            }
        }
    }
    fn dig_wall(&mut self, position: &Vec2, direction: DirectionEnum) {
        if !position.is_inside_maze() {
            return;
        }
        self.maze[position.y as usize * MAZE_WIDTH + position.x as usize].walls
            [direction as usize] = false;

        let next_position = position.add_new(&self.directions[direction as usize]);
        if next_position.is_inside_maze() {
            let next_direction = (direction as usize + 2) % DirectionEnum::Max as usize;

            self.maze[next_position.y as usize * MAZE_WIDTH + next_position.x as usize].walls
                [next_direction as usize] = false;
        }
    }
    fn can_dig_wall(&self, position: &Vec2, direction: DirectionEnum) -> bool {
        let next_position = position.add_new(&self.directions[direction as usize]);
        if !next_position.is_inside_maze() {
            return false;
        }
        for i in 0..DirectionEnum::Max as usize {
            if !self.maze[next_position.y as usize * MAZE_WIDTH + next_position.x as usize].walls[i]
            {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let mut ctx = Context::new();

    ctx.init();

    loop {
        clearscreen::clear().unwrap();

        ctx.draw_map();

        match ctx.g.getch() {
            Ok(Key::Char('w')) => {
                if !ctx.maze
                    [ctx.player.position.y as usize * MAZE_WIDTH + ctx.player.position.x as usize]
                    .walls[ctx.player.direction as usize]
                {
                    let next_position = ctx
                        .player
                        .position
                        .add_new(&ctx.directions[ctx.player.direction as usize]);

                    if next_position.is_inside_maze() {
                        ctx.player.position = next_position;
                    }
                }
            }
            Ok(Key::Char('a')) => {
                ctx.player.turn_left();
            }
            Ok(Key::Char('s')) => {
                ctx.player.turn_back();
            }
            Ok(Key::Char('d')) => {
                ctx.player.turn_right();
            }
            Ok(Key::Esc) => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}
