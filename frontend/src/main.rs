use langlib::{interpreter::Err, lexer::Lexer, parser::Parser};

fn main() -> Result<(), Err> {
    let x = "!((5) == 5)";

    let tokens = Lexer::new(x).tokenize()?;


    let mut parser = Parser::new(tokens);

    println!("{:?}", parser.un_expr()?.eval());

    // println!("{:?}", parser.get_statements());


    // Interpreter::repl()?;

    Ok(())
}
