use std::ops::Range;

use crate::lexer;

#[derive(Debug)]
pub struct Tree {
    pub nodes: Vec<Result<Node>>,
}

// impl Tree {
//     pub fn text(&self, token: &Token) -> &str {
//         &self.src[token.range.clone()]
//     }
// }

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Debug,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Range<usize>,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Debug,
    Missing(Box<TokenKind>),
    Skipped(lexer::TokenKind),
    Delimiter(lexer::TokenKind),
    Keyword(lexer::TokenKind),
    Ident,
}

#[derive(Debug)]
pub enum Node {
    FnDecl(FnDecl),
}

#[derive(Debug)]
pub struct FnDecl {
    pub fn_keyword: Token,
    pub name: Token,
    pub inputs: Result<FnInputs>,
    // TODO: output.
    // TODO: body.
}

#[derive(Debug)]
pub struct FnInputs {
    pub l_paren: Token,
    pub inputs: Vec<FnInput>,
    pub r_paren: Token,
}

#[derive(Debug)]
pub struct FnInput {
    pub name: Token,
    pub colon: Token,
    pub r#type: Result<Type>,
    pub comma: Token,
}

#[derive(Debug)]
pub enum Type {
    Simple(Token),
}

impl Error {
    pub fn debug() -> Self {
        Self {
            kind: ErrorKind::Debug,
            tokens: vec![],
        }
    }
}

impl Token {
    pub fn debug() -> Self {
        Self {
            kind: TokenKind::Debug,
            range: 0..0,
        }
    }

    pub fn missing(expected: TokenKind, found: &lexer::Token) -> Self {
        Self {
            kind: TokenKind::Missing(Box::new(expected)),
            range: found.range.start..found.range.start,
        }
    }

    pub fn skip(skipped: &lexer::Token) -> Self {
        Self {
            kind: TokenKind::Skipped(skipped.kind),
            range: skipped.range.clone(),
        }
    }
}
