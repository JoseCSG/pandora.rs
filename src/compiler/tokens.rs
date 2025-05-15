use logos::Logos;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Default, Clone, Debug, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    InvalidFloat(ParseFloatError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

impl From<ParseFloatError> for LexicalError {
    fn from(err: ParseFloatError) -> Self {
        LexicalError::InvalidFloat(err)
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[token("program")]
    Program,
    #[token("main")]
    Main,
    #[token("end")]
    End,
    #[token("print")]
    Print,
    #[token("var")]
    Var,
    #[token("void")]
    Void,
    #[token("while")]
    While,
    #[token("do")]
    Do,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("int")]
    IntDatatype,
    #[token("float")]
    FloatDatatype,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Id(String),
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(i32),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f32),
    #[regex(r#""[^"\n]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    String(String),
    #[token("+")]
    OpAdd,
    #[token("-")]
    OpSub,
    #[token("/")]
    OpDiv,
    #[token("*")]
    OpMul,
    #[token(">")]
    Gt,
    #[token("<")]
    Lt,
    #[token("!=")]
    Ne,
    #[token("=")]
    Eq,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(".")]
    Dot,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    #[error]
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
