use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::path::Path;

use langlib::interpreter::Interpreter;

fn main() -> Result<(), langlib::interpreter::Err> {
    Interpreter::from_file(Path::new("test.lt"))?.interpret()?;
    Ok(())
}
