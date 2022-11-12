use std::path::Path;

use langlib::{interpreter::Interpreter, lexer::Lexer, parser::Parser};

fn main() -> Result<(), langlib::interpreter::Err> {
    // Interpreter::from_file(Path::new("test.lt"))?.interpret()?;

    let s = "
    while (5 == 5) {
        if (true or false) {
            print \"HERE\";
        }
    }
    ";

    let tokens = Lexer::new(s).tokenize()?;
    let ast = Parser::new(tokens).get_statements()?;

    println!("{ast:?}");

    Ok(())
}
