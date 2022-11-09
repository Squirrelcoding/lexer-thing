use langlib::{interpreter::Err, lexer::Lexer, parser::Parser};

fn main() -> Result<(), Err> {
    let s = "12 > 43 <= 324 + 2345 -3254  == 324 == 243 == 34";

    let tokens = Lexer::new(s).tokenize()?;

    let mut parser = Parser::new(tokens);
    println!("expr: {}", parser.expr_e()?);

    Ok(())
}
