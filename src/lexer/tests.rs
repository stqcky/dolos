#[cfg(test)]
mod tests {
    use crate::lexer::lexer::{Lexer, Token};

    #[test]
    fn operators() {
        let lexer = Lexer::new("+ - * / ~ % ^ # == ~= <= >= < > = () {} [] ;:,. .. ...".to_string());
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Plus,
                Token::Minus,
                Token::Multiply,
                Token::Divide,
                Token::Tilde,
                Token::Modulo,
                Token::Xor,
                Token::Length,
                Token::Equal,
                Token::NotEq,
                Token::LessThanOrEqual,
                Token::GreaterThanOrEqual,
                Token::LessThan,
                Token::GreaterThan,
                Token::Assignment,
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBracket,
                Token::RightBracket,
                Token::LeftSquareBracket,
                Token::RightSquareBracket,
                Token::Semicolon,
                Token::Colon,
                Token::Comma,
                Token::Period,
                Token::DoublePeriod,
                Token::TriplePeriod
            ]
        );
    }

    #[test]
    fn keywords() {
        let lexer = Lexer::new("and break do else elseif end false for function if in local nil not or repeat return then true until while".to_string());
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::And,
                Token::Break,
                Token::Do,
                Token::Else,
                Token::Elseif,
                Token::End,
                Token::False,
                Token::For,
                Token::Function,
                Token::If,
                Token::In,
                Token::Local,
                Token::Nil,
                Token::Not,
                Token::Or,
                Token::Repeat,
                Token::Return,
                Token::Then,
                Token::True,
                Token::Until,
                Token::While,
            ]
        );
    }

    #[test]
    fn strings_in_quotes() {
        let lexer = Lexer::new("local test = 'test'; local test2 = \"test2\";".to_string());
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Local,
                Token::Identifier("test".to_string()),
                Token::Assignment,
                Token::String("test".to_string()),
                Token::Semicolon,
                Token::Local,
                Token::Identifier("test2".to_string()),
                Token::Assignment,
                Token::String("test2".to_string()),
                Token::Semicolon
            ]
        );
    }
}
