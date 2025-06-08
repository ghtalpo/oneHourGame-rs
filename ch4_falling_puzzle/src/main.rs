struct Context {}

impl Context {
    pub fn new() -> Self {}
    pub fn init(&mut self) {
        self.draw_screen();
    }
    pub fn draw_screen(&self) {}
}

fn main() {
    let mut ctx = Context::new();
    ctx.init();
    loop {}
}
