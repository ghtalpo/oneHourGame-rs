use getch_rs::{Getch, Key};
use rand::{rngs::ThreadRng, seq::IndexedRandom};

// [2]상수를 정의하는 곳

const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

// [3-1]턴의 종류를 정의한다
#[derive(Clone, Copy, PartialEq)]
enum TurnEnum {
    Black = 0,
    White = 1,
    None = 2,
    Max = 3,
}

// [3-2]방향의 종류를 정의한다
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

// [3-3]게임 모드의 종류를 정의한다
#[derive(Clone, Copy, PartialEq)]
enum ModeEnum {
    OnePlayer = 0,
    TwoPlayers = 1,
    Watch = 2,
    Max = 3,
}

impl TryFrom<usize> for ModeEnum {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == ModeEnum::OnePlayer as usize => Ok(ModeEnum::OnePlayer),
            x if x == ModeEnum::TwoPlayers as usize => Ok(ModeEnum::TwoPlayers),
            x if x == ModeEnum::Watch as usize => Ok(ModeEnum::Watch),
            _ => Err(()),
        }
    }
}

impl ModeEnum {
    pub fn increase(&mut self) {
        *self = ((*self as usize + 1) % Self::Max as usize)
            .try_into()
            .unwrap();
    }
    pub fn decrease(&mut self) {
        *self = ((*self as usize + Self::Max as usize - 1) % Self::Max as usize)
            .try_into()
            .unwrap();
    }
}

// [4-1]벡터 구조체를 선언한다
#[derive(Clone, Copy, Default)]
struct Vec2 {
    x: i8,
    y: i8,
}

impl Vec2 {
    // [6-1]벡터를 더하는 함수를 선언한다
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
    mode_names: [String; ModeEnum::Max as usize],
    mode: ModeEnum,
    is_player: [bool; TurnEnum::Max as usize],
    rng: ThreadRng,
}

impl Context {
    pub fn new() -> Self {
        Self {
            // [5-1]돌의 아스키아트를 선언한다
            disk_aa: ["●".to_string(), "○".to_string(), "·".to_string()],
            // [5-2]턴의 이름을 선언한다
            turn_names: ["검은 돌".to_string(), "흰 돌".to_string(), "·".to_string()],
            // [5-3]모드의 이름을 선언한다
            mode_names: [
                "1P GAME".to_string(),
                "2P GAME".to_string(),
                "WATCH".to_string(),
            ],
            // [5-4]방향을 선언한다
            directions: [
                Vec2 { x: 0, y: -1 },
                Vec2 { x: -1, y: -1 },
                Vec2 { x: -1, y: 0 },
                Vec2 { x: -1, y: 1 },
                Vec2 { x: 0, y: 1 },
                Vec2 { x: 1, y: 1 },
                Vec2 { x: 1, y: 0 },
                Vec2 { x: 1, y: -1 },
            ],
            // [5-5]모눈판 각 칸의 상태를 선언한다
            board: vec![TurnEnum::None; BOARD_HEIGHT * BOARD_WIDTH],
            // [5-6]커서의 좌표를 선언한다
            cursor_position: Vec2::default(),
            // [5-7]현재의 턴을 선언한다
            turn: TurnEnum::Black,
            // [5-8]현재의 게임 모드를 선언한다
            mode: ModeEnum::Max,
            // [5-9]각 턴이 플레이어인지 여부를 선언한다
            is_player: [false; TurnEnum::Max as usize],
            g: Getch::new(),
            rng: rand::rng(),
        }
    }

