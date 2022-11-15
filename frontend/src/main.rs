
use langlib::{lexer::Lexer, parser::Parser, interpreter::Interpreter};

fn main() -> Result<(), langlib::interpreter::Err> {
    let s = "
    someFunctionCall(a, b, c)(\"A COOL STRING!!!!!!!!!!!!!\", 1232435 * 15 / 23, !(!true == false))();

    ";

    let mut lexer = Lexer::new(s);
    
    let expr = Parser::new(lexer.tokenize()?).get_statements()?;

    println!("{expr:?}");


    Interpreter::new(expr).interpret()?;
    Ok(())
}
