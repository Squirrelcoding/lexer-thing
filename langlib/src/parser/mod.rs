pub mod error;
mod num;

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

    /// Attempts to match an assignment
    pub fn assignment(&mut self) -> Result<Assignment, ParserError> {
        // Check if it starts with a `let` keyword

        if !self.match_rule(&[Token::Keyword("let".to_owned())]) {
            return Err(ParserError::InvalidLetStatement);
        }

        let ident = self.curr().try_into_ident()?;

        self.adv();

        // Check if there follows an equals sign
        if !self.match_rule(&[Token::AssignmentSign]) {
            return Err(ParserError::InvalidLetStatement);
        }

        let expr = self.num_expr()?;

        Ok(Assignment {
            ident,
            val: Expr::Num(expr),
        })
    }

    pub fn compare(&mut self) -> Result<Expr, ParserError> {
        let lhs = self.num_expr()?;

        if !self.match_rule(&[Token::EqSign]) {
            return Err(ParserError::Expected(Token::EqSign));
        }

        println!("{:?}", self.curr());

        let rhs = self.num_expr()?;

        Ok(Expr::Comparison(
            Box::new(Expr::Num(lhs)),
            Box::new(Expr::Num(rhs)),
        ))
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
    fn pos(&self) -> usize {
        self.cursor
    }

    /// Increments the `pos` field
    fn adv(&mut self) {
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
}
