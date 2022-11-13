use langlib::{interpreter::Interpreter, lexer::Lexer, parser::Parser};

fn main() -> Result<(), langlib::interpreter::Err> {
    let s = "
    let x = 0;
    while (x < 5) {
        print x;
        x = x + 1;
    }
    ";

    let tokens = Lexer::new(s).tokenize()?;

    let ast = Parser::new(tokens).get_statements()?;

    let interpreter = Interpreter::new(ast);

    interpreter.interpret()?;

    Ok(())
}
