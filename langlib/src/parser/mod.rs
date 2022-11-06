pub mod error;
mod expr;
mod num;
mod stmt;

use crate::stmt::Stmt;

use self::error::ParserError;

use super::lexer::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn get_statements(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut stmt_vec = Vec::new();

        let stmt = self.stmt()?;

        stmt_vec.push(stmt);

        self.adv();

        while !self.is_at_end() {
            let stmt = self.stmt()?;

            // Advance for some weird edge case.
            self.adv();

            stmt_vec.push(stmt);
        }

        Ok(stmt_vec)
    }

    /// Checks if the current token matches one of the given possible tokens, and advances if successful.
    fn matches(&mut self, possible_tokens: &[Token]) -> Option<Token> {
        let old_pos = self.cursor;

        // Iterate through the possible tokens
        match possible_tokens.iter().find_map(|token| {
            // Return the token if the current token matches

            if &self.tokens[self.cursor] == token {
                self.adv();
                return Some(token.to_owned());
            }
            None
        }) {
            Some(token) => Some(token),
            None => {
                // If we got here it means that the token didnt match any rule so we need to reset the cursor field

                self.cursor = old_pos;

                None
            }
        }
    }

    /// Attempts to match against a rule and advances if the match is successful.
    fn match_rule(&mut self, rules: &[Token]) -> bool {
        // Check if the current cursor is a `let` keyword.

        let old_pos = self.cursor;

        // Try to match against a rule and advance if successful
        let is_ok = rules.iter().all(|token| {
            // We use a _ here because we dont care about the actual number itself
            // Check if an int token is the current rule
            if let Token::Int(_) = token {
                // Check if the token matches the current rule
                if let Token::Int(_) = self.tokens[self.cursor] {
                    self.adv();
                    return true;
                }
            }

            // If the token matches then increment and return true;
            if let Token::Ident(_) = token {
                self.adv();
                return true;
            }

            // Match the rest of the tokens
            if &self.tokens[self.cursor] == token {
                self.adv();
                return true;
            }

            false
        });

        // Reset the cursor if there was an error
        if !is_ok {
            self.cursor = old_pos;
        }

        is_ok
    }

    /// Returns the current cursor of the parser
    pub fn pos(&self) -> usize {
        self.cursor
    }

    /// Increments the `pos` field
    pub fn adv(&mut self) {
        self.cursor += 1;
    }

    /// Returns the previous token
    fn prev(&self) -> Result<Token, ParserError> {
        if self.cursor == 0 {
            return Err(ParserError::InvalidTokenIndex);
        }

        Ok(self.tokens[self.cursor - 1].clone())
    }

    /// Returns the current token
    fn curr(&self) -> Token {
        self.tokens[self.cursor].clone()
    }

    /// Returns the token at the given index `i`
    fn at(&self, i: usize) -> Result<Token, ParserError> {
        if i >= self.tokens.len() {
            return Err(ParserError::InvalidTokenIndex);
        }

        Ok(self.tokens[i].clone())
    }

    /// Returns a boolean indicating whether the position is at the end of the token stream.
    pub fn is_at_end(&self) -> bool {
        self.cursor + 1 >= self.tokens.len()
    }
}

#[cfg(test)]
mod parser_tests {

    use super::super::lexer::op::BinOp;
    use crate::{
        expr::Expr,
        lexer::{token::Keyword, Lexer},
        stmt::Declaration,
    };

    use super::*;

    #[test]
    fn test_helpers() {
        let s = "let a = (1 + 1) + 2 - 432; let b = 3;";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let fourth = parser.at(4);

        assert!(fourth.is_ok());

        let fourth = fourth.unwrap();

        assert_eq!(fourth, Token::Int(1));
        assert_eq!(parser.curr(), Token::Keyword(Keyword::Let));

        parser.adv();

        assert_eq!(parser.prev(), Ok(Token::Keyword(Keyword::Let)));

        (1..(parser.tokens.len() - 1)).for_each(|_| {
            parser.adv();
        });

        assert!(parser.is_at_end());

        assert_eq!(parser.pos(), parser.tokens.len() - 1);
    }

