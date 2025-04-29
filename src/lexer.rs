use crate::tokens::{LexicalError, Token};
use logos::{Logos, SpannedIter};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
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
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| Ok((span.start, token, span.end)))
    }
}
