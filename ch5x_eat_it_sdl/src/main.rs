use std::time::{Duration, SystemTime};

use rand::{rngs::ThreadRng, seq::IndexedRandom};

use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    render::{Canvas, FPoint, FRect, Texture, TextureQuery},
    video::Window,
};

// [2]상수를 정의하는 곳

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const MAZE_WIDTH: usize = 19;
const MAZE_HEIGHT: usize = 19;

const CELL_SIZE: f32 = 32.0;

const FPS: usize = 2;
const INTERVAL: f32 = 1000.0 / FPS as f32; // 밀리 초 

// [3-1]캐릭터의 종류를 정의한다
enum CharacterEnum {
    Player = 0,
    Random = 1,
    Chase = 2,
    Ambush = 3,
    Siege = 4,
    Max = 5,
}

impl TryFrom<usize> for CharacterEnum {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == CharacterEnum::Player as usize => Ok(CharacterEnum::Player),
            x if x == CharacterEnum::Random as usize => Ok(CharacterEnum::Random),
            x if x == CharacterEnum::Chase as usize => Ok(CharacterEnum::Chase),
            x if x == CharacterEnum::Ambush as usize => Ok(CharacterEnum::Ambush),
            x if x == CharacterEnum::Siege as usize => Ok(CharacterEnum::Siege),
            _ => Err(()),
        }
    }
}

// [3-2]방향의 종류를 정의한다
enum DirectionEnum {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
    Max = 4,
}

#[derive(PartialEq)]
enum GameStateEnum {
    Playing,
    GameOver,
    GameEnd,
}

// [4-1]벡터 구조체를 선언한다
#[derive(Clone, Copy, Default, PartialEq, Debug)]
struct Vec2 {
    x: i8,
    y: i8,
}

