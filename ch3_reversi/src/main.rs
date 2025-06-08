use getch_rs::{Getch, Key};

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

#[derive(Clone, Copy, PartialEq)]
enum TurnEnum {
    Black = 0,
    White = 1,
    None = 2,
    Max = 3,
}

#[derive(PartialEq)]
enum DirectionEnum {
    Up = 0,
    UpLeft = 1,
    Left = 2,
    DownLeft = 3,
    Down = 4,
    DownRight = 5,
    Right = 6,
    UpRight = 7,
    Max = 8,
}

#[derive(Clone, Copy, Default)]
struct Vec2 {
    x: i8,
    y: i8,
}

impl Vec2 {
    pub fn add(&mut self, other: &Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}
struct Context {
    board: Vec<TurnEnum>,
    disk_aa: [String; TurnEnum::Max as usize],
    cursor_position: Vec2,
    g: Getch,
    turn: TurnEnum,
    turn_names: [String; TurnEnum::Max as usize],
    directions: [Vec2; DirectionEnum::Max as usize],
}

impl Context {
    pub fn new() -> Self {
        Self {
            board: vec![TurnEnum::None; BOARD_HEIGHT * BOARD_WIDTH],
            disk_aa: ["●".to_string(), "○".to_string(), "·".to_string()],
            cursor_position: Vec2::default(),
            g: Getch::new(),
            turn: TurnEnum::Black,
            turn_names: ["검은 돌".to_string(), "흰 돌".to_string(), "·".to_string()],
            directions: [
                Vec2 { x: 0, y: -1 },
                Vec2 { x: -1, y: -1 },
                Vec2 { x: -1, y: 0 },
                Vec2 { x: -1, y: 1 },
                Vec2 { x: 0, y: 1 },
                Vec2 { x: 1, y: 1 },
                Vec2 { x: 1, y: 0 },
                Vec2 { x: 1, y: 1 },
            ],
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

        if self.turn != TurnEnum::None {
            println!("{}의 턴입니다.", self.turn_names[self.turn as usize]);
        } else {
            let black_count = self.get_disk_count(TurnEnum::Black);

            let white_count = self.get_disk_count(TurnEnum::White);

            let winner = if black_count > white_count {
                TurnEnum::Black
            } else if white_count > black_count {
                TurnEnum::White
            } else {
                TurnEnum::None
            };
            println!(
                "{}{} - {}{}",
                self.turn_names[TurnEnum::Black as usize],
                black_count,
                self.turn_names[TurnEnum::White as usize],
                white_count,
            );

            if winner == TurnEnum::None {
                println!("무승부");
            } else {
                println!("{}의 승리", self.turn_names[winner as usize]);
            }
        }
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
                _ => {
                    if self.check_can_place(self.turn, self.cursor_position, true) {
                        return self.cursor_position;
                    } else {
                        println!("놓을 수 없는 곳입니다.");
                        let _ = self.g.getch();
                    }
                }
            }
            self.cursor_position.x =
                (BOARD_WIDTH as i8 + self.cursor_position.x) % (BOARD_WIDTH as i8);
            self.cursor_position.y =
                (BOARD_HEIGHT as i8 + self.cursor_position.y) % (BOARD_HEIGHT as i8);
        }
    }
    pub fn check_can_place(&mut self, color: TurnEnum, position: Vec2, turn_over: bool) -> bool {
        let mut can_place = false;
        if self.board[position.y as usize * BOARD_WIDTH + position.x as usize] != TurnEnum::None {
            return false;
        }
        for i in 0..DirectionEnum::Max as usize {
            let mut current_position = position;

            current_position.add(&self.directions[i]);
            if current_position.x < 0
                || current_position.x >= BOARD_WIDTH as i8
                || current_position.y < 0
                || current_position.y >= BOARD_HEIGHT as i8
            {
                continue;
            }

            let opponent = if color == TurnEnum::Black {
                TurnEnum::White
            } else {
                TurnEnum::Black
            };

            if self.board[current_position.y as usize * BOARD_WIDTH + current_position.x as usize]
                != opponent
            {
                continue;
            }
            loop {
                current_position.add(&self.directions[i]);

                if current_position.x < 0
                    || current_position.x >= BOARD_WIDTH as i8
                    || current_position.y < 0
                    || current_position.y >= BOARD_HEIGHT as i8
                {
                    break;
                }
                if self.board
                    [current_position.y as usize * BOARD_WIDTH + current_position.x as usize]
                    == TurnEnum::None
                {
                    break;
                }
                if self.board
                    [current_position.y as usize * BOARD_WIDTH + current_position.x as usize]
                    == color
                {
                    can_place = true;
                    if turn_over {
                        let mut reverse_position = position;
                        reverse_position.add(&self.directions[i]);
                        loop {
                            self.board[reverse_position.y as usize * BOARD_WIDTH
                                + reverse_position.x as usize] = color;

                            reverse_position.add(&self.directions[i]);
                            if reverse_position.x < 0
                                || reverse_position.x >= BOARD_WIDTH as i8
                                || reverse_position.y < 0
                                || reverse_position.y >= BOARD_HEIGHT as i8
                            {
                                break;
                            }

                            if self.board[reverse_position.y as usize * BOARD_WIDTH
                                + reverse_position.x as usize]
                                == color
                            {
                                break;
                            }
                        }
                    }
                }
            }
        }
        return can_place;
    }
    fn check_can_place_all(&mut self, color: TurnEnum) -> bool {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let position = Vec2 {
                    x: x as i8,
                    y: y as i8,
                };

                if self.check_can_place(color, position, false) {
                    return true;
                }
            }
        }
        return false;
    }
    fn get_disk_count(&self, color: TurnEnum) -> u32 {
        let mut count = 0;
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.board[y as usize * BOARD_WIDTH + x as usize] == color {
                    count += 1;
                }
            }
        }
        return count;
    }
}

fn main() {
    let mut ctx = Context::new();
    ctx.init();
    loop {
        if !ctx.check_can_place_all(ctx.turn) {
            ctx.turn = if ctx.turn == TurnEnum::Black {
                TurnEnum::White
            } else {
                TurnEnum::Black
            };
            if !ctx.check_can_place_all(ctx.turn) {
                ctx.turn = TurnEnum::None;

                ctx.draw_screen();

                let _ = ctx.g.getch();
            } else {
                continue;
            }
        }
        let mut place_position = Vec2::default();

        place_position = ctx.input_position();

        ctx.board[place_position.y as usize * BOARD_WIDTH + place_position.x as usize] = ctx.turn;
        ctx.turn = if ctx.turn == TurnEnum::Black {
            TurnEnum::White
        } else {
            TurnEnum::Black
        };
    }
}