    // [6-2]돌을 놓을 수 있는지 여부의 판정, 또는 돌을 뒤집는 함수를 선언한다
    pub fn check_can_place(&mut self, color: TurnEnum, position: Vec2, turn_over: bool) -> bool {
        let mut can_place = false; // [6-2-1]돌을 놓을 수 있는지 여부의 플래그를 선언한다

        // [6-2-2]대상 좌표에 돌이 놓여 있지 않은지 여부를 판정한다
        if self.board[position.y as usize * BOARD_WIDTH + position.x as usize] != TurnEnum::None {
            return false; // [6-2-3]돌이 놓여 있으면 놓을 수 없다는 결과를 반환한다
        }

        // [6-2-4]상대의 돌 색을 선언한다
        let opponent = if color == TurnEnum::Black {
            TurnEnum::White
        } else {
            TurnEnum::Black
        };

        // [6-2-5]모든 방향을 반복한다
        for i in 0..DirectionEnum::Max as usize {
            // [6-2-6]현재 체크 중인 좌표를 선언한다
            let mut current_position = position;

            // [6-2-7]옆의 칸으로 이동한다
            current_position.add(&self.directions[i]);

            // [6-2-7.1]체크하는 칸이 모눈판의 범위 내인지 판정한다
            if current_position.x < 0
                || current_position.x >= BOARD_WIDTH as i8
                || current_position.y < 0
                || current_position.y >= BOARD_HEIGHT as i8
            {
                // [6-2-7.2]대상 방향의 체크를 스킵한다
                continue;
            }

            // [6-2-8]상대의 돌이 아닌지 판정한다
            if self.board[current_position.y as usize * BOARD_WIDTH + current_position.x as usize]
                != opponent
            {
                // [6-2-9]상대의 돌이 아니면 그 방향의 체크를 중지한다
                continue;
            }

            // [6-2-10]무한 루프한다
            loop {
                // [6-2-11]옆 칸으로 이동한다
                current_position.add(&self.directions[i]);

                // [6-2-12]체크하는 칸이 모눈판의 범위 내인지 판정한다
                if current_position.x < 0
                    || current_position.x >= BOARD_WIDTH as i8
                    || current_position.y < 0
                    || current_position.y >= BOARD_HEIGHT as i8
                {
                    // [6-2-13]모눈판 바깥쪽으로 나가면 현재 방향의 체크를 빠져나간다
                    break;
                }

                // [6-2-14]체크하는 칸에 돌이 있는지 여부를 판정한다
                if self.board
                    [current_position.y as usize * BOARD_WIDTH + current_position.x as usize]
                    == TurnEnum::None
                {
                    break; // [6-2-15]돌이 없으면 현재 방향의 체크를 빠져나간다
                }

                // [6-2-16]체크하는 칸에 자신의 돌이 있으면
                if self.board
                    [current_position.y as usize * BOARD_WIDTH + current_position.x as usize]
                    == color
                {
                    // [6-2-17]돌을 놓을 수 있는 것이 확정된다
                    can_place = true;

                    // [6-2-18]뒤집기 플래그가 설정되어 있는지 여부를 판정한다
                    if turn_over {
                        // [6-2-19]뒤집는 좌표를 선언한다
                        let mut reverse_position = position;

                        // [6-2-20]옆 칸으로 이동한다
                        reverse_position.add(&self.directions[i]);

                        // [6-2-21]현재 턴의 돌을 찾을 때까지 반복한다
                        loop {
                            // [6-2-22]상대의 돌을 뒤집는다
                            self.board[reverse_position.y as usize * BOARD_WIDTH
                                + reverse_position.x as usize] = color;

                            // [6-2-23]옆 칸으로 이동한다
                            reverse_position.add(&self.directions[i]);

                            // 크래시 막기
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
        can_place// [6-2-24]돌을 놓을 수 있는지 여부를 반환한다
    }

    // [6-3]모눈판 위에 돌을 놓을 수 있는 칸이 있는지 여부를 판정하는 함수를 선언한다
    fn check_can_place_all(&mut self, color: TurnEnum) -> bool {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                // [6-3-3]판정하는 좌표를 선언한다
                let position = Vec2 {
                    x: x as i8,
                    y: y as i8,
                };

                // [6-3-4]대상 좌표에 돌을 놓을 수 있는지 여부를 판정한다
                if self.check_can_place(color, position, false) {
                    return true; // [6-3-5]돌을 놓을 수 있는 칸이 있다는 결과를 반환한다
                }
            }
        }
        false// [6-3-6]돌을 놓을 수 있는 칸이 없다는 결과를 반환한다
    }

    // [6-4]임의의 돌의 개수를 세는 함수를 선언한다
    fn get_disk_count(&self, color: TurnEnum) -> u32 {
        let mut count = 0; // [6-4-1]세는 돌의 개수를 보유하는 변수를 선언한다

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                // [6-4-4]대상 칸에 대상의 돌이 있는지 여부를 판정한다
                if self.board[y * BOARD_WIDTH + x] == color {
                    count += 1; // [6-4-5]돌의 개수를 더한다
                }
            }
        }
        count// [6-4-6]센 돌의 개수를 반환한다
    }

    // [6-5]화면을 그리는 함수를 선언한다
    pub fn draw_screen(&self) {
        clearscreen::clear().unwrap();

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                print!("{}", self.disk_aa[self.board[y * BOARD_WIDTH + x] as usize]);
            }

            // [6-5-5]플레이어의 담당인지 여부를 판정한다
            if self.is_player[self.turn as usize] {
                // [6-5-6]대상 행이 커서와 같은 행인지 여부를 판정한다
                if y == self.cursor_position.y as usize {
                    print!("←");
                }
            }

            println!();
        }

