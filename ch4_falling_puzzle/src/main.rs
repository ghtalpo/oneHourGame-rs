use getch_rs::{Getch, Key};
use rand::{Rng, rngs::ThreadRng, seq::IndexedRandom};

// [2]상수를 정의하는 곳

const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 18;

const BLOCK_WIDTH_MAX: usize = 4;
const BLOCK_HEIGHT_MAX: usize = 4;

// [3-1]블록의 종류를 정의한다
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

// [3-2]낙하 블록의 종류를 정의한다
enum BlockShapeEnum {
    I = 0,
    L = 1,
    Max = 2,
}

fn read_byte(data: &[u8], x: usize, y: usize) -> u8 {
    data[y * FIELD_WIDTH + x]
}

// [4-1]낙하 블록 형태의 구조체를 선언한다
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

// [4-2]낙하 블록의 구조체를 선언한다
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
        // [5-1]낙하 블록의 형태를 선언한다
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
        // [5-3]필드의 초기 상태를 선언한다
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
        Self {
            field: [0; FIELD_HEIGHT * FIELD_WIDTH],
            default_field,
            block_shapes,
            block: Block::new(),
            g: Getch::new(),
            rng: rand::rng(),
        }
    }

    // [6-1]낙하 블록과 필드의 충돌 판정을 실시하는 함수를 선언한다
    fn block_intersect_field(&self) -> bool {
        for y in 0..self.block.shape.size {
            for x in 0..self.block.shape.size {
                // [6-1-3]대상 칸에 블록이 있는지 여부를 판정한다
                if self.block.shape.pattern[y * BLOCK_WIDTH_MAX + x] > 0 {
                    let global_x = self.block.x + x as isize;
                    let global_y = self.block.y + y as isize;

                    // [6-1-6]블록과 필드의 충돌 판정을 실시한다
                    if global_x < 0
                        || global_x >= FIELD_WIDTH as isize
                        || global_y < 0
                        || global_y >= FIELD_HEIGHT as isize
                        // 필드 위에 블록이 있는지 여부
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

    // [6-2]채워진 행의 블록을 삭제하는 함수를 선언한다
    fn erase_line(&mut self) {
        for y in 0..FIELD_HEIGHT {
            let mut completed = true;
            for x in 0..FIELD_WIDTH {
                // [6-2-4]대상 칸에 블록이 있는지 여부를 판정한다
                if self.field[y * FIELD_WIDTH + x] == BlockEnum::None as u8 {
                    completed = false;
                    break; // [6-2-6]그 행의 체크를 빠져나간다
                }
            }

            // [6-2-7]그 행이 채워졌는지 여부를 판정한다
            if completed {
                for x in 0..FIELD_WIDTH {
                    if self.field[y * FIELD_WIDTH + x] == BlockEnum::Soft as u8 {
                        // [6-2-10]대상 칸의 블록을 지운다
                        self.field[y * FIELD_WIDTH + x] = BlockEnum::None as u8;
                    }
                }

                for x in 0..FIELD_WIDTH {
                    for y2 in (0..=y).rev() {
                        // [6-2-13]지울 수 없는 블록을 찾으면 반복을 빠져나간다
                        if self.field[y2 * FIELD_WIDTH + x] == BlockEnum::Hard as u8 {
                            break;
                        }

                        // [6-2-14]맨 앞의 행인지 여부를 판정한다
                        if y2 == 0 {
                            // [6-2-15]블록을 지운다
                            self.field[y2 * FIELD_WIDTH + x] = BlockEnum::None as u8;
                        } else {
                            // [6-2-17]위 칸이 지울 수 없는 블록이 아닌지 여부를 판정하다
                            if self.field[(y2 - 1) * FIELD_WIDTH + x] != BlockEnum::Hard as u8 {
                                // [6-2-18]위 칸을 아래 칸으로 복사한다
                                self.field[y2 * FIELD_WIDTH + x] =
                                    self.field[(y2 - 1) * FIELD_WIDTH + x];
                            }
                        }
                    }
                }
            }
        }
    }

    // [6-3]화면을 그리는 함수를 선언한다
    pub fn draw_screen(&self) {
        let mut screen = [0; FIELD_HEIGHT * FIELD_WIDTH];

        // [6-3-2]필드를 화면 버퍼에 복사한다
        screen.clone_from(&self.field);

        for y in 0..BLOCK_HEIGHT_MAX {
            for x in 0..BLOCK_WIDTH_MAX {
                // [6-3-5]블록이 있는지 여부를 판정한다
                if self.block.shape.pattern[y * BLOCK_WIDTH_MAX + x] > 0 {
                    // [6-3-6]화면 버퍼에 낙하 블록을 써넣는다
                    screen
                        [(self.block.y as usize + y) * FIELD_WIDTH + (self.block.x as usize + x)] =
                        BlockEnum::Fall as u8;
                }
            }
        }

        clearscreen::clear().unwrap();

        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                // [6-3-10]블록의 종류로 분기한다
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

    // [6-4]낙하 블록을 회전시키는 함수를 선언한다
    pub fn rotate_block(&mut self) {
        let mut rotated_block = self.block.clone();

        for y in 0..self.block.shape.size {
            for x in 0..self.block.shape.size {
                // [6-4-4]회전 후의 블록 형태를 작성한다
                rotated_block.shape.pattern
                    [(self.block.shape.size - 1 - x) * BLOCK_WIDTH_MAX + y] =
                    self.block.shape.pattern[y * BLOCK_WIDTH_MAX + x];
            }
        }

        // [6-4-5]회전 후의 블록을 적용한다
        self.block = rotated_block.clone();
    }

    // [6-5]낙하 블록을 초기화하는 함수를 선언한다
    pub fn init_block(&mut self) {
        // [6-5-1]낙하 블록의 형태를 랜덤으로 설정한다
        self.block.shape = *self.block_shapes.choose(&mut self.rng).unwrap();

        // [6-5-2]낙하 블록의 열을 중심으로 한다
        self.block.x = (FIELD_WIDTH / 2 - self.block.shape.size / 2) as isize;
        self.block.y = 0;

        // [6-5-4]낙하 블록을 회전시키는 횟수를 선언한다
        let rotate_count = self.rng.random::<u8>() % 4;

        for _ in 0..rotate_count {
            // [6-5-6]낙하 블록을 회전시킨다
            self.rotate_block();
        }
    }

    // [6-6]게임을 초기화하는 함수를 선언한다
    pub fn init(&mut self) {
        // [6-6-1]필드에 초기 상태를 복사한다
        self.field.clone_from_slice(&self.default_field);

        self.init_block();

        self.draw_screen();
    }

    // [6-7]낙하 블록을 떨어뜨리는 함수를 선언한다
    fn fall_block(&mut self) {
        // [6-7-1]블록 이동 전의 상태를 선언한다
        let last_block = self.block.clone();

        self.block.y += 1; // [6-7-2]블록을 떨어뜨린다

        // [6-7-3]블록과 필드가 겹쳤는지 여부를 판정한다
        if self.block_intersect_field() {
            // [6-7-4]낙하 블록을 이동 전의 상태로 되돌린다
            self.block = last_block.clone();

            for y in 0..self.block.shape.size {
                for x in 0..self.block.shape.size {
                    // [6-7-7]블록이 있는 칸인지 여부를 판정한다
                    if self.block.shape.pattern[y as usize * BLOCK_WIDTH_MAX + x as usize] > 0 {
                        // [6-7-8]필드에 지울 수 있는 블록을 써넣는다
                        self.field[(self.block.y as usize + y) * FIELD_WIDTH
                            + (self.block.x as usize + x)] = BlockEnum::Soft as u8;
                    }
                }
            }

            self.erase_line();

            self.init_block();

            if self.block_intersect_field() {
                self.init(); // [6-7-12]게임을 초기화한다
            }
        }

        self.draw_screen();
    }
}

// [6-8]프로그램 실행의 시작점을 선언한다
fn main() {
    let mut ctx = Context::new();
    ctx.init();

    // [6-8-4]메인 루프
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
