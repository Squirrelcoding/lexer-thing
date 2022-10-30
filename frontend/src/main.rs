use langlib::{lexer::Lexer, parser::error::ParserError};

use langlib::parser::Parser;

fn main() -> Result<(), ParserError> {
    let s = "let v = (3 == 2);";

    let mut lexer = Lexer::new(s);

    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);

    println!("{:?}", parser.stmt());

    Ok(())
}
