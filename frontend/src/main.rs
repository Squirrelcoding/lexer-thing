use std::path::Path;

use langlib::{interpreter::Interpreter, lexer::Lexer, parser::Parser};

fn main() -> Result<(), langlib::interpreter::Err> {
    let s = "
    if (2 == 5) {
        print \"2 is equal to 5\";
        print \"The univese makes no sense.\";
    } else {
        print \"2 is NOT equal to 5.\";
        print \"As it should be.\";
    }
    ";
    let tokens = Lexer::new(s).tokenize().unwrap();
    let ast = Parser::new(tokens).stmt()?;

    println!("{ast:?}");

    Ok(())
}
