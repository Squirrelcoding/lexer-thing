# Lexer-thing

A little toy language that I'm currently working on, written in Rust. So far it can only interpret basic expressions and statements. It's called "lexer-thing" because it was originally supposed to be a lexer. But then I wanted to turn the project into a language. So far the following code is valid:
```
let x = 271828;
let y = 314159;

print "Hello, world!";
print true == false;

print x - y;
```

## Currently working on
- (More complicated) expressions with variables

## Features
- Basic unary expressions
- Boolean, string, and integer literals
- print statements