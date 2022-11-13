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
                        print "And run loops!";
                        x = x + 1;
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
- Control flow (loops)
## Features
- Basic unary expressions
- Boolean, string, and integer literals
- print statements
- Basic variables
- Comparisions for numbers (e.g `a > b` , `a != b`, etc.)
- Lexical scope

## TODO
- Allow redeclaring variables
- Allow assignment without having to declare a new variable.
- Theres gonna be a looooooooooootttttttt of refactoring sooner or later.