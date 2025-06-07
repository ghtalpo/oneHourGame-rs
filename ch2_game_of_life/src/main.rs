use getch_rs::Getch;
use getch_rs::Key;

const FieldWidth: usize = 12;
const FieldHeight: usize = 12;

struct Context {
    field: Vec<bool>, //bool[FieldHeight][FieldWidth];
    g: Getch,
}

impl Context {
    pub fn new() -> Self {
        let mut field = vec![false; FieldHeight * FieldWidth];
        field[0 * FieldWidth + 1] = true;
        field[1 * FieldWidth + 2] = true;
        field[2 * FieldWidth + 0] = true;
        field[2 * FieldWidth + 1] = true;
        field[2 * FieldWidth + 2] = true;
        Self {
            field,
            g: Getch::new(),
        }
    }
    pub fn draw_field(&self) {
        clearscreen::clear().unwrap();

        for y in 0..FieldHeight {
            for x in 0..FieldWidth {
                print!(
                    "{}",
                    if self.field[y * FieldWidth + x] {
                        "■"
                    } else {
                        "□"
                    }
                );
            }
            println!();
        }

        let _ = self.g.getch();
    }
}

fn main() {
    let ctx = Context::new();
    loop {
        ctx.draw_field();
    }
}
