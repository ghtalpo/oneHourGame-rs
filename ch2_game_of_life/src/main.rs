use std::process::exit;

use getch_rs::Getch;
use getch_rs::Key;

const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 12;

struct Context {
    field: Vec<bool>, //bool[FieldHeight][FieldWidth];
    g: Getch,
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

        match self.g.getch() {
            Ok(Key::Esc) => {
                exit(0);
            }
            _ => {}
        }
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
}

fn main() {
    let mut ctx = Context::new();
    loop {
        ctx.draw_field();
        ctx.step_simulation();
    }
}
