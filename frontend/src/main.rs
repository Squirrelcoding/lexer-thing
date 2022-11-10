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
    
    let i = 2;
    let j = \"hello\";
    {
        let i = 15;
        let k = false;
        print i;
        print j;
        {
            {
                {
                    {
                        print \"x is different when it's in a small block:\";
                        let x = true;
                        print x;
                    }
                }
            }
        }
    }
    
    print i;
    print \"Since k doesn't exist up here, the program crashes here!\";
    print k;
    ";

    let tokens = Lexer::new(s).tokenize()?;

    let stmts = Parser::new(tokens).get_statements()?;

    let interpreter = Interpreter::new(stmts);

    interpreter.interpret()?;

    Ok(())
}
