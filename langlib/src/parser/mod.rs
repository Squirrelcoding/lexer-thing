pub mod error;
mod num;

use crate::stmt::Stmt;

use self::error::ParserError;

use super::{expr::Expr, lexer::token::Token, stmt::Assignment};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn stmt(&mut self) -> Result<Stmt, ParserError> {
        let idx = match (self.cursor..self.tokens.len()).into_iter().find_map(|i| {
            if self.at(i).unwrap() == Token::Semi {
                return Some(i);
            }
            None
        }) {
            Some(idx) => idx,
            None => return Err(ParserError::BadStatement),
        };

        let slice = &self.tokens[self.cursor..idx];

        // Big ugly match expression

        match slice.iter().next() {
            // Check if there's even a token
            Some(token) => match token {
                // Match the keyword
                Token::Keyword(keyword) => match keyword.as_str() {
                    // If it's an assignment statement
                    "let" => {
                        // Then check if these things apply
                        if self.match_rule(&[
                            Token::Keyword("let".to_owned()),
                            Token::Ident("".to_owned()),
                            Token::AssignmentSign,
                        ]) {
                            // Get the identier and value
                            let ident = self.at(self.cursor - 2)?.try_into_ident()?;
                            let expr = self.expr()?;

                            // Advance because of semicolon
                            self.adv();

                            return Ok(Stmt::Assignment(Assignment { ident, val: expr }));
                        }
                        Err(ParserError::BadStatement)
                    }

                    _ => {
                        Err(ParserError::BadStatement)
                    }
                },
                _ => Err(ParserError::BadStatement),
            },
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    pub fn expr(&mut self) -> Result<Expr, ParserError> {
        self.num_expr()
            .map(|result| Expr::Num(result))
            .or_else(|_| self.str_expr())
            .or_else(|_| self.bool_expr())
    }

    /// Attempts to match a string token, and advances if successful.
    pub fn str_expr(&mut self) -> Result<Expr, ParserError> {
        if let Token::String(string) = self.curr() {
            self.adv();
            return Ok(Expr::Str(string));
        }

        Err(ParserError::Expected(Token::String("".to_owned())))
    }

    /// Attempts to match a boolean token, and advances if successful.
    pub fn bool_expr(&mut self) -> Result<Expr, ParserError> {
        if let Some(bool_val) = self.matches(&[
            Token::Keyword("true".to_owned()),
            Token::Keyword("false".to_owned()),
        ]) {
            return Ok(bool_val.into_expr()?);
        }

        Err(ParserError::Expected(Token::String("".to_owned())))
    }

    /// Attempts to match an assignment
    pub fn assignment(&mut self) -> Result<Stmt, ParserError> {
        // Check if it starts with a `let` keyword

        if !self.match_rule(&[Token::Keyword("let".to_owned())]) {
            return Err(ParserError::Expected(Token::Keyword("let".to_owned())));
        }

        let ident = self.curr().try_into_ident()?;

        self.adv();

        // Check if there follows an equals sign
        if !self.match_rule(&[Token::AssignmentSign]) {
            return Err(ParserError::Expected(Token::AssignmentSign));
        }

        let expr = self.num_expr()?;

        Ok(Stmt::Assignment(Assignment {
            ident,
            val: Expr::Num(expr),
        }))
    }

    /// Attempts to parse a comparision statement
    pub fn compare(&mut self) -> Result<Expr, ParserError> {
        let lhs = self.expr()?;

        if !self.match_rule(&[Token::EqSign]) {
            return Err(ParserError::Expected(Token::EqSign));
        }

        let rhs = self.expr()?;

        Ok(Expr::Comparison(Box::new(lhs), Box::new(rhs)))
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

        let old_post = self.cursor;

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
            self.cursor = old_post;
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
    fn prev(&self) -> Token {
        self.tokens[self.cursor - 1].clone()
    }

    /// Returns the current token
    fn curr(&self) -> Token {
        self.tokens[self.cursor].clone()
    }

    /// Returns the next token
    fn next(&self) -> Result<Token, ParserError> {
        if self.is_at_end() {
            return Err(ParserError::UnexpectedEOF);
        }

        Ok(self.tokens[self.cursor + 1].clone())
    }

    /// Returns the next token and increments the cursor
    fn consume_next(&mut self) -> Result<Token, ParserError> {
        if self.is_at_end() {
            return Err(ParserError::UnexpectedEOF);
        }

        self.adv();

        Ok(self.curr())
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

    use super::super::lexer::op::Op;
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn test_helpers() {
        let s = "let a = (1 + 1) + 2 - 432; let b = 3;";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let fourth = parser.at(4);
        let next = parser.next();

        assert!(fourth.is_ok());
        assert!(next.is_ok());

        let fourth = fourth.unwrap();
        let next = next.unwrap();

        assert_eq!(fourth, Token::Int(1));
        assert_eq!(parser.curr(), Token::Keyword("let".to_owned()));
        assert_eq!(next, Token::Ident("a".to_owned()));

        parser.adv();

        assert_eq!(parser.prev(), Token::Keyword("let".to_owned()));

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
            Token::Keyword("let".to_owned()),
            Token::Ident("x".to_owned()),
            Token::AssignmentSign,
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
            Token::Keyword("let".to_owned()),
            Token::Ident("x".to_owned()),
            Token::AssignmentSign,
            Token::Int(0),
            Token::Semi,
        ];

        assert!(!parser.match_rule(&rules));
    }

    #[test]
    fn matches_success() {
        let pm = [Token::Op(Op::Add), Token::Op(Op::Sub)];

        let a = "+ - *";
        let mut lexer = Lexer::new(a);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        assert_eq!(parser.matches(&pm), Some(Token::Op(Op::Add)));
        assert_eq!(parser.matches(&pm), Some(Token::Op(Op::Sub)));
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

        assert!(result.eval());
    }

    #[test]
    fn test_compare_nums_fail() {
        let s = "(3 + 15) / 2 == 20";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert!(!result.eval());
    }

    #[test]
    fn test_compare_strs_success() {
        let s = " \"This is a string\" == \"This is a string\"";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert!(result.eval());
    }

    #[test]
    fn test_compare_strs_fail() {
        let s = " \"This is a string\" == \"This is another string\"";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert!(!result.eval());
    }

    #[test]
    pub fn compare_bools_success() {
        let s = "true == true";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert!(result.eval());
    }

    #[test]
    pub fn compare_bools_fail() {
        let s = "true == false";
        let mut lexer = Lexer::new(s);

        let mut parser = Parser::new(lexer.tokenize().unwrap());

        let result = parser.compare();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert!(!result.eval());
    }
}
