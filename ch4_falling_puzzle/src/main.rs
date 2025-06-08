use getch_rs::{Getch, Key};
use rand::{Rng, rngs::ThreadRng, seq::IndexedRandom};

const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 18;

const BLOCK_WIDTH_MAX: usize = 4;
const BLOCK_HEIGHT_MAX: usize = 4;

enum BlockEnum {
    None = 0,
    Hard = 1,
    Soft = 2,
    Fall = 3,
    Max = 4,
}

impl TryFrom<usize> for BlockEnum {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == BlockEnum::None as usize => Ok(BlockEnum::None),
            x if x == BlockEnum::Hard as usize => Ok(BlockEnum::Hard),
            x if x == BlockEnum::Fall as usize => Ok(BlockEnum::Fall),
            x if x == BlockEnum::Soft as usize => Ok(BlockEnum::Soft),
            _ => Err(()),
        }
    }
}

enum BlockShapeEnum {
    I = 0,
    L = 1,
    Max = 2,
}

fn read_byte(data: &[u8], x: usize, y: usize) -> u8 {
    data[y * FIELD_WIDTH + x]
}

#[derive(Copy, Clone)]
struct BlockShape {
    size: usize,
    pattern: [u8; BLOCK_HEIGHT_MAX * BLOCK_WIDTH_MAX],
}

impl BlockShape {
    pub fn new() -> Self {
        Self {
            size: 0,
            pattern: [0; BLOCK_HEIGHT_MAX * BLOCK_WIDTH_MAX],
        }
    }
}

#[derive(Copy, Clone)]
struct Block {
    x: isize,
    y: isize,
    shape: BlockShape,
}

impl Block {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            shape: BlockShape::new(),
        }
    }
}

struct Context {
    field: [u8; FIELD_HEIGHT * FIELD_WIDTH],
    default_field: [u8; FIELD_HEIGHT * FIELD_WIDTH],
    block_shapes: [BlockShape; BlockShapeEnum::Max as usize],
    block: Block,
    g: Getch,
    rng: ThreadRng,
}

