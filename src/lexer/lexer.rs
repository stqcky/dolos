use std::{
    collections::HashMap,
    iter::Peekable,
    str::{Chars, Lines},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    And,
    Break,
    Do,
    Else,
    Elseif,
    End,
    False,
    For,
    Function,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,

    // Operators
    Assignment,
    Squigly,
    Equal,
    NotEq,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Xor,
    Length,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftSquareBracket,
    RightSquareBracket,
    Semicolon,
    Colon,
    Comma,
    Period,
    DoublePeriod,
    TriplePeriod,

    Number(String),
    Identifier(String),
}

pub struct Lexer {
    code: String,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer { code }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let lines = self.code.lines();

        let mut tokens = vec![];

        for (line_number, line) in lines.enumerate() {
            let mut chars = line.chars().peekable();

            while let Some(char) = chars.next() {
                let potentially_operator = self.get_operator(char, &mut chars);

                match potentially_operator {
                    Some(operator) => {
                        tokens.push(operator);
                        continue;
                    }
                    None => {}
                };

                let potentially_word = self.get_word(char, &mut chars);

                match potentially_word {
                    Some(word) => {
                        let keyword = self.get_keyword(&word);

                        match keyword {
                            Some(token) => {
                                tokens.push(token);
                            }
                            None => tokens.push(Token::Identifier(word)),
                        }
                    }
                    None => {}
                }
            }
        }

        tokens
    }

    fn is_operator_possibly_long(&self, token: &Token) -> bool {
        match token {
            Token::Assignment => true,
            Token::Not => true,
            Token::LessThan => true,
            Token::GreaterThan => true,
            Token::Period => true,
            _ => false,
        }
    }

    fn get_long_operator(&self, op: &Token, chars: &mut Peekable<Chars>) -> Option<Token> {
        if !self.is_operator_possibly_long(&op) {
            return None
        }

        let next_char = chars.peek();

        let possibly_long_op = match next_char {
            Some(next_char) => match (op, next_char) {
                (Token::Assignment, '=') => Some(Token::Equal),
                (Token::Not, '=') => Some(Token::NotEq),
                (Token::LessThan, '=') => Some(Token::LessThanOrEqual),
                (Token::GreaterThan, '=') => Some(Token::GreaterThanOrEqual),
                (Token::Period, '.') => {
                    chars.next();
                    let next_char2 = chars.peek();

                    if let Some(ch) = next_char2 {
                        match ch {
                            '.' => {
                                chars.next();
                                return Some(Token::TriplePeriod)
                            },
                            _ => Some(Token::DoublePeriod)
                        }
                    } else {
                        Some(Token::DoublePeriod)
                    }
                }
                _ => None
            },
            None => panic!("expected long operator, got nothing")
        };

        match possibly_long_op {
            Some(long_op) => {
                chars.next();
                Some(long_op)
            },
            None => None
        }
    }

    fn get_operator(&self, ch: char, chars: &mut Peekable<Chars>) -> Option<Token> {
        let possibly_operator = match ch {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '%' => Some(Token::Modulo),
            '^' => Some(Token::Xor),
            '#' => Some(Token::Length),
            '<' => Some(Token::LessThan),
            '>' => Some(Token::GreaterThan),
            '=' => Some(Token::Assignment),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBracket),
            '}' => Some(Token::RightBracket),
            '[' => Some(Token::LeftSquareBracket),
            ']' => Some(Token::RightSquareBracket),
            ';' => Some(Token::Semicolon),
            ':' => Some(Token::Colon),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Period),
            '~' => Some(Token::Not),
            _ => None,
        };

        match possibly_operator {
            Some(operator) => {
                let possibly_long_op = self.get_long_operator(&operator, chars);

                match possibly_long_op {
                    Some(long_op) => Some(long_op),
                    None => Some(operator)
                }
            }
            None => None,
        }
    }

    fn get_word(&self, ch: char, chars: &mut Peekable<Chars>) -> Option<String> {
        if !ch.is_ascii_alphabetic() {
            return None;
        }

        let next_word: String = chars.take_while(|ch| ch.is_ascii_alphanumeric()).collect();

        Some(ch.to_string() + &next_word)
    }

    fn get_keyword(&self, word: &str) -> Option<Token> {
        match word {
            "and" => Some(Token::And),
            "break" => Some(Token::Break),
            "do" => Some(Token::Do),
            "else" => Some(Token::Else),
            "elseif" => Some(Token::Elseif),
            "end" => Some(Token::End),
            "false" => Some(Token::False),
            "for" => Some(Token::For),
            "function" => Some(Token::Function),
            "if" => Some(Token::If),
            "in" => Some(Token::In),
            "local" => Some(Token::Local),
            "nil" => Some(Token::Nil),
            "not" => Some(Token::Not),
            "or" => Some(Token::Or),
            "repeat" => Some(Token::Repeat),
            "return" => Some(Token::Return),
            "then" => Some(Token::Then),
            "true" => Some(Token::True),
            "until" => Some(Token::Until),
            "while" => Some(Token::While),
            _ => None,
        }
    }
}
