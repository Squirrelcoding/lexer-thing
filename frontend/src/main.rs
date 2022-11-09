use langlib::{
    interpreter::{Err, Interpreter},
    lexer::Lexer,
    parser::Parser,
};

fn main() -> Result<(), Err> {
    let s = "
        let a = 5;
        {

            let x = 43;
            print x;
        }

        let b = 14;
        print b;
    ";

    let tokens = Lexer::new(s).tokenize()?;

    let stmts = Parser::new(tokens).get_statements()?;


    let mut interpreter = Interpreter::new(stmts);

    interpreter.interpret()?;

    Ok(())
}
