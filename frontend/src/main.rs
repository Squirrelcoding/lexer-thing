use langlib::{interpreter::{Err, Interpreter}, lexer::Lexer, parser::Parser};

fn main() -> Result<(), Err> {
    let x = "
    
    
    print (324 + 3) / 4;
    print \"THIS IS A REALLY COOL STRING.\";
    
    ";

    let tokens = Lexer::new(x).tokenize()?;
    let mut parser = Parser::new(tokens);

    let statements = parser.get_statements()?;

    let interpreter = Interpreter::new(statements);

    interpreter.interpret();

    Ok(())
}
