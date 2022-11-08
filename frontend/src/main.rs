use langlib::{interpreter::Err, lexer::Lexer, parser::Parser};

fn main() -> Result<(), Err> {
    let s = "1 + 2 * 5";

    let tokens = Lexer::new(s).tokenize()?;

    let mut parser = Parser::new(lexer.tokenize()?);

    let stmts = parser.get_statements()?;

    let mut interpreter = Interpreter::new(stmts);

    interpreter.interpret()?;
    println!("{tokens:?}");

    let mut parser = Parser::new(tokens);

    println!("{}", parser.term_e()?);

    Ok(())
}
