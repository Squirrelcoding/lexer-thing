pub mod err;
mod expr;
mod stmt;
mod tests;

use crate::stmt::Stmt;

use self::err::ParserError;

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

        while !self.is_at_end() {
            let stmt = self.stmt()?;

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

            if (!self.is_at_end()) && (&self.tokens[self.cursor] == token) {
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
            if self.is_at_end() {
                return false;
            }

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

        if self.curr() == Ok(Token::Semi) && !self.is_at_end() {
            self.cursor += 1;
        }
    }

    /// Returns the previous token
    fn prev(&self) -> Result<Token, ParserError> {
        if self.cursor == 0 {
            return Err(ParserError::InvalidTokenIndex(self.cursor));
        }

        Ok(self.tokens[self.cursor - 1].to_owned())
    }

    /// Returns the current token, if there is one.
    fn curr(&self) -> Result<Token, ParserError> {
        if self.is_at_end() {
            return Err(ParserError::UnexpectedEOF);
        }
        Ok(self.tokens[self.cursor].to_owned())
    }

    /// Returns the token at the given index `i`
    fn at(&self, i: usize) -> Result<Token, ParserError> {
        if i >= self.tokens.len() {
            return Err(ParserError::InvalidTokenIndex(i));
        }

        Ok(self.tokens[i].to_owned())
    }

    /// Returns a boolean indicating whether the position is at the end of the token stream.
    pub fn is_at_end(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    pub fn expect_consume(&mut self, tokens: &[Token]) -> Result<(), ParserError> {
        if !self.match_rule(tokens) {
            return Err(ParserError::FailedRuleMatch(tokens.to_vec(), self.cursor));
        }

        Ok(())
    }
}
