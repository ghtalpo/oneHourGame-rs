use std::time::{Duration, SystemTime};

use rand::{rngs::ThreadRng, seq::IndexedRandom};

use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    render::{Canvas, FPoint, FRect, TextureQuery},
    video::Window,
};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const MAZE_WIDTH: usize = 19;
const MAZE_HEIGHT: usize = 19;

const CELL_SIZE: f32 = 32.0;

const FPS: usize = 2;
const INTERVAL: f32 = 1000.0 / FPS as f32; // 밀리 초 

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

enum DirectionEnum {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
    Max = 4,
}

#[derive(PartialEq)]
enum GameStateEnum {
    Playing = 0,
    GameOver = 1,
}

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

    pub fn init(&mut self) {
        // self.maze.clone_from_slice(&self.default_maze);
        for index in 0..MAZE_HEIGHT {
            self.maze
                .insert(index, self.default_maze.get(index).unwrap().clone());
        }

        for i in 0..CharacterEnum::Max as usize {
            self.characters[i].position = self.characters[i].default_position;
            self.characters[i].last_position = self.characters[i].default_position;
        }

        // self.draw_maze();
    }

    pub fn draw_maze(&mut self, width: u32, height: u32, texture: &sdl3::render::Texture<'_>) {
        let mut screen: Vec<String> = Vec::with_capacity(MAZE_HEIGHT);

        // screen.clone_from_slice(&self.maze);
        for index in 0..MAZE_HEIGHT {
            screen.insert(index, self.maze.get(index).unwrap().clone());
        }

        for i in 0..CharacterEnum::Max as usize {
            let x = self.characters[i].position.x as usize;
            let y = self.characters[i].position.y as usize;
            screen[y].replace_range(x..x + 1, format!("{}", i).as_str());
        }

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        for y in 0..MAZE_HEIGHT {
            for x in 0..MAZE_WIDTH {
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

        if self.game_state == GameStateEnum::GameOver {
            let target = FRect::new(
                MAZE_WIDTH as f32 * CELL_SIZE as f32,
                (SCREEN_HEIGHT as f32 - height as f32) / 2.0,
                width as f32,
                height as f32,
            );

            self.canvas.copy(&texture, None, Some(target)).unwrap();
        }

        self.canvas.present();
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

    fn get_random_position(&mut self, character: Character) -> Vec2 {
        let mut positions = Vec::new();
        for i in 0..DirectionEnum::Max as usize {
            let mut new_position = character.position.add_new(&self.directions[i]);

            new_position.get_loop_position();

            let current_block = self.maze[new_position.y as usize]
                .chars()
                .nth(new_position.x as usize)
                .unwrap();
            if current_block != '#' && new_position != character.last_position {
                positions.push(new_position);
            }
        }
        *positions.choose(&mut self.rng).unwrap()
    }

    fn get_chase_position(&mut self, character: Character, target_position: Vec2) -> Vec2 {
        let mut to_check_positions = Vec::new();

        to_check_positions.push(character.position);

        let mut distances = [0_isize; MAZE_HEIGHT * MAZE_WIDTH];

        for y in 0..MAZE_HEIGHT {
            for x in 0..MAZE_WIDTH {
                distances[y * MAZE_WIDTH + x] = -1;
            }
        }

        distances[character.position.y as usize * MAZE_WIDTH + character.position.x as usize] = 0;

        let mut routes = [const { Vec::new() }; MAZE_WIDTH * MAZE_HEIGHT];

        while !to_check_positions.is_empty() {
            for i in 0..DirectionEnum::Max as usize {
                let mut new_position = to_check_positions[0].add_new(&self.directions[i]);

                new_position.get_loop_position();

                let new_distance = distances[to_check_positions[0].y as usize * MAZE_WIDTH
                    + to_check_positions[0].x as usize]
                    + 1;
                if (distances[to_check_positions[0].y as usize * MAZE_WIDTH
                    + to_check_positions[0].x as usize]
                    < 0
                    || new_distance
                        < distances[to_check_positions[0].y as usize * MAZE_WIDTH
                            + to_check_positions[0].x as usize])
                    && self.maze[new_position.y as usize]
                        .chars()
                        .nth(new_position.x as usize)
                        .unwrap()
                        != '#'
                {
                    distances[to_check_positions[0].y as usize * MAZE_WIDTH
                        + to_check_positions[0].x as usize] = new_distance;

                    to_check_positions.push(new_position);

                    routes[new_position.y as usize * MAZE_WIDTH + new_position.x as usize] = routes
                        [to_check_positions[0].y as usize * MAZE_WIDTH
                            + to_check_positions[0].x as usize]
                        .clone();

                    routes[new_position.y as usize * MAZE_WIDTH + new_position.x as usize]
                        .push(new_position);
                }
            }

            to_check_positions.remove(0);
        }

        if !routes[target_position.y as usize * MAZE_WIDTH + target_position.x as usize].is_empty()
            && routes[target_position.y as usize * MAZE_WIDTH + target_position.x as usize][0]
                != character.last_position
        {
            return routes[target_position.y as usize * MAZE_WIDTH + target_position.x as usize][0];
        } else {
            return self.get_random_position(character);
        }
    }

    fn is_game_over(&self) -> bool {
        for i in CharacterEnum::Player as usize + 1..CharacterEnum::Max as usize {
            if self.characters[i].position
                == self.characters[CharacterEnum::Player as usize].position
            {
                return true;
            }
        }
        return false;
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

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    // Load a font
    // let mut font = ttf_context.load_font(font_path, 128.0)?;
    let mut font = ttf_context.load_font("DOSSaemmul.ttf", 32.0)?;
    font.set_style(sdl3::ttf::FontStyle::BOLD);

    let surface = font
        .render("GAME OVER")
        .blended(Color::WHITE)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let TextureQuery { width, height, .. } = texture.query();

    let mut events = sdl_context.event_pump()?;

    let mut ctx = Context::new(canvas);
    ctx.init();

    let mut last_clock = SystemTime::now();

    'running: loop {
        if ctx.game_state == GameStateEnum::Playing {
            match last_clock.elapsed() {
                Ok(elapsed) => {
                    if (elapsed.as_millis() as f32) >= INTERVAL {
                        last_clock = SystemTime::now();

                        for i in CharacterEnum::Player as usize + 1..CharacterEnum::Max as usize {
                            let mut new_position = ctx.characters[i].position;
                            match CharacterEnum::try_from(i).unwrap() {
                                CharacterEnum::Random => {
                                    new_position =
                                        ctx.get_random_position(ctx.characters[i].clone());
                                }
                                CharacterEnum::Chase => {
                                    new_position = ctx.get_chase_position(
                                        ctx.characters[i].clone(),
                                        ctx.characters[CharacterEnum::Player as usize].position,
                                    );
                                }
                                CharacterEnum::Ambush => {
                                    let player_direction = ctx.characters
                                        [CharacterEnum::Player as usize]
                                        .position
                                        .subtract_new(
                                            &ctx.characters[CharacterEnum::Player as usize]
                                                .last_position,
                                        );

                                    let mut target_position =
                                        ctx.characters[CharacterEnum::Player as usize].position;

                                    for _ in 0..3 {
                                        target_position.add(&player_direction);
                                    }

                                    target_position.get_loop_position();

                                    new_position = ctx.get_chase_position(
                                        ctx.characters[i].clone(),
                                        target_position,
                                    );
                                }
                                CharacterEnum::Siege => {
                                    let chase_to_player = ctx.characters
                                        [CharacterEnum::Player as usize]
                                        .position
                                        .subtract_new(
                                            &ctx.characters[CharacterEnum::Chase as usize].position,
                                        );

                                    let mut target_position = ctx.characters
                                        [CharacterEnum::Player as usize]
                                        .position
                                        .add_new(&chase_to_player);

                                    target_position.get_loop_position();

                                    new_position = ctx.get_chase_position(
                                        ctx.characters[i].clone(),
                                        target_position,
                                    );
                                }
                                _ => {}
                            }
                            ctx.characters[i].last_position = ctx.characters[i].position;
                            ctx.characters[i].position = new_position;
                        }
                    }
                }
                Err(e) => {
                    // an error occurred!
                    println!("Error: {e:?}");
                    std::process::exit(0);
                }
            }

            let mut new_position = ctx.characters[CharacterEnum::Player as usize].position;
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

            new_position.get_loop_position();

            let current_block = ctx.maze[new_position.y as usize]
                .chars()
                .nth(new_position.x as usize)
                .unwrap();
            if current_block != '#' {
                ctx.characters[CharacterEnum::Player as usize].last_position =
                    ctx.characters[CharacterEnum::Player as usize].position;

                if ctx.is_game_over() {
                    ctx.game_state = GameStateEnum::GameOver;
                }

                let x = new_position.x as usize;
                let y = new_position.y as usize;
                if current_block == 'o' {
                    ctx.maze[y].replace_range(x..x + 1, " ");
                }
                ctx.characters[CharacterEnum::Player as usize].position = new_position;
            }
        }
        if ctx.game_state == GameStateEnum::GameOver {
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
        ctx.draw_maze(width, height, &texture);

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
