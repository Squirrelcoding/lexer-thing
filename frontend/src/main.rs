use std::io::{self, BufRead};

use langlib::{lexer::Lexer, parser::error::ParserError};

use langlib::parser::Parser;

fn main() -> Result<(), ParserError> {
    let s = "let x = \"This is a string.\"; let y = 5;";

    let mut lexer = Lexer::new(s);

    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);

    println!("{:?}", parser.stmt());


    println!("{:?}", parser.stmt());


    Ok(())
}
