use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::path::Path;

use langlib::{interpreter::Interpreter, lexer::Lexer};

fn main() -> Result<(), langlib::interpreter::Err> {
    let s = "class Poop";
    let tokens = Lexer::new(s).tokenize()?;
    println!("{tokens:?}");
    Ok(())
}
