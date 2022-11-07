use langlib::{
    interpreter::{Err, Interpreter},
    lexer::Lexer,
    parser::Parser,
};

fn main() -> Result<(), Err> {
    let x = "

    let a = 1;

    let b = 2;

    print a + b;

    ";

    let tokens = Lexer::new(x).tokenize()?;

    let mut parser = Parser::new(tokens);

    let statements = parser.get_statements()?;

    let mut interpreter = Interpreter::new(statements);

    interpreter.interpret()?;

    Ok(())
}
