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
