use langlib::{lexer::Lexer, parser::error::ParserError};

use langlib::parser::Parser;

fn main() -> Result<(), ParserError> {
    let s = "(4 + 3) / 13 == (4 + 3) / 13;";

    let mut lexer = Lexer::new(s);

    let mut parser = Parser::new(lexer.tokenize().unwrap());

    let result = parser.expr()?;

    println!("{:?}", result.eval_bin());

    Ok(())
}