    #[test]
    fn match_rule_success() {
        let a = "let x = 5;";
        let mut lexer = Lexer::new(a);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let rules = [
            Token::Keyword(Keyword::Let),
            Token::Ident("x".to_owned()),
            Token::DeclarationSign,
            Token::Int(5),
            Token::Semi,
        ];

        assert!(parser.match_rule(&rules));
    }

    #[test]
    fn match_rule_fail() {
        let a = "let = 5;";
        let mut lexer = Lexer::new(a);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let rules = [
            Token::Keyword(Keyword::Let),
            Token::Ident("x".to_owned()),
            Token::DeclarationSign,
            Token::Int(0),
            Token::Semi,
        ];

        assert!(!parser.match_rule(&rules));
    }

    #[test]
    fn matches_success() {
        let pm = [Token::Op(BinOp::Add), Token::Op(BinOp::Sub)];

        let a = "+ - 3";
        let mut lexer = Lexer::new(a);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        assert_eq!(parser.matches(&pm), Some(Token::Op(BinOp::Add)));
        assert_eq!(parser.matches(&pm), Some(Token::Op(BinOp::Sub)));
        assert_eq!(parser.matches(&pm), None)
    }

    #[test]
    fn test_compare_nums_success() {
        let s = "(3 + 15) / 2 == 9";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert!(result.eval().is_ok());
    }

    #[test]
    fn test_compare_nums_fail() {
        let s = "(3 + 15) / 2 == 20";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap().eval();
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result, Expr::Bool(false));
    }

    #[test]
    fn test_compare_strs_success() {
        let s = " \"This is a string\" == \"This is a string\"";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap().eval();

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result, Expr::Bool(true));
    }

    #[test]
    fn test_compare_strs_fail() {
        let s = " \"This is a string\" == \"This is another string\"";

        println!("================================================");

        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap().eval();

        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result, Expr::Bool(false));
    }

    #[test]
    pub fn compare_bools_success() {
        let s = "true == true";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap().eval();

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result, Expr::Bool(true));
    }

    #[test]
    pub fn compare_bools_fail() {
        let s = "true == false";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap().eval();

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result, Expr::Bool(false));
    }

    #[test]
    pub fn test_unary_negation() {
        let s = "let x = !(true == false);";

        let mut lexer = Lexer::new(s);

        let binding_stmt = Parser::new(lexer.tokenize().unwrap()).stmt();

        assert!(binding_stmt.is_ok());

        let binding_stmt = binding_stmt.unwrap();

        assert_eq!(
            binding_stmt,
            Stmt::Declaration(Declaration {
                ident: "x".to_owned(),
                val: Expr::Bool(true)
            })
        );
    }

    #[test]
    pub fn test_unary_negation_but_with_strings() {
        let s = "let x = !(\"this is a string.\" == \"this is another string.\");";

        let mut lexer = Lexer::new(s);

        let binding_stmt = Parser::new(lexer.tokenize().unwrap()).stmt();

        assert!(binding_stmt.is_ok());

        let binding_stmt = binding_stmt.unwrap();

        assert_eq!(
            binding_stmt,
            Stmt::Declaration(Declaration {
                ident: "x".to_owned(),
                val: Expr::Bool(true)
            })
        );
    }

    #[test]
    pub fn test_parse_statements() {
        let s = "let    
    
        x =   !        (\"this is a string.\"                == \"this is another string.\"); print (23-5)/ 2; let y =  (           2 + 4) / 2; let z = !   true; 
        
        
        
        print 
        
        
        \"This is a very cool string.\"; let undefinedVar;";

        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let statements = parser.get_statements();

        assert!(statements.is_ok());

        let statements = statements.unwrap();

        assert_eq!(
            statements,
            vec![
                Stmt::Declaration(Declaration {
                    ident: "x".to_owned(),
                    val: Expr::Bool(true)
                }),
                Stmt::Print(Expr::Num(9)),
                Stmt::Declaration(Declaration {
                    ident: "y".to_owned(),
                    val: Expr::Num(3)
                }),
                Stmt::Declaration(Declaration {
                    ident: "z".to_owned(),
                    val: Expr::Bool(false)
                }),
                Stmt::Print(Expr::Str("This is a very cool string.".to_owned())),
                Stmt::Declaration(Declaration {
                    ident: "undefinedVar".to_owned(),
                    val: Expr::Null
                }),
            ]
        )
    }
}
