use std::{os::unix::fs::DirBuilderExt, thread::current};

use getch_rs::{Getch, Key};
use rand::{Rng, rngs::ThreadRng, seq::IndexedRandom};

const MAZE_WIDTH: usize = 8;
const MAZE_HEIGHT: usize = 8;

const GOAL_X: usize = MAZE_WIDTH - 1;
const GOAL_Y: usize = MAZE_HEIGHT - 1;

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

enum LocationEnum {
    FrontLeft = 0,
    FrontRight,
    Front,
    Left,
    Right,
    Center,
    Max,
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

#[derive(Clone, Copy, PartialEq)]
enum AAEnum {
    All = 0,
    FrontLeftNorth,
    FrontRightNorth,
    FrontNorth,
    FrontWest,
    FrontEast,
    LeftNorth,
    RightNorth,
    North,
    West,
    East,
    None,
}

impl TryFrom<usize> for AAEnum {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == AAEnum::All as usize => Ok(AAEnum::All),
            x if x == AAEnum::FrontLeftNorth as usize => Ok(AAEnum::FrontLeftNorth),
            x if x == AAEnum::FrontRightNorth as usize => Ok(AAEnum::FrontRightNorth),
            x if x == AAEnum::FrontNorth as usize => Ok(AAEnum::FrontNorth),
            x if x == AAEnum::FrontWest as usize => Ok(AAEnum::FrontWest),
            x if x == AAEnum::FrontEast as usize => Ok(AAEnum::FrontEast),
            x if x == AAEnum::LeftNorth as usize => Ok(AAEnum::LeftNorth),
            x if x == AAEnum::RightNorth as usize => Ok(AAEnum::RightNorth),
            x if x == AAEnum::North as usize => Ok(AAEnum::North),
            x if x == AAEnum::West as usize => Ok(AAEnum::West),
            x if x == AAEnum::East as usize => Ok(AAEnum::East),
            x if x == AAEnum::None as usize => Ok(AAEnum::None),
            _ => Err(()),
        }
    }
}

struct Resource {
    all: [String; 8],
    front_left_north: [String; 8],
    front_right_north: [String; 8],
    front_north: [String; 8],
    front_west: [String; 8],
    front_east: [String; 8],
    left_north: [String; 8],
    right_north: [String; 8],
    north: [String; 8],
    west: [String; 8],
    east: [String; 8],

    aa_table: [[AAEnum; DirectionEnum::Max as usize]; LocationEnum::Max as usize],
    locations: [[Vec2; LocationEnum::Max as usize]; DirectionEnum::Max as usize],
}