impl Vec2 {
    pub fn add(&mut self, other: &Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
    pub fn add_new(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn subtract_new(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    pub fn get_loop_position(&mut self) {
        self.x = (self.x + MAZE_WIDTH as i8) % (MAZE_WIDTH as i8);
        self.y = (self.y + MAZE_HEIGHT as i8) % (MAZE_HEIGHT as i8);
    }
}

// [4-2]캐릭터 구조체를 선언한다
#[derive(Clone, Copy)]
struct Character {
    position: Vec2,
    default_position: Vec2,
    last_position: Vec2,
}

impl Character {
    pub fn new() -> Self {
        Self {
            position: Vec2::default(),
            default_position: Vec2 { x: 9, y: 13 },
            last_position: Vec2::default(),
        }
    }
}

struct Context {
    maze: Vec<String>,
    default_maze: Vec<String>,
    canvas: Canvas<Window>,
    characters: [Character; CharacterEnum::Max as usize],
    directions: [Vec2; DirectionEnum::Max as usize],
    rng: ThreadRng,
    game_state: GameStateEnum,
}

impl Context {
    pub fn new(canvas: Canvas<Window>) -> Self {
        // [5-2]미로의 초기 상태를 선언한다
        let default_maze = vec![
            "#########o#########",
            "#ooooooo#o#ooooooo#",
            "#o###o#o#o#o#o###o#",
            "#o# #o#ooooo#o# #o#",
            "#o###o###o###o###o#",
            "#ooooooooooooooooo#",
            "#o###o###o###o###o#",
            "#ooo#o#ooooo#o#ooo#",
            "###o#o#o###o#o#o###",
            "oooooooo# #oooooooo",
            "###o#o#o###o#o#o###",
            "#ooo#o#ooooo#o#ooo#",
            "#o###o###o###o###o#",
            "#oooooooo oooooooo#",
            "#o###o###o###o###o#",
            "#o# #o#ooooo#o# #o#",
            "#o###o#o#o#o#o###o#",
            "#ooooooo#o#ooooooo#",
            "#########o#########",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        Self {
            maze: Vec::with_capacity(MAZE_HEIGHT),
            default_maze,
            canvas,
            // [5-3]캐릭터의 배열을 선언한다
            characters: [
                Character::new(),
                Character {
                    position: Vec2::default(),
                    default_position: Vec2 { x: 1, y: 1 },
                    last_position: Vec2::default(),
                },
                Character {
                    position: Vec2::default(),
                    default_position: Vec2 { x: 17, y: 1 },
                    last_position: Vec2::default(),
                },
                Character {
                    position: Vec2::default(),
                    default_position: Vec2 { x: 1, y: 17 },
                    last_position: Vec2::default(),
                },
                Character {
                    position: Vec2::default(),
                    default_position: Vec2 { x: 17, y: 17 },
                    last_position: Vec2::default(),
                },
            ],
            // [5-4]방향 벡터의 배열을 선언한다
            directions: [
                Vec2 { x: 0, y: -1 },
                Vec2 { x: -1, y: 0 },
                Vec2 { x: 0, y: 1 },
                Vec2 { x: 1, y: 0 },
            ],
            rng: rand::rng(),
            game_state: GameStateEnum::Playing,
        }
    }

    fn draw_circle(&mut self, y: usize, x: usize, step: usize) {
        let mut points = vec![];
        let r = CELL_SIZE / 2.0;
        for deg in (0..360).step_by(step) {
            let rad = (deg as f32) / (180.0) * std::f32::consts::PI;
            let pt = FPoint::new(
                x as f32 * CELL_SIZE + r + r * rad.cos(),
                y as f32 * CELL_SIZE + r + r * rad.sin(),
            );
            points.push(pt);
        }
        points.push(FPoint::new(
            x as f32 * CELL_SIZE + 2.0 * r,
            y as f32 * CELL_SIZE + r,
        ));

        self.canvas.draw_lines(&points[..]).unwrap();
    }

    // [6-5]랜덤한 이동 목적지를 얻는 함수를 선언한다
    fn get_random_position(&mut self, character: Character) -> Vec2 {
        // [6-5-1]이동 목적지의 후보 리스트를 선언한다
        let mut positions = Vec::new();

        for i in 0..DirectionEnum::Max as usize {
            // [6-5-3]각 방향의 좌표를 선언한다
            let mut new_position = character.position.add_new(&self.directions[i]);

            new_position.get_loop_position();

            // [6-5-5]대상 좌표에 이동 가능한지 여부를 판정한다
            let current_block = self.maze[new_position.y as usize]
                .chars()
                .nth(new_position.x as usize)
                .unwrap();
            if
            // 벽이 아니다
            current_block != '#' 
            // 그리고 이전 회의 좌표와 같지 않다
            && new_position != character.last_position
            {
                // [6-5-6]대상 좌표를 이동 목적지의 후보 리스트에 추가한다
                positions.push(new_position);
            }
        }

        // [6-5-7]이동 목적지의 후보 중에서 랜덤으로 좌표를 반환한다
        *positions.choose(&mut self.rng).unwrap()
    }

    // [6-6]목표 지점까지의 최단 경로에서 첫 좌표를 얻는 함수를 선언한다
    fn get_chase_position(&mut self, character: Character, target_position: Vec2) -> Vec2 {
        // [6-6-1]경로를 탐색해야 하는 좌표 리스트를 선언한다
        let mut to_check_positions = Vec::new();

        // [6-6-2]탐색을 하는 캐릭터 자신의 좌표를 탐색해야 하는 좌표 리스트에 추가한다
        to_check_positions.push(character.position);

        // [6-6-3]탐색 시작 지점에서 각 칸까지의 거리를 보유하는 배열을 선언한다
        let mut distances = [0_isize; MAZE_HEIGHT * MAZE_WIDTH];

        for y in 0..MAZE_HEIGHT {
            for x in 0..MAZE_WIDTH {
                // [6-6-6]대상 칸까지의 거리를 미설정으로 초기화한다
                distances[y * MAZE_WIDTH + x] = -1;
            }
        }

        // [6-6-7]탐색을 하는 캐릭터 자신의 좌표까지의 거리는 0으로 설정한다
        distances[character.position.y as usize * MAZE_WIDTH + character.position.x as usize] = 0;

        // [6-6-8]탐색 시작 지점에서 각 칸까지의 경로를 보유하는 배열을 선언한다
        let mut routes = [const { Vec::new() }; MAZE_WIDTH * MAZE_HEIGHT];

        // [6-6-9]탐색해야 하는 좌표 리스트가 비워질 때까지 반복한다
        while !to_check_positions.is_empty() {
            for i in 0..DirectionEnum::Max as usize {
                // [6-8-11]탐색 중인 좌표에 인접하는 각 방향의 좌표를 구한다
                let mut new_position = to_check_positions[0].add_new(&self.directions[i]);

                new_position.get_loop_position();

                // [6-6-13]대상 좌표까지의 거리를 선언한다
                let new_distance = distances[to_check_positions[0].y as usize * MAZE_WIDTH
                    + to_check_positions[0].x as usize]
                    + 1;

                // [6-6-14]대상 좌표를 검색해야 하는지 여부를 판정한다
                if (
                    // 미설정
                    distances[to_check_positions[0].y as usize * MAZE_WIDTH
                    + to_check_positions[0].x as usize]
                    < 0
                    ||
                     // 또는 최단 경로 
                    new_distance
                        < distances[to_check_positions[0].y as usize * MAZE_WIDTH
                            + to_check_positions[0].x as usize])
                             // 그리고 벽이 아니다
                    && self.maze[new_position.y as usize]
                        .chars()
                        .nth(new_position.x as usize)
                        .unwrap()
                        != '#'
                {
                    // [6-6-15]대상 좌표까지의 거리를 갱신한다
                    distances[to_check_positions[0].y as usize * MAZE_WIDTH
                        + to_check_positions[0].x as usize] = new_distance;

                    // [6-6-16]대상 좌표를 탐색해야 하는 좌표 리스트로 추가한다
                    to_check_positions.push(new_position);

                    // [6-6-17]대상 좌표까지의 경로를 1개 전 좌표의 경로로 초기화한다
                    routes[new_position.y as usize * MAZE_WIDTH + new_position.x as usize] = routes
                        [to_check_positions[0].y as usize * MAZE_WIDTH
                            + to_check_positions[0].x as usize]
                        .clone();

                    // [6-6-18]대상 좌표까지의 경로에 대상 좌표를 추가한다
                    routes[new_position.y as usize * MAZE_WIDTH + new_position.x as usize]
                        .push(new_position);
                }
            }

            // [6-6-19]탐색해야 하는 좌표 리스트에서 맨 앞의 좌표를 삭제한다
            to_check_positions.remove(0);
        }

        // [6-6-20]목표 지점까지의 경로가 있는지 여부를 판정한다
        if
        // 경로가 있다
        !routes[target_position.y as usize * MAZE_WIDTH + target_position.x as usize].is_empty()
        // 그리고 이전 회의 좌표와 다른 좌표라면
            && routes[target_position.y as usize * MAZE_WIDTH + target_position.x as usize][0]
                != character.last_position
        {
            // [6-6-21]목표 지점까지의 경로 1개 전 좌표를 반환한다
            routes[target_position.y as usize * MAZE_WIDTH + target_position.x as usize][0]
        } else {
            // [6-6-22]목표 지점까지의 경로가 없으면
            // [6-6-23]랜덤한 좌표를 반환한다
            self.get_random_position(character)
        }
    }

    // [6-7]미로를 그리는 함수를 선언한다
    pub fn draw_maze(
        &mut self,
        width_bad: u32,
        height_bad: u32,
        texture_bad: &Texture<'_>,
        width_good: u32,
        height_good: u32,
        texture_good: &sdl3::render::Texture<'_>,
    ) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        match self.game_state {
            GameStateEnum::Playing => {
                // [6-7-1]화면 버퍼를 선언한다
                let mut screen: Vec<String> = Vec::with_capacity(MAZE_HEIGHT);

                // [6-7-2]화면 버퍼에 미로를 복사한다
                // screen.clone_from_slice(&self.maze);
                for index in 0..MAZE_HEIGHT {
                    screen.insert(index, self.maze.get(index).unwrap().clone());
                }

                for i in 0..CharacterEnum::Max as usize {
                    // [6-7-4]캐릭터의 번호를 화면 버퍼에 써넣는다
                    let x = self.characters[i].position.x as usize;
                    let y = self.characters[i].position.y as usize;
                    screen[y].replace_range(x..x + 1, format!("{}", i).as_str());
                }

                for y in 0..MAZE_HEIGHT {
                    for x in 0..MAZE_WIDTH {
                        // [6-7-8]칸을 그린다
                        match screen[y].chars().nth(x) {
                            Some(' ') => {}
                            Some('#') => {
                                self.canvas.set_draw_color(Color::WHITE);
                                self.canvas
                                    .fill_rect(FRect::new(
                                        x as f32 * CELL_SIZE,
                                        y as f32 * CELL_SIZE,
                                        CELL_SIZE,
                                        CELL_SIZE,
                                    ))
                                    .unwrap();
                            }
                            Some('o') => {
                                self.canvas.set_draw_color(Color::RED);
                                self.canvas
                                    .draw_rect(FRect::new(
                                        x as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                                        y as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                                        3.0,
                                        3.0,
                                    ))
                                    .unwrap();
                            }
                            Some('0') => {
                                self.canvas.set_draw_color(Color::YELLOW);
                                self.draw_circle(y, x, 30);
                            }
                            Some('1') => {
                                self.canvas.set_draw_color(Color::BLUE);
                                self.draw_circle(y, x, 72);
                            }
                            Some('2') => {
                                self.canvas.set_draw_color(Color::RED);
                                self.draw_circle(y, x, 120);
                            }
                            Some('3') => {
                                self.canvas.set_draw_color(Color::GREEN);
                                self.draw_circle(y, x, 90);
                            }
                            Some('4') => {
                                self.canvas.set_draw_color(Color::MAGENTA);
                                self.draw_circle(y, x, 60);
                            }
                            _ => {}
                        }
                    }
                }
            }
            GameStateEnum::GameOver => {
                let target = FRect::new(
                    (SCREEN_WIDTH as f32 - width_bad as f32) / 2.0,
                    (SCREEN_HEIGHT as f32 - height_bad as f32) / 2.0,
                    width_bad as f32,
                    height_bad as f32,
                );

                self.canvas.copy(texture_bad, None, Some(target)).unwrap();
            }
            GameStateEnum::GameEnd => {
                let target = FRect::new(
                    (SCREEN_WIDTH as f32 - width_good as f32) / 2.0,
                    (SCREEN_HEIGHT as f32 - height_good as f32) / 2.0,
                    width_good as f32,
                    height_good as f32,
                );

                self.canvas.copy(texture_good, None, Some(target)).unwrap();
            }
        }

        self.canvas.present();
    }

    // [6-8]게임을 초기화하는 함수를 선언한다
    pub fn init(&mut self) {
        // [6-8-1]미로에 초기 상태를 복사한다
        // self.maze.clone_from_slice(&self.default_maze);
        for index in 0..MAZE_HEIGHT {
            self.maze
                .insert(index, self.default_maze.get(index).unwrap().clone());
        }

        for i in 0..CharacterEnum::Max as usize {
            // [6-8-3]캐릭터의 좌표를 초기화한다
            self.characters[i].position = self.characters[i].default_position;
            self.characters[i].last_position = self.characters[i].default_position;
        }
    }

    // [6-9]게임 오버 함수를 선언한다
    fn is_game_over(&self) -> bool {
        for i in CharacterEnum::Player as usize + 1..CharacterEnum::Max as usize {
            // [6-9-2]대상 몬스터와 플레이어의 좌표가 동일한지 여부를 판정한다
            if self.characters[i].position
                == self.characters[CharacterEnum::Player as usize].position
            {
                // [6-9-8]게임 오버가 되었다는 결과를 반환한다
                return true;
            }
        }
        // [6-9-9]게임 오버가 되지 않았다는 결과를 반환한다
        false
    }

    // [6-10]엔딩 함수를 선언한다
    fn is_complete(&self) -> bool {
        for y in 0..MAZE_HEIGHT {
            for x in 0..MAZE_WIDTH {
                // [6-10-3]대상 칸이 도트인지 여부를 판정한다
                if self.maze[y].chars().nth(x).unwrap() == 'o' {
                    // [6-10-4]클리어가 아니라는 결과를 반환한다
                    return false;
                }
            }
        }
        // [6-10-10]클리어했다는 결과를 반환한다
        true
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl3::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Keyboard", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    // Load a font
    // let mut font = ttf_context.load_font(font_path, 128.0)?;
    let mut font = ttf_context.load_font("DOSSaemmul.ttf", 32.0)?;
    font.set_style(sdl3::ttf::FontStyle::BOLD);

    let surface_sad = font
        .render("GAME OVER")
        .blended(Color::WHITE)
        .map_err(|e| e.to_string())?;
    let texture_bad = texture_creator
        .create_texture_from_surface(&surface_sad)
        .map_err(|e| e.to_string())?;
    let TextureQuery { width, height, .. } = texture_bad.query();
    let width_bad = width;
    let height_bad = height;

    let surface_good = font
        .render("CONGRATULATIONS!")
        .blended(Color::WHITE)
        .map_err(|e| e.to_string())?;
    let texture_good = texture_creator
        .create_texture_from_surface(&surface_good)
        .map_err(|e| e.to_string())?;
    let TextureQuery { width, height, .. } = texture_good.query();
    let width_good = width;
    let height_good = height;

    let mut events = sdl_context.event_pump()?;

    let mut ctx = Context::new(canvas);

    // [6-11-4]게임을 초기화하는 함수를 호출한다
    ctx.init();

    // [6-11-6]이전 회의 갱신 시각을 선언한다
    let mut last_clock = SystemTime::now();

    'running: loop {
        if ctx.game_state == GameStateEnum::Playing {
            match last_clock.elapsed() {
                Ok(elapsed) => {
                    // [6-11-9]이전 회의 갱신으로부터 대기 시간이 경과했는지 여부를 판정한다
                    if (elapsed.as_millis() as f32) >= INTERVAL {
                        // [6-11-10]이전 회의 갱신 시각을 현재 시각으로 갱신한다
                        last_clock = SystemTime::now();

                        for i in CharacterEnum::Player as usize + 1..CharacterEnum::Max as usize {
                            // [6-11-12]이동 목적지의 좌표를 선언한다
                            let mut new_position = ctx.characters[i].position;

                            match CharacterEnum::try_from(i).unwrap() {
                                CharacterEnum::Random => {
                                    // [6-11-14]변덕 몬스터

                                    // [6-11-15]랜덤한 이동 목적지의 좌표를 설정한다
                                    new_position = ctx.get_random_position(ctx.characters[i]);
                                }
                                CharacterEnum::Chase => {
                                    // [6-11-16]추적 몬스터

                                    // [6-11-17]플레이어를 추적하는 좌표를 설정한다
                                    new_position = ctx.get_chase_position(
                                        ctx.characters[i],
                                        ctx.characters[CharacterEnum::Player as usize].position,
                                    );
                                }
                                CharacterEnum::Ambush => {
                                    // [6-11-18]전진 몬스터

                                    // [6-11-19]플레이어의 방향 벡터를 선언한다
                                    let player_direction = ctx.characters
                                        [CharacterEnum::Player as usize]
                                        .position
                                        .subtract_new(
                                            &ctx.characters[CharacterEnum::Player as usize]
                                                .last_position,
                                        );

                                    // [6-11-20]목표 지점을 선언한다
                                    let mut target_position =
                                        ctx.characters[CharacterEnum::Player as usize].position;

                                    // [6-11-21]3회 반복한다
                                    for _ in 0..3 {
                                        // [6-11-22]목표 지점에 플레이어의 방향 벡터를 더한다
                                        target_position.add(&player_direction);
                                    }

                                    // [6-11-23]목표 지점을 상하좌우로 루프시킨 좌표로 변환한다
                                    target_position.get_loop_position();

                                    // [6-11-24]목표 지점을 목표로 하는 좌표를 설정한다
                                    new_position =
                                        ctx.get_chase_position(ctx.characters[i], target_position);
                                }
                                CharacterEnum::Siege => {
                                    // [6-11-25]협공 몬스터

                                    // [6-11-26]추적 몬스터에서 플레이어까지의 벡터를 얻는다
                                    let chase_to_player = ctx.characters
                                        [CharacterEnum::Player as usize]
                                        .position
                                        .subtract_new(
                                            &ctx.characters[CharacterEnum::Chase as usize].position,
                                        );

                                    // [6-11-27]목적지를 선언한다
                                    let mut target_position = ctx.characters
                                        [CharacterEnum::Player as usize]
                                        .position
                                        .add_new(&chase_to_player);

                                    target_position.get_loop_position();

                                    // [6-11-29]목표 지점을 목표로 하는 좌표를 설정한다
                                    new_position =
                                        ctx.get_chase_position(ctx.characters[i], target_position);
                                }
                                _ => {}
                            }

                            // [6-11-30]이전 회의 좌표를 현재 좌표로 갱신한다
                            ctx.characters[i].last_position = ctx.characters[i].position;

                            // [6-11-31]이동 목적지로 이동시킨다
                            ctx.characters[i].position = new_position;
                        }

                        // [6-11-32]게임 오버가 되었는지 여부를 판정한다
                        if ctx.is_game_over() {
                            ctx.game_state = GameStateEnum::GameOver;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    // an error occurred!
                    println!("Error: {e:?}");
                    std::process::exit(0);
                }
            }

            // [6-11-36]플레이어의 새로운 좌표를 선언한다
            let mut new_position = ctx.characters[CharacterEnum::Player as usize].position;

            // [6-11-37]입력된 키에 따라 분기한다
            let mut key_up = false;
            let mut key_down = false;
            let mut key_left = false;
            let mut key_right = false;
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::W => key_up = true,
                        Keycode::S => key_down = true,
                        Keycode::A => key_left = true,
                        Keycode::D => key_right = true,
                        Keycode::Escape => std::process::exit(0),
                        _ => {}
                    },
                    _ => {}
                }
            }

            if key_up {
                new_position.y -= 1;
            }
            if key_down {
                new_position.y += 1;
            }
            if key_left {
                new_position.x -= 1;
            }
            if key_right {
                new_position.x += 1;
            }

            // [6-11-42]이동 목적지의 좌표를 상하좌우로 이동시킨다
            new_position.get_loop_position();

            // [6-11-43]이동 목적지가 벽이 아닌지 여부를 판정한다
            let current_block = ctx.maze[new_position.y as usize]
                .chars()
                .nth(new_position.x as usize)
                .unwrap();
            if current_block != '#' {
                // [6-11-44]플레이어의 이전 좌표를 현재 좌표로 갱신한다
                ctx.characters[CharacterEnum::Player as usize].last_position =
                    ctx.characters[CharacterEnum::Player as usize].position;

                // [6-11-45]플레이어의 좌표를 갱신한다
                ctx.characters[CharacterEnum::Player as usize].position = new_position;

                // [6-11-46]게임 오버가 되었는지 여부를 판정한다
                if ctx.is_game_over() {
                    ctx.game_state = GameStateEnum::GameOver;
                }

                // [6-11-48]플레이어의 좌표에 도트가 있는지 여부를 판정한다
                let x = new_position.x as usize;
                let y = new_position.y as usize;
                if current_block == 'o' {
                    // [6-11-49]플레이어 좌표의 도트를 지운다
                    ctx.maze[y].replace_range(x..x + 1, " ");

                    // [6-11-50]클리어했는지 여부를 판정한다
                    if ctx.is_complete() {
                        ctx.game_state = GameStateEnum::GameEnd;
                    }
                }
            }
        }
        if ctx.game_state == GameStateEnum::GameOver || ctx.game_state == GameStateEnum::GameEnd {
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::Escape => std::process::exit(0),
                        _ => {
                            ctx.game_state = GameStateEnum::Playing;
                            ctx.init();
                        }
                    },
                    _ => {}
                }
            }
        }

        // [6-11-34]화면을 다시 그린다
        ctx.draw_maze(
            width_bad,
            height_bad,
            &texture_bad,
            width_good,
            height_good,
            &texture_good,
        );

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
