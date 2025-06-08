const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

#[derive(Clone, Copy)]
enum TurnEnum {
    Black = 0,
    White = 1,
    None = 2,
    Max = 3,
}

struct Context {
    board: Vec<TurnEnum>,
    disk_aa: [String; TurnEnum::Max as usize],
}

impl Context {
    pub fn new() -> Self {
        Self {
            board: vec![TurnEnum::None; BOARD_HEIGHT * BOARD_WIDTH],
            disk_aa: ["⬤".to_string(), "◯".to_string(), "·".to_string()],
        }
    }
    pub fn init(&mut self) {
        self.draw_screen();
    }
    pub fn draw_screen(&self) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                print!("{}", self.disk_aa[self.board[y * BOARD_WIDTH + x] as usize]);
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
