use std::process::exit;
use std::time::SystemTime;

use getch_rs::Getch;
use getch_rs::Key;

const FIELD_WIDTH: usize = 160;
const FIELD_HEIGHT: usize = 160;
const FPS: usize = 10;
const INTERVAL: f32 = 1000.0 / FPS as f32; // 밀리 초 

struct Context {
    field: Vec<bool>, //bool[FieldHeight][FieldWidth];
    g: Getch,
    last_clock: SystemTime,
}

impl Context {
    pub fn new() -> Self {
        let mut field = vec![false; FIELD_HEIGHT * FIELD_WIDTH];
        field[0 * FIELD_WIDTH + 1] = true;
        field[1 * FIELD_WIDTH + 2] = true;
        field[2 * FIELD_WIDTH + 0] = true;
        field[2 * FIELD_WIDTH + 1] = true;
        field[2 * FIELD_WIDTH + 2] = true;
        Self {
            field,
            g: Getch::new(),
            last_clock: SystemTime::now(),
        }
    }
    pub fn draw_field(&self) {
        clearscreen::clear().unwrap();

        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                print!(
                    "{}",
                    if self.field[y * FIELD_WIDTH + x] {
                        "■"
                    } else {
                        "□"
                    }
                );
            }
            println!();
        }

        // match self.g.getch() {
        //     Ok(Key::Esc) => {
        //         exit(0);
        //     }
        //     _ => {}
        // }
    }
    pub fn get_living_cells_count(&self, x_: i64, y_: i64) -> u64 {
        let mut count = 0;
        for y in y_ - 1..=y_ + 1 {
            // if y < 0 || y >= FieldHeight as i64 {
            //     continue;
            // }
            let rooped_y = (FIELD_HEIGHT as i64 + y) % FIELD_HEIGHT as i64;
            for x in x_ - 1..=x_ + 1 {
                // if x < 0 || x >= FieldWidth as i64 {
                //     continue;
                // }
                let rooped_x = (FIELD_WIDTH as i64 + x) % FIELD_WIDTH as i64;
                if rooped_x == x_ && rooped_y == y_ {
                    continue;
                }
                if self.field[rooped_y as usize * FIELD_WIDTH + rooped_x as usize] {
                    count += 1;
                }
            }
        }
        return count;
    }
    pub fn step_simulation(&mut self) {
        let mut next_field = vec![false; FIELD_HEIGHT * FIELD_WIDTH];
        for y in 0..FIELD_HEIGHT as i64 {
            for x in 0..FIELD_WIDTH as i64 {
                let living_cell_count = self.get_living_cells_count(x, y);
                next_field[y as usize * FIELD_WIDTH + x as usize] = match living_cell_count {
                    0 | 1 => false,
                    2 => self.field[y as usize * FIELD_WIDTH + x as usize],
                    3 => true,
                    _ => false,
                };
            }
            println!();
        }
        self.field.clone_from_slice(&next_field);
    }
    pub fn pattern_transfer(
        &mut self,
        dest_x: usize,
        dest_y: usize,
        src_width: usize,
        src_height: usize,
        p_pattern: &Vec<bool>,
    ) {
        for y in 0..src_height {
            for x in 0..src_width {
                self.field[(dest_y + y) * FIELD_WIDTH + dest_x + x] = p_pattern[y * src_width + x];
            }
        }
    }
}

fn main() {
    let mut ctx = Context::new();
    loop {
        match ctx.last_clock.elapsed() {
            Ok(elapsed) => {
                if (elapsed.as_millis() as f32) < INTERVAL {
                    continue;
                }
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {e:?}");
                exit(0);
            }
        }
        // if new_clock
        ctx.draw_field();
        ctx.step_simulation();

        ctx.last_clock = SystemTime::now();
    }
}