        // [6-5-9]플레이어의 담당인지 여부를 판정한다
        if self.is_player[self.turn as usize] {
            for x in 0..BOARD_WIDTH {
                if x == self.cursor_position.x as usize {
                    print!("↑");
                } else {
                    print!(" ");
                }
            }
        }

        println!();

        // [6-5-15]승부가 났는지 여부를 판정한다
        if self.turn != TurnEnum::None {
            // [6-5-16]턴을 표시한다
            println!("{}의 턴입니다.", self.turn_names[self.turn as usize]);
        } else {
            // [6-5-17]승부가 났다면
            let black_count = self.get_disk_count(TurnEnum::Black);

            let white_count = self.get_disk_count(TurnEnum::White);

            // [6-5-21]승자를 판정한다
            let winner = if black_count > white_count {
                TurnEnum::Black
            } else if white_count > black_count {
                TurnEnum::White
            } else {
                TurnEnum::None
            };

            // [6-5-28]양측 돌의 개수를 표시한다
            println!(
                "{}{} - {}{}",
                self.turn_names[TurnEnum::Black as usize],
                black_count,
                self.turn_names[TurnEnum::White as usize],
                white_count,
            );

            // [6-5-29]무승부인지 여부를 판정한다
            if winner == TurnEnum::None {
                println!("무승부");
            } else {
                // [6-5-31]승부가 났다면
                // [6-5-32]승자를 표시한다
                println!("{}의 승리", self.turn_names[winner as usize]);
            }
        }
    }

    // [6-6]모드 선택 화면의 함수를 선언한다
    fn select_mode(&mut self) {
        self.mode = ModeEnum::OnePlayer; // [6-6-1]게임 모드를 초기화한다

        loop {
            clearscreen::clear().unwrap();

            println!("모드를 선택하세요\n\n");

            for i in 0..ModeEnum::Max as usize {
                // [6-6-7]현재의 모드에는 커서를, 그 밖에는 공백을 그린다
                print!("{}", if i == self.mode as usize { ">" } else { " " });

                println!("{}\n", self.mode_names[i]);
            }

            match self.g.getch() {
                Ok(Key::Char('w')) => {
                    self.mode.decrease(); // [6-6-12]이전 모드로 바꾼다
                }
                Ok(Key::Char('s')) => {
                    self.mode.increase(); // [6-6-14]다음 모드로 바꾼다
                }
                Ok(Key::Esc) => {
                    std::process::exit(0);
                }
                _ => {
                    // [6-6-16]선택된 모드로 분기한다
                    match self.mode {
                        ModeEnum::OnePlayer => {
                            // [6-6-17]AI와 대전하는 모드라면
                            self.is_player[TurnEnum::Black as usize] = true;
                            self.is_player[TurnEnum::White as usize] = false;
                        }
                        ModeEnum::TwoPlayers => {
                            // [6-6-20]사람 간의 대전 모드라면
                            self.is_player[TurnEnum::Black as usize] = true;
                            self.is_player[TurnEnum::White as usize] = true;
                        }
                        ModeEnum::Watch => {
                            // [6-6-22]AI간 대결의 관전 모드라면
                            self.is_player[TurnEnum::Black as usize] = false;
                            self.is_player[TurnEnum::White as usize] = false;
                        }
                        _ => {}
                    }
                    return; // [6-6-24]모드 선택을 빠져나간다
                }
            }
        }
    }

    // [6-7]게임을 초기화하는 함수를 선언한다
    pub fn init(&mut self) {
        // [6-7-3]대상 칸을 돌이 놓여 있지 않은 상태로 한다
        self.board = vec![TurnEnum::None; BOARD_HEIGHT * BOARD_WIDTH];

        // [6-7-4]모눈판 중앙의 오른쪽 위와 왼쪽 아래에 검은 돌을 놓는다
        self.board[4 * BOARD_WIDTH + 3] = TurnEnum::Black;
        self.board[3 * BOARD_WIDTH + 4] = TurnEnum::Black;

        // [6-7-5]모눈판 중앙의 왼쪽 위와 오른쪽 아래에 흰 돌을 놓는다
        self.board[3 * BOARD_WIDTH + 3] = TurnEnum::White;
        self.board[4 * BOARD_WIDTH + 4] = TurnEnum::White;

        self.turn = TurnEnum::Black; // [6-7-6]검은 돌의 턴으로 초기화한다

        self.cursor_position = Vec2 { x: 3, y: 3 }; // [6-7-7]커서의 좌표를 초기화한다

        self.draw_screen();
    }

    // [6-8]돌을 놓는 칸을 선택하는 함수를 선언한다
    pub fn input_position(&mut self) -> Vec2 {
        loop {
            self.draw_screen();

            match self.g.getch() {
                Ok(Key::Char('w')) => {
                    // [6-8-5]커서를 위쪽으로 이동한다
                    self.cursor_position.y -= 1;
                }
                Ok(Key::Char('s')) => {
                    // [6-8-7]커서를 아래쪽으로 이동한다
                    self.cursor_position.y += 1;
                }
                Ok(Key::Char('a')) => {
                    // [6-8-9]커서를 왼쪽으로 이동한다
                    self.cursor_position.x -= 1;
                }
                Ok(Key::Char('d')) => {
                    // [6-8-11]커서를 오른쪽으로 이동한다
                    self.cursor_position.x += 1;
                }
                Ok(Key::Esc) => {
                    std::process::exit(0);
                }
                _ => {
                    // [6-8-13]커서의 좌표에 돌을 놓을 수 있는지 여부를 판정한다
                    if self.check_can_place(self.turn, self.cursor_position, false) {
                        return self.cursor_position;
                    } else {
                        // [6-8-15]놓을 수 없다면
                        println!("놓을 수 없는 곳입니다.");

                        let _ = self.g.getch();
                    }
                }
            }

            // [6-8-18]커서를 좌우로 루프시킨다
            self.cursor_position.x =
                (BOARD_WIDTH as i8 + self.cursor_position.x) % (BOARD_WIDTH as i8);

            // [6-8-19]커서를 상하로 루프시킨다
            self.cursor_position.y =
                (BOARD_HEIGHT as i8 + self.cursor_position.y) % (BOARD_HEIGHT as i8);
        }
    }
}

