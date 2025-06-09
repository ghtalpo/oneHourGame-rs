use std::time::{Duration, SystemTime};

use sdl3::{
    event::Event,
    pixels::Color,
    render::{Canvas, FRect},
    video::Window,
};

const MAZE_WIDTH: usize = 19;
const MAZE_HEIGHT: usize = 19;

const CELL_SIZE: f32 = 32.0;

struct Context {
    maze: Vec<String>,
    default_maze: Vec<String>,
    canvas: Canvas<Window>,
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
        }
    }

    pub fn init(&mut self) {
        // self.maze.clone_from_slice(&self.default_maze);
        for index in 0..MAZE_HEIGHT {
            self.maze
                .insert(index, self.default_maze.get(index).unwrap().clone());
        }

        self.draw_maze();
    }

    pub fn draw_maze(&mut self) {
        let mut screen: Vec<String> = Vec::with_capacity(MAZE_HEIGHT);

        // screen.clone_from_slice(&self.maze);
        for index in 0..MAZE_HEIGHT {
            screen.insert(index, self.maze.get(index).unwrap().clone());
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
                    _ => {}
                }
            }
        }

        self.canvas.present();
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
        // ctx.wasd = 0;
        match last_clock.elapsed() {
            Ok(elapsed) => {
                // if (elapsed.as_millis() as f32) >= INTERVAL {
                //     last_clock = SystemTime::now();

                //     // ctx.fall_block(); // [6-8-8]낙하 블록을 떨어뜨리는 함수를 호출한다
                //     //                   // continue;
                // }
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {e:?}");
                std::process::exit(0);
            }
        }

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    // Keycode::W => ctx.wasd |= Key::W as u8,
                    // Keycode::A => ctx.wasd |= Key::A as u8,
                    // Keycode::S => ctx.wasd |= Key::S as u8,
                    // Keycode::D => ctx.wasd |= Key::D as u8,
                    _ => {}
                },
                _ => {}
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