impl Context {
    pub fn new() -> Self {
        let default_field = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0,
            0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        let block_shapes = [
            BlockShape {
                size: 3,
                pattern: [0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            },
            BlockShape {
                size: 3,
                pattern: [0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            },
        ];
        Self {
            field: [0; FIELD_HEIGHT * FIELD_WIDTH],
            default_field,
            block_shapes,
            block: Block::new(),
            g: Getch::new(),
            rng: rand::rng(),
        }
    }
    pub fn init(&mut self) {
        self.field.clone_from_slice(&self.default_field);

        self.init_block();

        self.draw_screen();
    }
    pub fn draw_screen(&self) {
        let mut screen = [0; FIELD_HEIGHT * FIELD_WIDTH];
        screen.clone_from(&self.field);

        for y in 0..BLOCK_HEIGHT_MAX {
            for x in 0..BLOCK_WIDTH_MAX {
                if self.block.shape.pattern[y * BLOCK_WIDTH_MAX + x] > 0 {
                    screen
                        [(self.block.y as usize + y) * FIELD_WIDTH + (self.block.x as usize + x)] =
                        BlockEnum::Fall as u8;
                }
            }
        }

        clearscreen::clear().unwrap();

        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                match BlockEnum::try_from(read_byte(&screen, x, y) as usize).unwrap() {
                    BlockEnum::None => {
                        print!(" ");
                    }
                    BlockEnum::Hard => {
                        print!("+");
                    }
                    BlockEnum::Soft => {
                        print!("◆");
                    }
                    BlockEnum::Fall => {
                        print!("◇");
                    }
                    _ => {}
                }
            }
            println!();
        }
    }
    pub fn init_block(&mut self) {
        self.block.shape = *self.block_shapes.choose(&mut self.rng).unwrap();
        self.block.x = (FIELD_WIDTH / 2 - self.block.shape.size / 2) as isize;
        self.block.y = 0;

        let rotate_count = self.rng.random::<u8>() % 4;
        for _ in 0..rotate_count {
            self.rotate_block();
        }
    }
    pub fn rotate_block(&mut self) {
        let mut rotated_block = self.block.clone();

        for y in 0..self.block.shape.size {
            for x in 0..self.block.shape.size {
                rotated_block.shape.pattern
                    [(self.block.shape.size - 1 - x) * BLOCK_WIDTH_MAX + y] =
                    self.block.shape.pattern[y * BLOCK_WIDTH_MAX + x];
            }
        }
        self.block = rotated_block.clone();
    }
    fn fall_block(&mut self) {
        let last_block = self.block.clone();

        self.block.y += 1;

        if self.block_intersect_field() {
            self.block = last_block.clone();

            for y in 0..self.block.shape.size {
                for x in 0..self.block.shape.size {
                    if self.block.shape.pattern[y as usize * BLOCK_WIDTH_MAX + x as usize] > 0 {
                        self.field[(self.block.y as usize + y) * FIELD_WIDTH
                            + (self.block.x as usize + x)] = BlockEnum::Soft as u8;
                    }
                }
            }

            self.erase_line();

            self.init_block();

            if self.block_intersect_field() {
                self.init();
            }
        }

        self.draw_screen();
    }
    fn block_intersect_field(&self) -> bool {
        for y in 0..self.block.shape.size {
            for x in 0..self.block.shape.size {
                if self.block.shape.pattern[y * BLOCK_WIDTH_MAX + x] > 0 {
                    let global_x = self.block.x + x as isize;
                    let global_y = self.block.y + y as isize;
                    if global_x < 0
                        || global_x >= FIELD_WIDTH as isize
                        || global_y < 0
                        || global_y >= FIELD_HEIGHT as isize
                        || self.field[global_y as usize * FIELD_WIDTH + global_x as usize]
                            != BlockEnum::None as u8
                    {
                        return true;
                    }
                }
            }
        }
        return false;
    }
    fn erase_line(&mut self) {
        for y in 0..FIELD_HEIGHT {
            let mut completed = true;
            for x in 0..FIELD_WIDTH {
                if self.field[y * FIELD_WIDTH + x] == BlockEnum::None as u8 {
                    completed = false;
                    break;
                }
            }

            if completed {
                for x in 0..FIELD_WIDTH {
                    if self.field[y * FIELD_WIDTH + x] == BlockEnum::Soft as u8 {
                        self.field[y * FIELD_WIDTH + x] = BlockEnum::None as u8;
                    }
                }

                for x in 0..FIELD_WIDTH {
                    for y2 in (0..=y).rev() {
                        if self.field[y2 * FIELD_WIDTH + x] == BlockEnum::Hard as u8 {
                            break;
                        }

                        if y2 == 0 {
                            self.field[y2 * FIELD_WIDTH + x] = BlockEnum::None as u8;
                        } else {
                            self.field[y2 * FIELD_WIDTH + x] =
                                self.field[(y2 - 1) * FIELD_WIDTH + x];
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut ctx = Context::new();
    ctx.init();
    loop {
        // FIXME: _kbhit 류의 함수가 필요함
        let last_block = ctx.block.clone();
        match ctx.g.getch() {
            Ok(Key::Char('w')) => {}
            Ok(Key::Char('s')) => {
                ctx.block.y += 1;
            }
            Ok(Key::Char('a')) => {
                ctx.block.x -= 1;
            }
            Ok(Key::Char('d')) => {
                ctx.block.x += 1;
            }
            Ok(Key::Esc) => {
                std::process::exit(0);
            }
            _ => {
                ctx.rotate_block();
            }
        }

        if ctx.block_intersect_field() {
            ctx.block = last_block.clone();
        } else {
            ctx.draw_screen();
        }

        // FIXME:
        ctx.fall_block();
    }
}
