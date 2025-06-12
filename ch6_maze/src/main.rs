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
    wall: [bool; DirectionEnum::Max as usize],
}

impl Tile {
    pub fn new() -> Self {
        Self {
            wall: [false; DirectionEnum::Max as usize],
        }
    }
}

struct Context {
    maze: [Tile; MAZE_HEIGHT * MAZE_WIDTH],
}

impl Context {
    pub fn new() -> Self {
        Self {
            maze: [Tile::new(); MAZE_HEIGHT * MAZE_WIDTH],
        }
    }
}

fn main() {
    loop {}
}
