use std::ops::Range;

use crate::lexer;

#[derive(Debug)]
pub struct Tree {
    pub tokens: Vec<Token>,
    nodes: Vec<Node>,
}

impl Tree {
    pub fn node_for(&self, token: &Token) -> &Node {
        &self.nodes[token.node]
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Range<usize>,
    /// The index used to get the associated [Node] from the [Tree].
    pub(crate) node: usize,
}

#[derive(Debug)]
pub enum TokenKind {
    Missing(Box<TokenKind>),
    Skipped,
    Whitespace,
    Delimiter(lexer::TokenKind),
    FnName,
}

#[derive(Debug)]
pub struct Error;

#[derive(Debug)]
pub enum Node {
    FnDecl(FnDecl),
}

#[derive(Debug)]
pub struct FnDecl {
    pub name: Result<String, Error>,
    pub inputs: Result<FnInputs, Error>,
    pub output: Result<Type, Error>,
    pub body: Result<FnBody, Error>,
}

#[derive(Debug)]
pub struct FnInputs;

#[derive(Debug)]
pub struct Type;

#[derive(Debug)]
pub struct FnBody;

// use std::rc::Rc;

// use crate::lexer;

// #[derive(Debug)]
// pub enum Node {
//     FnDecl(FnDecl),
// }

// #[derive(Debug)]
// pub struct Error;

// #[derive(Debug)]
// pub struct Type;

// #[derive(Debug)]
// pub struct FnDecl {
//     pub name: Result<String, Error>,
//     pub inputs: Result<FnInputs, Error>,
//     pub output: Result<Type, Error>,
//     pub body: Result<FnBody, Error>,
// }

// #[derive(Debug)]
// pub struct FnInputs;

// #[derive(Debug)]
// pub struct FnBody;

// #[derive(Debug)]
// pub struct Token {
//     pub inner: lexer::Token,
//     pub node: Rc<Node>,
//     pub kind: TokenKind,
// }

// #[derive(Debug, Clone)]
// pub enum TokenKind {
//     Missing,
//     Whitespace,
//     FnName,
// }
