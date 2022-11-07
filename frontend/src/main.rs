use langlib::{
    interpreter::{Err, Interpreter},
    lexer::Lexer,
    parser::Parser,
};

fn main() -> Result<(), Err> {
    let x = "
    let x = 271828;
    let y = 314159;
    
    print \"Hello, world!\";
    print true == false;
    
    print x - y;

    ";

    let tokens = Lexer::new(x).tokenize()?;

    let mut parser = Parser::new(tokens);

    let statements = parser.get_statements()?;

    let mut interpreter = Interpreter::new(statements);

    interpreter.interpret()?;

    Ok(())
}
