use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::path::Path;

use langlib::interpreter::Interpreter;

fn main() -> Result<(), langlib::interpreter::Err> {
    let now = std::time::Instant::now();
    Interpreter::from_file(Path::new("test.lt"))?.interpret()?;
    println!("{}", now.elapsed().as_micros());
    Ok(())
}
