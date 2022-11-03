use crate::{
    lexer::token::Token,
    stmt::{Assignment, Stmt},
};

use super::{error::ParserError, Parser};

impl Parser {
    /// Attempts to parse a statement from some tokens
    pub fn stmt(&mut self) -> Result<Stmt, ParserError> {
        // The index of the next semicolon.
        let idx = match (self.cursor..self.tokens.len())
            .into_iter()
            .find(|i| self.at(*i).unwrap() == Token::Semi)
        {
            Some(idx) => idx,
            None => return Err(ParserError::Expected(Token::Semi)),
        };

        // Tokens of the statement
        let slice = &self.tokens[self.cursor..idx];

        // Big ugly match expression that's very difficult to read
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

                    _ => Err(ParserError::BadStatement),
                },
                _ => Err(ParserError::BadStatement),
            },
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    /// Attempts to match an assignment
    pub fn assignment(&mut self) -> Result<Stmt, ParserError> {
        // Check if it starts with a `let` keyword
        if !self.match_rule(&[Token::Keyword("let".to_owned())]) {
            return Err(ParserError::Expected(Token::Keyword("let".to_owned())));
        }

        // Identifier
        let ident = self.curr().try_into_ident()?;

        // Advance because we didn't advance for the identifier
        self.adv();

        // Check if there follows an equals sign
        if !self.match_rule(&[Token::AssignmentSign]) {
            return Err(ParserError::Expected(Token::AssignmentSign));
        }

        let expr = self.expr()?;

        Ok(Stmt::Assignment(Assignment { ident, val: expr }))
    }
}
