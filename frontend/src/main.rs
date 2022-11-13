use langlib::{interpreter::Interpreter, lexer::Lexer, parser::Parser};

fn main() -> Result<(), langlib::interpreter::Err> {
    let s = "
    for (let i = 0; i < 5; i = i + 1) {
        print i;
    }
    
    print \"i = \";
    print i;
    ";
    let tokens = Lexer::new(s).tokenize()?;


    let ast = Parser::new(tokens).get_statements()?;

    println!("{ast:?}");

    Interpreter::new(ast).interpret()?;
    Ok(())
}
