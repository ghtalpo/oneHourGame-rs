struct Context {}

impl Context {
    pub fn new() -> Self {}
    pub fn init(&mut self) {}
}

fn main() {
    let mut ctx = Context::new();
    ctx.init();
    loop {}
}
