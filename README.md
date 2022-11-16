# Lexer-thing

A little toy language that I'm currently working on by following [*Crafting Interpreters*](https://craftinginterpreters.com/), but its written in Rust. So far it can only interpret basic expressions and statements. It's called "lexer-thing" because it was originally supposed to be a lexer. But then I wanted to turn the project into a language. So far the following code is valid:
```
let stringA = "This is a nice string.";
let stringB = "This is not a nice string.";
if (stringA == stringB) {
    print "This shouldn't print but it's here anyway.";
} else {
    print "The two strings are not equal!";

    let a = 324;
    let b = 32;

    {
        let x = !true;
        let y = false;
        if (x == y) print "x and y are equal.";
    }

    {
        {
            {
                if ((true or false) and true) {
                    print "You can nest statements!";
                    let x = 0;
                    while (x < 10) {
                        print "And run while loops!";
                        x = x + 1;
                    }

                    for (let i = 0; i <= 10; i = i + 1) {
                        print "And for loops!";
                    }
                }
            }
        }
    }

    print "Some simple arithmetic with a and b:";
    print (a * 2) + b; 
}
```

## Currently working on
- MAJOR restructuring to 
## Features
- Basic unary expressions
- Boolean, string, and integer literals
- print statements
- Basic variables
- Comparisions for numbers (e.g `a > b` , `a != b`, etc.)
- Lexical scope
- Basic control flow (if statements and loops)