const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

#[derive(Clone, Copy)]
enum TurnEnum {
    Black = 0,
    White = 1,
    None = 2,
    Max = 3,
}

#[derive(Clone, Copy)]
struct Vec2 {
    x: usize,
    y: usize,
}

struct Context {
    board: Vec<TurnEnum>,
    disk_aa: [String; TurnEnum::Max as usize],
    cursor_position: Vec2,
}

impl Context {
    pub fn new() -> Self {
        Self {
            board: vec![TurnEnum::None; BOARD_HEIGHT * BOARD_WIDTH],
            disk_aa: ["●".to_string(), "○".to_string(), "·".to_string()],
            cursor_position: Vec2 { x: 0, y: 0 },
        }
    }
    pub fn init(&mut self) {
        self.board[4 * BOARD_WIDTH + 3] = TurnEnum::Black;
        self.board[3 * BOARD_WIDTH + 4] = TurnEnum::Black;
        self.board[3 * BOARD_WIDTH + 3] = TurnEnum::White;
        self.board[4 * BOARD_WIDTH + 4] = TurnEnum::White;
        self.draw_screen();
    }
    pub fn draw_screen(&self) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                print!("{}", self.disk_aa[self.board[y * BOARD_WIDTH + x] as usize]);
            }
            if y == self.cursor_position.y {
                print!("←");
            }
            println!();
        }
        for x in 0..BOARD_WIDTH {
            if x == self.cursor_position.x {
                print!("↑");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let mut ctx = Context::new();
    ctx.init();
    // loop {}
}