fn main() {
    let mut ctx = Context::new();
    'start: loop {
        ctx.select_mode();
        ctx.init();
        // [6-9-6]메인루프
        loop {
            // [6-9-7]놓을 수 있는 칸이 없는지 여부를 판정한다
            if !ctx.check_can_place_all(ctx.turn) {
                // [6-9-8]턴을 바꾼다
                ctx.turn = if ctx.turn == TurnEnum::Black {
                    TurnEnum::White
                } else {
                    TurnEnum::Black
                };

                // [6-9-9]놓을 수 있는 칸이 없는지 여부를 판정한다
                if !ctx.check_can_place_all(ctx.turn) {
                    ctx.turn = TurnEnum::None; // [6-9-10]승부가 난 것으로 한다

                    ctx.draw_screen();

                    let _ = ctx.g.getch();

                    ctx.select_mode();
                    ctx.init();
                    continue 'start; // [6-9-13]시작 라벨로 점프한다
                } else {
                    // [6-9-14]상대에게 놓을 수 있는 칸이 있으면
                    continue; // [6-9-15]상대의 턴으로 스킵한다
                }
            }

            // [6-9-16]돌을 놓는 칸을 선언한다
            let place_position = 

            // [6-9-17]현재 턴의 담당이 플레이어인지 여부를 판정한다
            if ctx.is_player[ctx.turn as usize] {
                // [6-9-18]돌을 놓는 칸을 선택하는 함수를 호출한다
                ctx.input_position()
            } else {
                // [6-9-19]현재 턴의 담당이 플레이어가 아니라면
                ctx.draw_screen();

                let _ = ctx.g.getch();

                // [6-9-22]놓을 수 있는 좌표를 보유하는 벡터를 선언한다
                let mut positions = Vec::new();

                for y in 0..BOARD_HEIGHT {
                    for x in 0..BOARD_WIDTH {
                        // [6-9-25]대상 칸의 좌표를 선언한다
                        let position = Vec2 {
                            x: x as i8,
                            y: y as i8,
                        };

                        // [6-9-26]대상 좌표에 돌을 놓을 수 있는지 여부를 판정한다
                        if ctx.check_can_place(ctx.turn, position, false) {
                            // [6-9-27]벡터에 대상 좌표를 추가한다
                            positions.push(position);
                        }
                    }
                }

                // [6-9-28]놓을 수 있는 곳을 랜덤으로 얻는다
                *positions.choose(&mut ctx.rng).unwrap()
            };

            // [6-9-29]돌을 뒤집는다
            ctx.check_can_place(ctx.turn, place_position, true);

            // [6-9-30]현재 턴의 돌을 놓는다
            ctx.board[place_position.y as usize * BOARD_WIDTH + place_position.x as usize] =
                ctx.turn;

            // [6-9-31]턴을 바꾼다
            ctx.turn = if ctx.turn == TurnEnum::Black {
                TurnEnum::White
            } else {
                TurnEnum::Black
            };
        }
    }
}
