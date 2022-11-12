use std::path::Path;

use langlib::{interpreter::Interpreter, lexer::Lexer, parser::Parser};

fn main() -> Result<(), langlib::interpreter::Err> {
    Interpreter::from_file(Path::new("test.lt"))?.interpret()?;
    Ok(())
}