impl Resource {
    pub fn new() -> Self {
        Self {
            all: [
                "L......./\n".to_string(),
                "#L...../#\n".to_string(),
                "#|L. ./|#\n".to_string(),
                "#|#|#|#|#\n".to_string(),
                "#|#| |#|#\n".to_string(),
                "#|/...L|#\n".to_string(),
                "#/.....L#\n".to_string(),
                "/.......L\n".to_string(),
            ],
            front_left_north: [
                ".........\n".to_string(),
                ".........\n".to_string(),
                ".._......\n".to_string(),
                ".|#|.....\n".to_string(),
                ".|_|.....\n".to_string(),
                ".........\n".to_string(),
                ".........\n".to_string(),
                ".........\n".to_string(),
            ],
            front_right_north: [
                ".........\n".to_string(),
                ".........\n".to_string(),
                "......_..\n".to_string(),
                ".....|#|.\n".to_string(),
                ".....|_|.\n".to_string(),
                ".........\n".to_string(),
                ".........\n".to_string(),
                ".........\n".to_string(),
            ],
            front_north: [
                ".........\n".to_string(),
                ".........\n".to_string(),
                "...._....\n".to_string(),
                "...|#|...\n".to_string(),
                "...|_|...\n".to_string(),
                ".........\n".to_string(),
                ".........\n".to_string(),
                ".........\n".to_string(),
            ],
            front_west: [
                ".........\n".to_string(),
                ".........\n".to_string(),
                ".|L......\n".to_string(),
                ".|#|.....\n".to_string(),
                ".|#|.....\n".to_string(),
                ".|/......\n".to_string(),
                ".........\n".to_string(),
                ".........\n".to_string(),
            ],
            front_east: [
                ".........\n".to_string(),
                ".........\n".to_string(),
                "....../|.\n".to_string(),
                ".....|#|.\n".to_string(),
                ".....|#|.\n".to_string(),
                "......L|.\n".to_string(),
                ".........\n".to_string(),
                ".........\n".to_string(),
            ],
            left_north: [
                ".........\n".to_string(),
                "_........\n".to_string(),
                "#|.......\n".to_string(),
                "#|.......\n".to_string(),
                "#|.......\n".to_string(),
                "_|.......\n".to_string(),
                ".........\n".to_string(),
                ".........\n".to_string(),
            ],
            right_north: [
                ".........\n".to_string(),
                "........_\n".to_string(),
                ".......|#\n".to_string(),
                ".......|#\n".to_string(),
                ".......|#\n".to_string(),
                ".......|_\n".to_string(),
                ".........\n".to_string(),
                ".........\n".to_string(),
            ],
            north: [
                ".........\n".to_string(),
                ".._____..\n".to_string(),
                ".|#####|.\n".to_string(),
                ".|#####|.\n".to_string(),
                ".|#####|.\n".to_string(),
                ".|_____|.\n".to_string(),
                ".........\n".to_string(),
                ".........\n".to_string(),
            ],
            west: [
                "L........\n".to_string(),
                "#L.......\n".to_string(),
                "#|.......\n".to_string(),
                "#|.......\n".to_string(),
                "#|.......\n".to_string(),
                "#|.......\n".to_string(),
                "#/.......\n".to_string(),
                "/........\n".to_string(),
            ],
            east: [
                "......../\n".to_string(),
                "......./#\n".to_string(),
                ".......|#\n".to_string(),
                ".......|#\n".to_string(),
                ".......|#\n".to_string(),
                ".......|#\n".to_string(),
                ".......L#\n".to_string(),
                "........L\n".to_string(),
            ],

            aa_table: [
                // location_front_left
                [
                    AAEnum::FrontLeftNorth,
                    AAEnum::None,
                    AAEnum::None,
                    AAEnum::None,
                ],
                // location_front_right
                [
                    AAEnum::FrontRightNorth,
                    AAEnum::None,
                    AAEnum::None,
                    AAEnum::None,
                ],
                // location_front
                [
                    AAEnum::FrontNorth,
                    AAEnum::FrontWest,
                    AAEnum::None,
                    AAEnum::FrontEast,
                ],
                // location_left
                [AAEnum::LeftNorth, AAEnum::None, AAEnum::None, AAEnum::None],
                // location_right
                [AAEnum::RightNorth, AAEnum::None, AAEnum::None, AAEnum::None],
                // location_center
                [AAEnum::North, AAEnum::West, AAEnum::None, AAEnum::East],
            ],
            locations: [
                // direction_north
                [
                    Vec2::new(-1, -1),
                    Vec2::new(1, -1),
                    Vec2::new(0, -1),
                    Vec2::new(-1, 0),
                    Vec2::new(1, 0),
                    Vec2::new(0, 0),
                ],
                // direction_west
                [
                    Vec2::new(-1, 1),
                    Vec2::new(-1, -1),
                    Vec2::new(-1, 0),
                    Vec2::new(0, 1),
                    Vec2::new(0, -1),
                    Vec2::new(0, 0),
                ],
                // direction_south
                [
                    Vec2::new(1, 1),
                    Vec2::new(-1, 1),
                    Vec2::new(0, 1),
                    Vec2::new(1, 0),
                    Vec2::new(-1, 0),
                    Vec2::new(0, 0),
                ],
                // direction_east
                [
                    Vec2::new(1, -1),
                    Vec2::new(1, 1),
                    Vec2::new(1, 0),
                    Vec2::new(0, -1),
                    Vec2::new(0, 1),
                    Vec2::new(0, 0),
                ],
            ],
        }
    }
}
struct Context {
    maze: [Tile; MAZE_HEIGHT * MAZE_WIDTH],
    directions: [Vec2; DirectionEnum::Max as usize],
    player: Character,
    resource: Resource,
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
            resource: Resource::new(),
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
                } else if x == GOAL_X && y == GOAL_Y {
                    floor_aa = 'G';
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
    pub fn draw_3d(&self) {
        const SCREEN_WIDTH: usize = 9;
        const SCREEN_HEIGHT: usize = 8;
        let mut screen = [
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];

        for i in 0..LocationEnum::Max as usize {
            let position = self
                .player
                .position
                .add_new(&self.resource.locations[self.player.direction as usize][i]);

            if !position.is_inside_maze() {
                continue;
            }

            for j in 0..DirectionEnum::Max as usize {
                let direction = (DirectionEnum::Max as usize + j - self.player.direction as usize)
                    % DirectionEnum::Max as usize;

                if !self.maze[position.y as usize * MAZE_WIDTH + position.x as usize].walls[j] {
                    continue;
                }

                if self.resource.aa_table[i][direction] == AAEnum::None {
                    continue;
                }

                let aa_index = self.resource.aa_table[i][direction];
                let aa = match aa_index {
                    AAEnum::All => &self.resource.all,
                    AAEnum::FrontLeftNorth => &self.resource.front_left_north,
                    AAEnum::FrontRightNorth => &self.resource.front_right_north,
                    AAEnum::FrontNorth => &self.resource.front_north,
                    AAEnum::FrontWest => &self.resource.front_west,
                    AAEnum::FrontEast => &self.resource.front_east,
                    AAEnum::LeftNorth => &self.resource.left_north,
                    AAEnum::RightNorth => &self.resource.right_north,
                    AAEnum::North => &self.resource.north,
                    AAEnum::West => &self.resource.west,
                    AAEnum::East => &self.resource.east,
                    AAEnum::None => unreachable!(),
                };

                for k in 0..SCREEN_HEIGHT {
                    for l in 0..SCREEN_WIDTH {
                        let ch = aa[k].chars().nth(l).unwrap();
                        if ch != '.' {
                            screen[k][l] = aa[k].chars().nth(l).unwrap();
                        }
                    }
                }
            }
        }

        for k in 0..SCREEN_HEIGHT {
            for l in 0..SCREEN_WIDTH {
                match screen[k][l] {
                    ' ' => print!(" "),
                    '#' => print!(" "),
                    '_' => print!("_"),
                    '|' => print!("|"),
                    '/' => print!("/"),
                    'L' => print!("\\"),
                    _ => print!("{}", screen[k][l]),
                }
                // screen[k][l] = aa[k].chars().nth(l).unwrap();
            }
            println!();
        }
    }
}

fn main() {
    let mut ctx = Context::new();

    ctx.init();

    loop {
        clearscreen::clear().unwrap();

        ctx.draw_3d();

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

                        if ctx.player.position.x as usize == GOAL_X
                            && ctx.player.position.y as usize == GOAL_Y
                        {}
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
