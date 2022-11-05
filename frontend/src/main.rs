use langlib::{interpreter::Err, lexer::Lexer, parser::Parser};

fn main() -> Result<(), Err> {
    let x = "let x = !(\"this is a string.\" == \"this is another string.\"); print (23 -5) / 2; let y = (2 + 4) / 2; let z = !true; print \"This is a very cool string.\";";

    let tokens = Lexer::new(x).tokenize()?;

    println!("{tokens:?}");

    let mut parser = Parser::new(tokens);

    println!("{:?}", parser.get_statements()?);

    // println!("{:?}", parser.get_statements());

    // Interpreter::repl()?;

    Ok(())
}
