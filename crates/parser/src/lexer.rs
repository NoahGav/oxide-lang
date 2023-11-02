use std::ops::Range;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub enum TokenKind {
    Unknown,
    Whitespace,
    Ident,
    /// e.g. 42
    IntLiteral,
    /// let
    Let,
    /// fn
    Fn,
    /// &
    Amp,
    /// ;
    SemiColon,
    /// ,
    Comma,
    /// (
    LParen,
    /// )
    RParen,
    /// =
    Eq,
    /// +
    Plus,
    /// =>
    FatArrow,
}

impl Token {
    fn new(kind: TokenKind, range: Range<usize>) -> Self {
        Self { kind, range }
    }
}

pub struct Lexer<'src> {
    src: &'src str,
    cursor: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self { src, cursor: 0 }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();

            match token {
                Some(token) => tokens.push(token),
                None => return tokens,
            }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        macro_rules! token {
            ($kind:expr) => {
                Some(Token::new($kind, self.cursor - 1..self.cursor))
            };
        }

        macro_rules! token_and_pop {
            ($kind:expr, $offset:expr) => {{
                self.cursor += 1;
                Some(Token::new($kind, self.cursor - $offset..self.cursor))
            }};
        }

        match self.peek() {
            Some(c) => match c {
                '(' => token_and_pop!(TokenKind::LParen, 1),
                ')' => token_and_pop!(TokenKind::RParen, 1),
                ',' => token_and_pop!(TokenKind::Comma, 1),
                ';' => token_and_pop!(TokenKind::SemiColon, 1),
                '+' => token_and_pop!(TokenKind::Plus, 1),
                '&' => token_and_pop!(TokenKind::Amp, 1),
                '=' => match self.next() {
                    Some('>') => token_and_pop!(TokenKind::FatArrow, 2),
                    _ => token!(TokenKind::Eq),
                },
                c => {
                    if c.is_ascii_whitespace() {
                        return Some(self.whitespace());
                    }

                    if c.is_ascii_digit() {
                        return Some(self.int_literal());
                    }

                    if c == '_' || c.is_ascii_alphabetic() {
                        return Some(self.identifier_or_keyword());
                    }

                    token_and_pop!(TokenKind::Unknown, 1)
                }
            },
            None => None,
        }
    }

    fn whitespace(&mut self) -> Token {
        let start = self.cursor;

        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.cursor += 1;
            } else {
                break;
            }
        }

        Token::new(TokenKind::Whitespace, start..self.cursor)
    }

    fn int_literal(&mut self) -> Token {
        let start = self.cursor;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.cursor += 1;
            } else {
                break;
            }
        }

        Token::new(TokenKind::IntLiteral, start..self.cursor)
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let start = self.cursor;

        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.cursor += 1;
            } else {
                break;
            }
        }

        match &self.src[start..self.cursor] {
            "let" => Token::new(TokenKind::Let, start..self.cursor),
            "fn" => Token::new(TokenKind::Fn, start..self.cursor),
            _ => Token::new(TokenKind::Ident, start..self.cursor),
        }
    }

    fn nth(&self, n: usize) -> Option<char> {
        self.src.as_bytes().get(n).map(|c| *c as char)
    }

    fn peek(&self) -> Option<char> {
        self.nth(self.cursor)
    }

    fn next(&mut self) -> Option<char> {
        self.cursor += 1;
        self.peek()
    }
}
