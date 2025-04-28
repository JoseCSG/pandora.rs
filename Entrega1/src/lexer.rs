use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("program")]
    Program,
    #[token("main")]
    Main,
    #[token("end")]
    End,
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
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Id,
    #[regex("-?[0-9]+")]
    Number,
    #[regex(r"-?[0-9]+\.[0-9]+")]
    Float,
    #[regex(r#""[^"\n]*""#)]
    String,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,
    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token("!=")]
    NotEqual,
    #[token("=")]
    Equal,
    #[token(";")]
    Semicolon,
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

pub fn lex(source: &str) -> String {
    let lexer = Token::lexer(source);

    let mut result = String::new();
    for token in lexer {
        result.push_str(&format!("{:?} ", token));
        if token == Token::Semicolon || token == Token::RBrace {
            result.push('\n');
        }
    }
    return result;
}
