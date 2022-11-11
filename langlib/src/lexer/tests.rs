#[cfg(test)]
mod lexer_tokenizer_tests {

    use crate::lexer::{
        op::BinOp,
        token::{Keyword, Token},
        Lexer,
    };

    #[test]
    fn parse_token() {
        let plus = "+";

        let token = Lexer::parse_token(plus);

        assert!(token.is_ok());

        let token = token.unwrap();

        assert_eq!(token.0, Token::Op(BinOp::Add));

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
                Token::Keyword(Keyword::Let),
                Token::Ident("a".to_owned()),
                Token::DeclarationSign,
                Token::Int(3),
                Token::Semi,
                Token::Keyword(Keyword::Let),
                Token::Ident("b".to_owned()),
                Token::Op(BinOp::EqSign),
                Token::String("4".to_owned()),
                Token::Semi
            ]
        );
    }

    #[test]
    fn test_comparision_signs() {
        let eq = "==";

        let mut lexer = Lexer::new(eq);
        let result = lexer.tokenize();
        assert_eq!(result.unwrap(), vec![Token::Op(BinOp::EqSign)]);

        let neq = "!=";

        let mut lexer = Lexer::new(neq);
        let result = lexer.tokenize();
        assert_eq!(result.unwrap(), vec![Token::Op(BinOp::NeqSign)]);

        let g = ">";

        let mut lexer = Lexer::new(g);
        let result = lexer.tokenize();
        assert_eq!(result.unwrap(), vec![Token::Op(BinOp::GreaterSign)]);

        let geq = ">=";

        let mut lexer = Lexer::new(geq);
        let result = lexer.tokenize();
        assert_eq!(result.unwrap(), vec![Token::Op(BinOp::GreaterEqSign)]);

        let l = "<";

        let mut lexer = Lexer::new(l);
        let result = lexer.tokenize();
        assert_eq!(result.unwrap(), vec![Token::Op(BinOp::LessSign)]);

        let leq = "<=";

        let mut lexer = Lexer::new(leq);
        let result = lexer.tokenize();
        assert_eq!(result.unwrap(), vec![Token::Op(BinOp::LessEqSign)]);
    }
}
