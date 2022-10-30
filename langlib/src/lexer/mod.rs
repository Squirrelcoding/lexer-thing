pub mod op;
pub mod token;

use self::{op::Op, token::Token};
use std::num::IntErrorKind;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,      // Source code
    pub position: usize, // Reading position
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer` instance
    pub fn new(input: &'a str) -> Lexer {
        Self { input, position: 0 }
    }

    /// Tokenizes a string`
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut vec = Vec::new();

        while let Ok((tok, _)) = self.next_token() {
            vec.push(tok);
        }

        Ok(vec)
    }

    /// Returns the next token with its length
    fn next_token(&mut self) -> Result<(Token, usize), LexerError> {
        self.skip_whitespace()?;

        if self.input.len() - self.position == 0 {
            return Err(LexerError::UnexpectedEOF);
        }

        let (token, len) = Lexer::parse_token(&self.input[self.position..])?;
        self.position += len;

        Ok((token, len))
    }

    /// Attempts to parse a single token
    fn parse_token(data: &str) -> Result<(Token, usize), LexerError> {
        let next = match data.chars().next() {
            Some(c) => c,
            None => return Err(LexerError::InvalidToken),
        };

        match next {
            '+' => Ok((Token::Op(Op::Add), 1)),
            '-' => Ok((Token::Op(Op::Sub), 1)),
            '*' => Ok((Token::Op(Op::Mul), 1)),
            '/' => Ok((Token::Op(Op::Div), 1)),
            '=' => {
                if Some('=') == data.chars().nth(1) {
                    return Ok((Token::EqSign, 2));
                }
                Ok((Token::AssignmentSign, 1))
            }
            '(' => Ok((Token::LeftBracket, 1)),
            ')' => Ok((Token::RightBracket, 1)),
            ';' => Ok((Token::Semi, 1)),
            '"' | '\'' => Lexer::tokenize_string(data),
            '0'..='9' => Lexer::tokenize_num(data),
            _ => Lexer::tokenize_word(data),
        }
    }

    /// Attempts to tokenize a string
    fn tokenize_string(data: &str) -> Result<(Token, usize), LexerError> {
        let quote = match data.chars().next() {
            Some(c) => match c {
                '\'' => '\'',
                '"' => '"',
                _ => {
                    return Err(LexerError::InvalidToken);
                }
            },
            None => return Err(LexerError::UnexpectedEOF),
        };

        let (string, len) = Lexer::take_while(&data[1..], |c| c != quote)?;

        // Case where no closing quote was found
        if len == data.len() {
            return Err(LexerError::Expected(quote));
        }

        // Add 2 to the len for some weird edge case
        Ok((Token::String(string), len + 2))
    }

    /// Attempt to tokenize a "word", which could be an identifier or a keyword.
    fn tokenize_word(data: &str) -> Result<(Token, usize), LexerError> {
        // Check if the word starts with valid character
        match data.chars().next() {
            Some(ch) if ch.is_ascii_digit() => panic!("Identifiers can't start with a number"),
            None => panic!(),
            _ => {}
        };

        // take until we encounter a whitespace
        let (word, len) = Lexer::take_while(data, |c| c.is_alphanumeric())?;

        let word = match word.as_str() {
            "let" => Token::Keyword("let".to_owned()),
            "true" => Token::Keyword("true".to_owned()),
            "false" => Token::Keyword("false".to_owned()),
            s => Token::Ident(s.to_owned()),
        };

        // match ident

        Ok((word, len))
    }

    /// Attemps to tokenize a number
    fn tokenize_num(data: &str) -> Result<(Token, usize), LexerError> {
        // take_while will try to return
        match Lexer::take_while(data, |c| c.is_ascii_digit()) {
            // If it's ok we need to try to parse the number
            Ok((num_string, len)) => match num_string.parse::<i32>() {
                Ok(num) => Ok((Token::Int(num), len)),
                Err(err) => Err(LexerError::IntError(err.kind().to_owned())),
            },

            Err(error) => Err(error),
        }
    }

    /// Returns a substring with a sequence of characters starting at 0 which have satisfied the given predicate.
    fn take_while<F>(s: &str, predicate: F) -> Result<(String, usize), LexerError>
    where
        F: Fn(char) -> bool,
    {
        let x = s
            .char_indices()
            .find_map(|(idx, char)| {
                if predicate(char) {
                    return None;
                }

                Some(idx)
            })
            .unwrap_or(s.len());

        Ok((s[..x].to_owned(), x))
    }

    /// If there are any whitespaces in the input, skip them by incrementing the `position` field.
    fn skip_whitespace(&mut self) -> Result<(), LexerError> {
        loop {
            match self.input.chars().nth(self.position) {
                Some(c) => {
                    if c.is_whitespace() {
                        self.position += 1;
                    } else {
                        break;
                    }
                }
                None => return Err(LexerError::UnexpectedEOF),
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum LexerError {
    #[error("Failed to parse int")]
    IntError(IntErrorKind),
    #[error("Invalid token encountered")]
    InvalidToken,
    #[error("Unexpected EOF encountered")]
    UnexpectedEOF,
    #[error("Expected '{0}'")]
    Expected(char),
}

#[cfg(test)]
mod lexer_tokenizer_tests {

    use super::*;

    #[test]
    fn parse_token() {
        let plus = "+";

        let token = Lexer::parse_token(plus);

        assert!(token.is_ok());

        let token = token.unwrap();

        assert_eq!(token.0, Token::Op(Op::Add));

        let num = "123456789";

        let num_token = Lexer::parse_token(num);

        assert!(num_token.is_ok());

        let num_token = num_token.unwrap();

        assert_eq!(num_token.0, Token::Int(123456789));

        let overflow_num = "123456785435643829043568";

        let overflow_num_token = Lexer::parse_token(overflow_num);

        assert!(overflow_num_token.is_err());
    }

    #[test]
    fn take_while() {
        let text = "thisIsSomeSampleText! this next sentence will not be read :(";

        let s = Lexer::take_while(text, |c| c.is_alphanumeric());

        assert!(s.is_ok());

        let s = s.unwrap();

        assert_eq!(s.0, "thisIsSomeSampleText");
    }

    #[test]
    fn skip_whitespace() {
        let mut lexer_tokenizer = Lexer::new("         Hello, world!");

        assert!(lexer_tokenizer.skip_whitespace().is_ok());

        assert_eq!(
            &lexer_tokenizer.input[lexer_tokenizer.position..],
            "Hello, world!"
        );
    }

    #[test]
    fn test_tokenize() {
        let s = "let a = 3; let b == \"4\";";

        let mut lexer = Lexer::new(s);

        let tokens = lexer.tokenize();

        assert!(tokens.is_ok());

        let tokens = tokens.unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Keyword("let".to_owned()),
                Token::Ident("a".to_owned()),
                Token::AssignmentSign,
                Token::Int(3),
                Token::Semi,
                Token::Keyword("let".to_owned()),
                Token::Ident("b".to_owned()),
                Token::EqSign,
                Token::String("4".to_owned()),
                Token::Semi
            ]
        );
    }
}
