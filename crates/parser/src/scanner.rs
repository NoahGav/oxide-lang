use crate::lexer::{Lexer, Token};

pub struct Scanner<'src> {
    src: &'src str,
    tokens: Vec<Token>,
    cursor: usize,
}

impl<'src> Scanner<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            tokens: Lexer::new(src).tokenize(),
            cursor: 0,
        }
    }

    pub fn peek(&self, offset: usize) -> Token {
        let index = (self.cursor + offset).min(self.src.len() - 1);
        self.tokens[index].clone()
    }

    pub fn eat(&mut self) -> Token {
        let token = self.peek(0);
        self.cursor += 1;
        token
    }

    pub fn text(&self, token: &Token) -> &str {
        &self.src[token.range.clone()]
    }
}
