use logos::Logos;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LexerError {}

impl From<logos::Lexer<'_, Token>> for LexerError {
    fn from(value: logos::Lexer<Token>) -> Self {
        println!("santehuasntoesnth");
        println!("lexer error {:?}", value);

        LexerError {}
    }
}

impl From<Token> for LexerError {
    fn from(value: Token) -> Self {
        println!("santehuasntoesnth");
        println!("lexer error {:?}", value);

        LexerError {}
    }
}

fn number(lex: &mut logos::Lexer<Token>) -> Option<f64> {
    let number = lex.slice().parse::<f64>();

    match number {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn radix_number(lex: &mut logos::Lexer<Token>) -> Option<f64> {
    let slice = lex.slice();

    if let Some(s) = slice.strip_prefix("0x") {
        if let Ok(num) = i64::from_str_radix(s, 16) {
            return Some(num as f64);
        }
    } 

    if let Some(s) = slice.strip_prefix("-0x") {
        if let Ok(num) = i64::from_str_radix(s, 16) {
            return Some((num * -1) as f64);
        }
    } 

    if let Some(s) = slice.strip_prefix("0b") {
        if let Ok(num) = i64::from_str_radix(s, 2) {
            return Some(num as f64);
        }
    } 

    if let Some(s) = slice.strip_prefix("-0b") {
        if let Ok(num) = i64::from_str_radix(s, 2) {
            return Some((num * -1) as f64);
        }
    } 

    None
}

fn string(lex: &mut logos::Lexer<Token>) -> String {
    let slice = lex.slice();

    slice.trim_matches('\"').trim_matches('\'').to_string()
}

fn comment(lex: &mut logos::Lexer<Token>) -> Option<String> {
    match lex.slice().strip_prefix("--") {
        Some(v) => Some(v.to_string()),
        None => None
    }
}

#[derive(Clone, Default, Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(error = LexerError)]
pub enum Token {
    #[default]
    Invalid,

    // Keywords
    #[token("and")]
    And,
    #[token("break")]
    Break,
    #[token("do")]
    Do,
    #[token("else")]
    Else,
    #[token("elseif")]
    Elseif,
    #[token("end")]
    End,
    #[token("false")]
    False,
    #[token("for")]
    For,
    #[token("function")]
    Function,
    #[token("if")]
    If,
    #[token("in")]
    In,
    #[token("local")]
    Local,
    #[token("nil")]
    Nil,
    #[token("not")]
    Not,
    #[token("or")]
    Or,
    #[token("repeat")]
    Repeat,
    #[token("return")]
    Return,
    #[token("then")]
    Then,
    #[token("true")]
    True,
    #[token("until")]
    Until,
    #[token("while")]
    While,

    // Operators
    #[token("=")]
    Assignment,
    #[token("~")]
    Tilde,
    #[token("==")]
    Equal,
    #[token("~=")]
    NotEq,
    #[token("<=")]
    LessThanOrEqual,
    #[token(">=")]
    GreaterThanOrEqual,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("^")]
    Xor,
    #[token("#")]
    Length,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBracket,
    #[token("}")]
    RightBracket,
    #[token("[")]
    LeftSquareBracket,
    #[token("]")]
    RightSquareBracket,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Period,
    #[token("..")]
    DoublePeriod,
    #[token("...")]
    TriplePeriod,

    #[regex(r"-?[0-9]+[eE]?[+-]?\.?[0-9]*[eE]?[+-]?[0-9]*", number)]
    #[regex(r"-?0x[a-fA-F0-9]+", radix_number)]
    #[regex(r"-?0b[01]+", radix_number)]
    Number(f64),

    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, string)]
    #[regex(r#"'([^'\\]|\\['\\bnfrt]|u[a-fA-F0-9]{4})*'"#, string)]
    String(String),

    #[regex(r#"--.*"#, comment)]
    Comment(String)
}

pub struct Lexer {
    code: String,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer { code }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let tokens = Token::lexer(&self.code);

        tokens
            .map(|token| match token {
                Ok(v) => v,
                Err(e) => {
                    panic!("error lexing {:?}", e);
                }
            })
            .collect()
    }
}
