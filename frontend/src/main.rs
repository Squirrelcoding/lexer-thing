use langlib::{interpreter::Err, lexer::Lexer, parser::Parser};

fn main() -> Result<(), Err> {
    let x = "
        let x = (\"some cool string.\" == \"some cool string.\");
        let y = 23;
    ";

    let tokens = Lexer::new(x).tokenize()?;

    println!("{tokens:?}");

    let mut parser = Parser::new(tokens);

    println!("{:?}", parser.get_statements());

    // Interpreter::repl()?;

    Ok(())
}
