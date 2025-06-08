use getch_rs::{Getch, Key};

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

#[derive(Clone, Copy)]
enum TurnEnum {
    Black = 0,
    White = 1,
    None = 2,
    Max = 3,
}

#[derive(Clone, Copy, Default)]
struct Vec2 {
    x: i8,
    y: i8,
}

struct Context {
    board: Vec<TurnEnum>,
    disk_aa: [String; TurnEnum::Max as usize],
    cursor_position: Vec2,
    g: Getch,
}

impl Context {
    pub fn new() -> Self {
        Self {
            board: vec![TurnEnum::None; BOARD_HEIGHT * BOARD_WIDTH],
            disk_aa: ["●".to_string(), "○".to_string(), "·".to_string()],
            cursor_position: Vec2::default(),
            g: Getch::new(),
        }
    }
    pub fn init(&mut self) {
        self.board[4 * BOARD_WIDTH + 3] = TurnEnum::Black;
        self.board[3 * BOARD_WIDTH + 4] = TurnEnum::Black;
        self.board[3 * BOARD_WIDTH + 3] = TurnEnum::White;
        self.board[4 * BOARD_WIDTH + 4] = TurnEnum::White;

        self.cursor_position = Vec2 { x: 3, y: 3 };

        self.draw_screen();
    }
    pub fn draw_screen(&self) {
        clearscreen::clear().unwrap();

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                print!("{}", self.disk_aa[self.board[y * BOARD_WIDTH + x] as usize]);
            }
            if y == self.cursor_position.y as usize {
                print!("←");
            }
            println!();
        }
        for x in 0..BOARD_WIDTH {
            if x == self.cursor_position.x as usize {
                print!("↑");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    pub fn input_position(&mut self) -> Vec2 {
        loop {
            self.draw_screen();

            match self.g.getch() {
                Ok(Key::Char('w')) => {
                    self.cursor_position.y -= 1;
                }
                Ok(Key::Char('s')) => {
                    self.cursor_position.y += 1;
                }
                Ok(Key::Char('a')) => {
                    self.cursor_position.x -= 1;
                }
                Ok(Key::Char('d')) => {
                    self.cursor_position.x += 1;
                }
                Ok(Key::Esc) => {
                    std::process::exit(0);
                }
                _ => {}
            }
            self.cursor_position.x =
                (BOARD_WIDTH as i8 + self.cursor_position.x) % (BOARD_WIDTH as i8);
            self.cursor_position.y =
                (BOARD_HEIGHT as i8 + self.cursor_position.y) % (BOARD_HEIGHT as i8);
        }
        return Vec2::default();
    }
}

fn main() {
    let mut ctx = Context::new();
    ctx.init();
    loop {
        let mut place_position = Vec2::default();

        place_position = ctx.input_position();
    }
}
