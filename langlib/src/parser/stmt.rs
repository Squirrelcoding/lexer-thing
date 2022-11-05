use crate::{
    lexer::token::{Keyword, Token},
    stmt::{Assignment, Stmt},
};

use super::{error::ParserError, Parser};

impl Parser {
    /// Attempts to parse a statement from the next tokens until it encounters a semicolon.
    pub fn stmt(&mut self) -> Result<Stmt, ParserError> {
        let curr = self.cursor;

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
                Token::Keyword(keyword) => match keyword {
                    // If it's an assignment statement
                    Keyword::Let => {
                        // Then check if these things apply
                        if self.match_rule(&[
                            Token::Keyword(Keyword::Let),
                            Token::Ident("".to_owned()),
                            Token::AssignmentSign,
                        ]) {
                            // Get the identier and value
                            let ident = self.at(self.cursor - 2)?.try_into_ident()?;

                            let expr = self.expr()?;

                            return Ok(Stmt::Assignment(Assignment { ident, val: expr }));
                        }

                        // Reset position
                        self.cursor = curr;
                        Err(ParserError::BadStatement)
                    }
                    Keyword::Print => {
                        if self.match_rule(&[Token::Keyword(Keyword::Print)]) {
                            if let Token::String(string) = self.curr() {

                                self.adv();

                                return Ok(Stmt::Print(string));
                            }
                        }

                        // Reset position
                        Err(ParserError::BadStatement)
                    }

                    // Reset position
                    _ => Err(ParserError::BadStatement),
                },

                // Reset position
                _ => Err(ParserError::BadStatement),
            },

            // Reset position
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    /// Attempts to match an assignment
    pub fn assignment(&mut self) -> Result<Stmt, ParserError> {
        // Check if it starts with a `let` keyword
        if !self.match_rule(&[Token::Keyword(Keyword::Let)]) {
            return Err(ParserError::Expected(Token::Keyword(Keyword::Let)));
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
