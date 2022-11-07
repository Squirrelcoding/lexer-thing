use langlib::{
    interpreter::{Err, Interpreter},
    lexer::Lexer,
    parser::Parser,
};

fn main() -> Result<(), Err> {
    // let s = "let x = !(\"this is a string.\" == \"this is another string.\"); 
    // print (23-5)/ 2; 
    // let y = (2 + 4) / 2; 
    // let z = !true;    

    // print \"This is a very cool string.\"; 
    // let undefinedVar;";

    let s = "(false) == true;";

    let mut lexer = Lexer::new(s);

    let mut parser = Parser::new(lexer.tokenize()?);

    let stmts = parser.get_statements()?;

    let mut interpreter = Interpreter::new(stmts);

    interpreter.interpret()?;

    Ok(())
}
