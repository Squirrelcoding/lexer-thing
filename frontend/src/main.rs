
use langlib::{lexer::Lexer, parser::Parser, interpreter::Interpreter};

fn main() -> Result<(), langlib::interpreter::Err> {
    let s = "
        {
            let x = 1;
            print x;
        }
        ";

    let mut lexer = Lexer::new(s);
    
    let expr = Parser::new(lexer.tokenize()?).get_statements()?;

    println!("{expr:?}");

    Interpreter::new(expr).interpret()?;
    Ok(())
}
