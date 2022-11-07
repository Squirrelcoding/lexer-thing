# Lexer-thing

A little toy language that I'm currently working on, written in Rust. So far it can only interpret basic expressions and statements. It's called "lexer-thing" because it was originally supposed to be a lexer. But then I wanted to turn the project into a language. So far the following code is valid:
```
print "Hello, world!";
    
let a = 5;
let b = 4;
    
print a == b;
print a;
print b;
print (a + b) * 23;
```

## Currently working on
- Statements and stuff

## Features
- Basic unary expressions
- Boolean, string, and integer literals
- print statements
- Basic variables

## TODO
- Add support for comparisions other than `==`
- Get rid of so many `.clone()`s in the code, make it more efficient.