use std::time::{Duration, SystemTime};

use sdl3::{event::Event, pixels::Color};

struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }
    pub fn init(&mut self) {}
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

    let mut ctx = Context::new();
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

        // if ctx.block_intersect_field() {
        //     // ctx.block = last_block;
        // } else {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // draw something

        canvas.present();
        // }

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
