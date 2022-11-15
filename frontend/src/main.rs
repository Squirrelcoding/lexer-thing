
use langlib::{lexer::Lexer, parser::Parser};

fn main() -> Result<(), langlib::interpreter::Err> {
    let s = "
    let stringA = \"This is a nice string.\";
let stringB = \"This is not a nice string.\";
if (stringA == stringB) {
    print \"This shouldn't print but it's here anyway.\";
} else {
    print \"The two strings are not equal!\";

    let a = 324;
    let b = 32;

    {
        let x = !true;
        let y = false;
        if (x == y) print \"x and y are equal.\";
    }

    {
        {
            {
                if ((true or false) and true) {
                    print \"You can nest statements!\";
                    let x = 0;
                    while (x < 10) {
                        print \"And run while loops!\";
                        x = x + 1;
                    }

                    for (let i = 0; i <= 10; i = i + 1) {
                        print \"And for loops!\";
                    }

                    someFunctionCall(a, b, c)(\"A COOL STRING!!!!!!!!!!!!!\", 1232435 * 15 / 23, !(!true == false))();
                }
            }
        }
    }

    print \"Some simple arithmetic with a and b:\";
    print (a * 2) + b; 
}
    ";

    let mut lexer = Lexer::new(s);
    
    let expr = Parser::new(lexer.tokenize()?).get_statements()?;


    println!("{:?}", expr);
    Ok(())
}
