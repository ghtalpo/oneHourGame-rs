use getch_rs::{Getch, Key};

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

struct Context {
    maze: [Tile; MAZE_HEIGHT * MAZE_WIDTH],
    directions: [Vec2; DirectionEnum::Max as usize],
    g: Getch,
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
            directions,
            g: Getch::new(),
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
                let floor_aa = ' ';
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
    }
    fn generate_map(&mut self) {
        for y in 0..MAZE_HEIGHT {
            for x in 0..MAZE_WIDTH {
                for i in 0..DirectionEnum::Max as usize {
                    self.maze[y * MAZE_WIDTH + x].walls[i] = true;
                }
            }
        }
    }
    fn dig_wall(&mut self, position: Vec2, direction: DirectionEnum) {
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
}

fn main() {
    let mut ctx = Context::new();

    ctx.init();

    loop {
        clearscreen::clear().unwrap();

        ctx.draw_map();

        match ctx.g.getch() {
            Ok(Key::Esc) => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}
