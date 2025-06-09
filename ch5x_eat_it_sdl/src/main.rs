use std::time::{Duration, SystemTime};

use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    render::{Canvas, FPoint, FRect},
    video::Window,
};

const MAZE_WIDTH: usize = 19;
const MAZE_HEIGHT: usize = 19;

const CELL_SIZE: f32 = 32.0;

const FPS: usize = 2;
const INTERVAL: f32 = 1000.0 / FPS as f32; // 밀리 초 

enum CharacterEnum {
    Player = 0,
    Random = 1,
    Max = 2,
}

struct Character {
    position: Vec2,
    default_position: Vec2,
}

impl Character {
    pub fn new() -> Self {
        Self {
            position: Vec2::default(),
            default_position: Vec2 { x: 9, y: 13 },
        }
    }
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
                },
            ],
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
        }

        self.draw_maze();
    }

    pub fn draw_maze(&mut self) {
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
                    _ => {}
                }
            }
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
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Keyboard", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas();

    let mut events = sdl_context.event_pump()?;

    let mut ctx = Context::new(canvas);
    ctx.init();

    let mut last_clock = SystemTime::now();

    'running: loop {
        match last_clock.elapsed() {
            Ok(elapsed) => {
                if (elapsed.as_millis() as f32) >= INTERVAL {
                    last_clock = SystemTime::now();

                    // ctx.fall_block(); // [6-8-8]낙하 블록을 떨어뜨리는 함수를 호출한다
                    //                   // continue;
                }
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {e:?}");
                std::process::exit(0);
            }
        }

        let mut new_position = ctx.characters[CharacterEnum::Player as usize].position;
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => new_position.y -= 1,
                    Keycode::S => new_position.y += 1,
                    Keycode::A => new_position.x -= 1,
                    Keycode::D => new_position.x += 1,
                    Keycode::Escape => std::process::exit(0),
                    _ => {}
                },
                _ => {}
            }
        }
        new_position.get_loop_position();

        let current_block = ctx.maze[new_position.y as usize]
            .chars()
            .nth(new_position.x as usize)
            .unwrap();
        if current_block != '#' {
            let x = new_position.x as usize;
            let y = new_position.y as usize;
            if current_block == 'o' {
                ctx.maze[y].replace_range(x..x + 1, " ");
            }
            ctx.characters[CharacterEnum::Player as usize].position = new_position;
        }

        ctx.draw_maze();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
