use langlib::{
    interpreter::{Err},
    lexer::Lexer,
    parser::Parser,
};

fn main() -> Result<(), Err> {
    let s = "let x = !(true == false);";

    let mut lexer = Lexer::new(s);

    let binding_stmt = Parser::new(lexer.tokenize().unwrap()).stmt();

    assert!(binding_stmt.is_ok());

    let binding_stmt = binding_stmt.unwrap();

    println!("{binding_stmt:?}");

    Ok(())
}
