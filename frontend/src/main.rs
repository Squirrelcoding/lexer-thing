use langlib::{lexer::Lexer, parser::error::ParserError};

use langlib::parser::Parser;

fn main() -> Result<(), ParserError> {
    let s = "(3 * 3) - 6 == \"hello\" ";

    let mut lexer = Lexer::new(s);

    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);

    println!("{:?}", parser.compare()?.eval());

    Ok(())
}
