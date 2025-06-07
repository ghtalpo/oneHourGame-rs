const FieldWidth: usize = 12;
const FieldHeight: usize = 12;

#[derive(Debug)]
struct Context {
    field: Vec<bool>, //bool[FieldHeight][FieldWidth];
}

impl Context {
    pub fn new() -> Self {
        let mut field = vec![false; FieldHeight * FieldWidth];
        field[0 * FieldWidth + 1] = true;
        field[1 * FieldWidth + 2] = true;
        field[2 * FieldWidth + 0] = true;
        field[2 * FieldWidth + 1] = true;
        field[2 * FieldWidth + 2] = true;
        Self { field }
    }
}

fn main() {
    // loop {}
    let mut context = Context::new();
    println!("{:?}", context);
}
