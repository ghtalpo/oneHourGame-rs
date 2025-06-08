const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 18;

const BLOCK_WIDTH: usize = 4;
const BLOCK_HEIGHT: usize = 4;

enum BlockEnum {
    None = 0,
    Hard = 1,
    Max = 2,
}

impl TryFrom<usize> for BlockEnum {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == BlockEnum::None as usize => Ok(BlockEnum::None),
            x if x == BlockEnum::Hard as usize => Ok(BlockEnum::Hard),
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

struct BlockShape {
    size: isize,
    pattern: [u8; BLOCK_HEIGHT * BLOCK_WIDTH],
}

struct Context {
    field: [u8; FIELD_HEIGHT * FIELD_WIDTH],
    default_field: [u8; FIELD_HEIGHT * FIELD_WIDTH],
    block_shapes: [BlockShape; BlockShapeEnum::Max as usize],
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
        }
    }
    pub fn init(&mut self) {
        self.field.clone_from_slice(&self.default_field);

        self.draw_screen();
    }
    pub fn draw_screen(&self) {
        let mut screen = [0; FIELD_HEIGHT * FIELD_WIDTH];
        screen.clone_from(&self.field);

        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                match BlockEnum::try_from(read_byte(&screen, x, y) as usize).unwrap() {
                    BlockEnum::None => {
                        print!(" ");
                    }
                    BlockEnum::Hard => {
                        print!("+");
                    }
                    _ => {}
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut ctx = Context::new();
    ctx.init();
    // loop {}
}
