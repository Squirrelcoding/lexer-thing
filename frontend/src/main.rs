use langlib::{lexer::Lexer, parser::error::ParserError};

use langlib::parser::Parser;

fn main() -> Result<(), ParserError> {
    let s = "let x = (3 == 1);";


    let mut lexer = Lexer::new(s);
    
    
    let mut parser = Parser::new(lexer.tokenize().unwrap());
    
    let result = parser.stmt()?;

    println!("{result:?}");

    Ok(())
}
