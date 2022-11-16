use super::{err::{ParserError}, Parser};
use crate::{
    expr::Expr,
    lexer::token::{Keyword, Token},
    stmt::{Declaration, Stmt},
};

impl Parser {
    /// Attempts to parse a statement from the next tokens until it encounters a semicolon.
    pub fn stmt(&mut self) -> Result<Stmt, ParserError> {
        // The index of the next semicolon.
        let idx = match (self.cursor..self.tokens.len())
            .into_iter()
            .find(|i| self.at(*i).unwrap() == Token::Semi)
        {
            Some(idx) => idx,
            None => return Err(ParserError::Expected(Token::Semi, self.cursor)),
        };

        // Tokens of the statement
        let slice = &self.tokens[self.cursor..idx];

        // Big ugly match expression that's very difficult to read
        match slice.iter().next() {
            // Check if there's even a token
            Some(token) => match token {
                // Match the keyword
                Token::Keyword(keyword) => match keyword {
                    // If it's an Declaration statement
                    Keyword::Let => self.declaration(),
                    Keyword::Print => self.print(),
                    Keyword::If => self.if_stmt(),
                    Keyword::While => self.while_stmt(),
                    Keyword::For => self.for_stmt(),

                    _ => Err(ParserError::BadStatement(self.cursor)),
                },

                Token::LeftCurly => self.block(),
                Token::Ident(_) => {
                    if self.at(self.cursor + 1)? != Token::LeftBracket {
                        self.assignment()
                    } else {
                        Ok(Stmt::Expr(self.expr()?))
                    }
                }

                // Attempt to parse an expression statement
                _ => match self.expr() {
                    Ok(expr) => Ok(Stmt::Expr(expr)),
                    Err(err) => Err(err),
                },
            },

            // Reset position
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    /// Attempts to parse a declaration statement.
    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.match_rule(&[Token::Keyword(Keyword::Let), Token::Ident(String::from(""))]) {
            if self.match_rule(&[Token::DeclarationSign]) {
                // Get the identier and value
                let ident = self.at(self.cursor - 2)?.try_into_ident()?;

                let expr = self.expr()?;

                return Ok(Stmt::Declaration(Declaration { ident, val: expr }));
            }

            let ident = self.at(self.cursor - 2)?.try_into_ident()?;

            // Set the variable to null by default;
            return Ok(Stmt::Declaration(Declaration {
                ident,
                val: Expr::Null,
            }));
        }

        Err(ParserError::Expected(
            Token::Keyword(Keyword::Let),
            self.cursor,
        ))
    }

    /// Attempts to parse a print statement.
    fn print(&mut self) -> Result<Stmt, ParserError> {
        if self.match_rule(&[Token::Keyword(Keyword::Print)]) {
            let expr = self.expr()?;

            return Ok(Stmt::Print(expr));
        }

        // Reset position
        Err(ParserError::BadStatement(self.cursor))
    }

    /// Attempts to parse a block.
    fn block(&mut self) -> Result<Stmt, ParserError> {
        // Advance from the "{" token.
        self.adv();

        let mut stmts = Vec::new();

        while self.curr() != Ok(Token::RightCurly) {
            let stmt = self.stmt()?;

            stmts.push(stmt);
        }

        // Advance from the "}" token.
        self.adv();

        Ok(Stmt::Block(stmts))
    }

    /// Attempts to parse an if statement.
    fn if_stmt(&mut self) -> Result<Stmt, ParserError> {
        if !self.match_rule(&[Token::Keyword(Keyword::If), Token::LeftBracket]) {
            return Err(ParserError::Expected(Token::LeftBracket, self.cursor));
        }

        let expr = self.expr()?;

        if !self.match_rule(&[Token::RightBracket]) {
            return Err(ParserError::Expected(Token::RightBracket, self.cursor));
        }

        // If we execute the block inside then that means we may have reached a line of code that looks like:
        // if (condition) statement;
        if self.curr()? != Token::LeftCurly {
            let stmt = self.stmt()?;

            return Ok(Stmt::If(expr, Box::new(stmt), None));
        }

        let block = self.block()?;

        if self.match_rule(&[Token::Keyword(Keyword::Else)]) {
            let else_block = self.block()?;
            Ok(Stmt::If(expr, Box::new(block), Some(Box::new(else_block))))
        } else {
            Ok(Stmt::If(expr, Box::new(block), None))
        }
    }

    /// Attempts to parse a while loop
    fn while_stmt(&mut self) -> Result<Stmt, ParserError> {
        if !self.match_rule(&[Token::Keyword(Keyword::While), Token::LeftBracket]) {
            return Err(ParserError::Expected(Token::LeftBracket, self.cursor));
        }

        let expr = self.expr()?;

        if !self.match_rule(&[Token::RightBracket]) {
            return Err(ParserError::Expected(Token::RightBracket, self.cursor));
        }

        let block = self.block()?;

        Ok(Stmt::While(expr, Box::new(block)))
    }

    /// Attempts to parse an assignment.
    fn assignment(&mut self) -> Result<Stmt, ParserError> {
        if self.match_rule(&[Token::Ident(String::from(""))]) {
            let ident = self.prev()?;

            if self.match_rule(&[Token::DeclarationSign]) {
                let expr = self.expr()?;

                return Ok(Stmt::Assignment(Declaration {
                    ident: ident.try_into_ident()?,
                    val: expr,
                }));
            }
            return Err(ParserError::Expected(Token::DeclarationSign, self.cursor));
        }

        Err(ParserError::UnexpectedToken(
            self.prev()?,
            self.cursor,
        ))
    }

    /// Attempt to parse a for loop, by parsing it into a while loop.
    fn for_stmt(&mut self) -> Result<Stmt, ParserError> {
        if !self.match_rule(&[Token::Keyword(Keyword::For), Token::LeftBracket]) {
            return Err(ParserError::Expected(
                Token::Keyword(Keyword::For),
                self.cursor,
            ));
        }

        let initializer = self.stmt()?;

        let condition = self.expr()?;
        let increment = self.stmt()?;

        if !self.match_rule(&[Token::RightBracket]) {
            return Err(ParserError::Expected(Token::RightBracket, self.cursor));
        }

        let mut stmts = match self.block()? {
            Stmt::Block(stmts) => stmts,
            _ => todo!(),
        };

        stmts.push(increment);

        Ok(Stmt::Block(vec![
            initializer,
            Stmt::While(condition, Box::new(Stmt::Block(stmts))),
        ]))
    }
}
