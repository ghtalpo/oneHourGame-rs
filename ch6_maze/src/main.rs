use getch_rs::{Getch, Key};

const MAZE_WIDTH: usize = 8;
const MAZE_HEIGHT: usize = 8;

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

struct Context {
    maze: [Tile; MAZE_HEIGHT * MAZE_WIDTH],
    g: Getch,
}

impl Context {
    pub fn new() -> Self {
        Self {
            maze: [Tile::new(); MAZE_HEIGHT * MAZE_WIDTH],
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
