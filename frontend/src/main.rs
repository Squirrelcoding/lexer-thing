use std::path::Path;

use langlib::{interpreter::Interpreter, lexer::Lexer, parser::Parser};

fn main() -> Result<(), langlib::interpreter::Err> {
    let s = "
    let x = !(\"this is a string.\" == \"this is another string.\"); 
    print (23-5)/ 2; 
    let y =  (2 + 4) / 2; 
    let z = !true; 
    
    print \"This is a very cool string.\"; let undefinedVar;
    ";
    let tokens = Lexer::new(s).tokenize().unwrap();
    let ast = Parser::new(tokens).get_statements();

    println!("{ast:?}");

    Ok(())
}
