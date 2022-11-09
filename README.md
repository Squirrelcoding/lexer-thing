# Lexer-thing

A little toy language that I'm currently working on by following [*Crafting Interpreters*](https://craftinginterpreters.com/), but its written in Rust. So far it can only interpret basic expressions and statements. It's called "lexer-thing" because it was originally supposed to be a lexer. But then I wanted to turn the project into a language. So far the following code is valid:
```
let a = 250;
let b = 2;

print "This progam can do some math:";
print ((a*b) * 2) - 500;

let x = "This is a cool string.";
let y = "This is another cool string.";

print "The two strings are not equal:";
print x == y;

let c = !(!true == false);
let d = false;
print "But c and d are:";
print c == d;
```

## Currently working on
- Lexical scope
## Features
- Basic unary expressions
- Boolean, string, and integer literals
- print statements
- Basic variables
- Comparisions for numbers (e.g `a > b` , `a != b`, etc.)

## TODO
- Theres gonna be a looooooooooootttttttt of refactoring sooner or later.