use langlib::{
    interpreter::{Err, Interpreter},
    lexer::Lexer,
    parser::Parser,
};

fn main() -> Result<(), Err> {
    let s = "
    let a = 250;
    let b = 2;
    
    print \"This progam can do some math:\";
    print ((a*b) * 2) - 500;
    
    let x = \"This is a cool string.\";
    let y = \"This is another cool string.\";
    
    print \"The two strings are not equal:\";
    print x == y;
    
    let c = !(!true == false);
    let d = false;
    print \"But c and d are:\";
    print c == d;
    ";

    let tokens = Lexer::new(s).tokenize()?;

    let stmts = Parser::new(tokens).get_statements()?;

    let mut interpreter = Interpreter::new(stmts);

    interpreter.interpret()?;

    Ok(())
}
