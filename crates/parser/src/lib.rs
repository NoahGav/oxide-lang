mod lexer;
mod syntax;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(self) -> syntax::Tree {
        todo!()
    }
}

// Ok, now that I have an idea of how it should work. I can refactor everything.
// The general design is fairly simple. We build a list of syntax::Tokens where
// a syntax token is like a regular token, but is more specific. For example,
// a regular token might be an ident, but it's a fn's name. A syntax token also
// references the node it's a part of (directly). So, the fn name token would
// reference the FnDecl node. The parser actually returns a syntax::TokenStream
// which is just a list of syntax tokens and their referenced syntax nodes.
// This is the building block of the rest of the compiler. Instead of traversing
// the syntax tree directly (in most cases) we actually start with the token
// stream. Also, when parsing a section (fn name, fn inputs, fn output, ...) and
// we encounter an error, we simply just eat all tokens until reaching the
// delimiter and emit a syntax::Error instead of a syntax::Node. The error will
// contain the reason for that specific error, but none after it. This approach
// is better because it's faster, simpler, and gives better error messages in
// general as attempting to parse the syntax tree with errors normally just
// gives garbage errors. Also, when a token is missing, the syntax token is
// still created, but with the span of where the token should've started and
// with the missing flag set to true.

// use std::{iter::Peekable, rc::Rc};

// mod lexer;
// mod syntax;

// pub struct Parser<'src> {
//     input: Peekable<lexer::TokenStream<'src>>,
//     nodes: Vec<Rc<syntax::Node>>,
//     tokens: Vec<syntax::Token>,
// }

// impl<'src> Parser<'src> {
//     pub fn new(src: &'src str) -> Self {
//         let input: lexer::TokenStream = src.into();

//         Self {
//             input: input.peekable(),
//             nodes: vec![],
//             tokens: vec![],
//         }
//     }

//     pub fn parse(mut self) {
//         while let Some(token) = self.input.peek() {
//             match token.kind {
//                 lexer::TokenKind::Fn => self.fn_decl(),
//                 _ => {}
//             }
//         }

//         println!("{:#?}", self.tokens);
//         println!("{:#?}", self.nodes);
//     }

//     fn skip_whitespace(&mut self, ctx: &mut ParseContext) {
//         while let Some(token) = self.input.peek() {
//             if token.kind == lexer::TokenKind::Whitespace {
//                 ctx.token(token.clone(), syntax::TokenKind::Whitespace);
//                 self.input.next();
//             } else {
//                 break;
//             }
//         }
//     }

//     fn ident(
//         &mut self,
//         ctx: &mut ParseContext,
//         kind: syntax::TokenKind,
//     ) -> Result<String, syntax::Error> {
//         self.skip_whitespace(ctx);

//         if let Some(token) = self.input.next() {
//             match token.kind {
//                 lexer::TokenKind::Ident => {
//                     ctx.token(token, kind);
//                     Ok(self.tokens.);
//                 }
//                 _ => {
//                     ctx.token(
//                         // TODO: A Missing token.
//                         lexer::Token {
//                             kind: lexer::TokenKind::Unknown,
//                             range: token.range,
//                         },
//                         syntax::TokenKind::Missing,
//                     );

//                     return Err(syntax::Error);
//                 }
//             }
//         } else {
//             ctx.token(
//                 // TODO: A Missing token with the range being the end of the src.
//                 lexer::Token {
//                     kind: lexer::TokenKind::Unknown,
//                     range: 0..0,
//                 },
//                 syntax::TokenKind::Missing,
//             );

//             Err(syntax::Error)
//         }
//     }

//     fn fn_decl(&mut self) {
//         let mut ctx = ParseContext::new();

//         let node = Rc::new(syntax::Node::FnDecl(syntax::FnDecl {
//             name: self.ident(&mut ctx),
//             inputs: Err(syntax::Error),
//             output: Err(syntax::Error),
//             body: Err(syntax::Error),
//         }));

//         ctx.finish(node, self);
//     }
// }

// struct ParseContext {
//     tokens: Vec<(lexer::Token, syntax::TokenKind)>,
// }

// impl ParseContext {
//     fn new() -> Self {
//         Self { tokens: vec![] }
//     }

//     fn token(&mut self, token: lexer::Token, kind: syntax::TokenKind) {
//         self.tokens.push((token, kind));
//     }

//     fn finish(self, node: Rc<syntax::Node>, parser: &mut Parser) {
//         parser.nodes.push(node.clone());

//         parser.tokens.extend(
//             self.tokens
//                 .iter()
//                 .cloned()
//                 .map(|(inner, kind)| syntax::Token {
//                     inner,
//                     node: node.clone(),
//                     kind,
//                 }),
//         );
//     }
// }
