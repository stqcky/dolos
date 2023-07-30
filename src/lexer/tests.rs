#[cfg(test)]
mod synthetic {
    use crate::lexer::lexer::{Lexer, Token};

    #[test]
    fn operators() {
        let lexer =
            Lexer::new("+ - * / ~ % ^ # == ~= <= >= < > = () {} [] ; : , . .. ...".to_string());
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
    fn strings() {
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

    #[test]
    fn numbers() {
        let lexer = Lexer::new(
            r#"
        1
        111
        11e111
        11E1
        11e+111
        11e-11
        22E+2
        22E-22
        1e5

        -1
        -111
        -11e11
        -11E1
        -11e+1
        -11e-11
        -22E+2
        -22E-22

        1.0
        11.01
        11.01e1
        11.01E11
        11.01e+1
        11.01e-11
        11.01E+1
        11.01E-11

        0xf
        0xabcdef
        0x11
        -0xf
        -0xabcdef
        -0x11

        0b1
        0b101
        -0b1
        -0b101
        "#
            .to_string(),
        );
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Number(1.0),
                Token::Number(111.0),
                Token::Number(1.1e112),
                Token::Number(110.0),
                Token::Number(1.1e112),
                Token::Number(1.1e-10),
                Token::Number(2200.0),
                Token::Number(2.2e-21),
                Token::Number(100000.0),
                Token::Number(-1.0),
                Token::Number(-111.0),
                Token::Number(-1100000000000.0),
                Token::Number(-110.0),
                Token::Number(-110.0),
                Token::Number(-1.1e-10),
                Token::Number(-2200.0),
                Token::Number(-2.2e-21),
                Token::Number(1.0),
                Token::Number(11.01),
                Token::Number(110.1),
                Token::Number(1101000000000.0),
                Token::Number(110.1),
                Token::Number(1.101e-10),
                Token::Number(110.1),
                Token::Number(1.101e-10),
                Token::Number(15.0),
                Token::Number(11259375.0),
                Token::Number(17.0),
                Token::Number(-15.0),
                Token::Number(-11259375.0),
                Token::Number(-17.0),
                Token::Number(1.0),
                Token::Number(5.0),
                Token::Number(-1.0),
                Token::Number(-5.0)
            ]
        );
    }
}

#[cfg(test)]
mod real {
    use crate::lexer::lexer::{Lexer, Token};

    #[test]
    fn real1() {
        let lexer = Lexer::new(
            r#"
            local t = {}
            local array = {1, 2, 3};
            local array_el = array[1]
            var_string = "string";
            local var_number = 222
            local var_number2 = 111
            local a = var_number - var_number2;

            -- comment 

            for k, v in pairs(t) do
                print(k, v)
            end

            local math = 1.0 * 2 + 0xff-0b100/1e5
            "#
            .to_string(),
        );

        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Local,
                Token::Identifier(String::from("t")),
                Token::Assignment,
                Token::LeftBracket,
                Token::RightBracket,
                Token::Local,
                Token::Identifier(String::from("array")),
                Token::Assignment,
                Token::LeftBracket,
                Token::Number(1.0),
                Token::Comma,
                Token::Number(2.0),
                Token::Comma,
                Token::Number(3.0),
                Token::RightBracket,
                Token::Semicolon,
                Token::Local,
                Token::Identifier(String::from("array_el")),
                Token::Assignment,
                Token::Identifier(String::from("array")),
                Token::LeftSquareBracket,
                Token::Number(1.0),
                Token::RightSquareBracket,
                Token::Identifier(String::from("var_string")),
                Token::Assignment,
                Token::String(String::from("string")),
                Token::Semicolon,
                Token::Local,
                Token::Identifier(String::from("var_number")),
                Token::Assignment,
                Token::Number(222.0),
                Token::Local,
                Token::Identifier(String::from("var_number2")),
                Token::Assignment,
                Token::Number(111.0),
                Token::Local,
                Token::Identifier(String::from("a")),
                Token::Assignment,
                Token::Identifier(String::from("var_number")),
                Token::Minus,
                Token::Identifier(String::from("var_number2")),
                Token::Semicolon,
                Token::Comment(String::from(" comment ")),
                Token::For,
                Token::Identifier(String::from("k")),
                Token::Comma,
                Token::Identifier(String::from("v")),
                Token::In,
                Token::Identifier(String::from("pairs")),
                Token::LeftParen,
                Token::Identifier(String::from("t")),
                Token::RightParen,
                Token::Do,
                Token::Identifier(String::from("print")),
                Token::LeftParen,
                Token::Identifier(String::from("k")),
                Token::Comma,
                Token::Identifier(String::from("v")),
                Token::RightParen,
                Token::End,
                Token::Local,
                Token::Identifier(String::from("math")),
                Token::Assignment,
                Token::Number(1.0),
                Token::Multiply,
                Token::Number(2.0),
                Token::Plus,
                Token::Number(255.0),
                Token::Number(-4.0),
                Token::Divide,
                Token::Number(100000.0)
            ]
        );
    }
}
