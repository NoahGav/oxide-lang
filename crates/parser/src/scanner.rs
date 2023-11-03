use crate::lexer::{Lexer, Token, TokenKind};

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
        if self.src.is_empty() {
            Token {
                kind: TokenKind::Eoi,
                range: 0..0,
            }
        } else {
            let index = (self.cursor + offset).min(self.tokens.len() - 1);
            self.tokens[index].clone()
        }
    }

    pub fn eat(&mut self) -> Token {
        let token = self.peek(0);
        self.cursor += 1;
        token
    }

    pub fn next(&mut self) -> Token {
        self.cursor += 1;
        self.peek(0)
    }

    pub fn text(&self, token: &Token) -> &str {
        &self.src[token.range.clone()]
    }
}
