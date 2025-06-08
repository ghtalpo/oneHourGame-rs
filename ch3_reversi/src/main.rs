const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

enum TurnEnum {
    Black = 0,
    White = 1,
    None = 2,
    Max = 3,
}

struct Context {
    board: vec<TurnEnum>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            board: vec![TurnEnum.None; BOARD_HEIGHT * BOARD_WIDTH],
        }
    }
    pub fn init(&mut self) {}
}
fn main() {
    let mut ctx = Context::new();
    ctx.init();
    loop {}
}
