use std::ops::Range;

mod lexer;
mod syntax;

pub struct Parser<'src> {
    src: &'src str,
}

impl<'src> Parser<'src> {
    pub fn new(src: &'src str) -> Self {
        Self { src }
    }

    pub fn parse(self) -> syntax::Tree {
        // Ok, so the way parsing will work is like this.
        // We lookahead to determine what node to parse
        // For example, a FnDecl. We then transfer over
        // to parsing the FnDecl. A FnDecl is split into
        // sections (name, inputs, output, and body).
        // When parsing each section we either succeed
        // or we fail (with a syntax::Error). If it fails
        // it will consume all tokens until it reaches
        // on of the delimiters for that section (for
        // example, the inputs delimiter is the ")" token).
        // While we are doing this, we emit each lexer
        // Token, range, and syntax TokenKind. After the
        // node is fully parsed (even with errors), we
        // add one token to the tree's tokens list by
        // building a syntax token that includes the
        // syntax node's index. That's it.

        todo!()
    }
}

struct ParseContext<'src> {
    src: &'src str,
    tokens: Vec<ParseToken>,
}

impl<'src> ParseContext<'src> {
    fn fn_decl(self) -> (syntax::Node, Vec<ParseToken>) {
        // This is the entry point of parsing a fn decl. While
        // parsing we will emit parse tokens. This includes all
        // tokens, including skipped and missing tokens. When
        // we are done, we return the parsed syntax::Node and
        // the emitted parse tokens. The main parser then appends
        // the node to the tree (obtaining it's index) and then
        // converts the parse tokens to syntax tokens and extends
        // the tree tokens.

        todo!()
    }
}

struct ParseToken {
    kind: syntax::TokenKind,
    range: Range<usize>,
}

impl ParseToken {
    fn into(self, node: usize) -> syntax::Token {
        syntax::Token {
            kind: self.kind,
            range: self.range,
            node,
        }
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
