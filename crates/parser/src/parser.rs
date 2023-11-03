// Ok, I don't think this is the right approach. There's a few problems.
// 1). It's very verbose.
// 2). It doesn't actually work because the tokens don't actually point the nodes they
//     are direct descendants of.
// 3). I just don't think it's very well written.

// Instead I think nodes need to store their own tokens instead of it being separate.
// The reason is because the old method doesn't actually work. Also, I think that building
// a node should be like math operations (+). For example, when parsing the FnInputs,
// we say the FnInputs = "(" + ")" (assuming we aren't parsing the actual inputs currently).
// It would actually be lexer::TokenKind::LParen + lexer::TokenKind::RParen. There is a few
// problems though.

// 1). How would we skip whitespace and comments?
// 2). How would we emit tokens?
// 3). How would we actually build the node?

/*
struct FnDecl {
    fn_keyword: Token, // syntax::Tokens need to be able to store leading and trailing trivia so that whitespace can be skipped.
    name: Result<Token>,
    inputs: Result<FnInputs>,
}

struct FnInputs {
    l_paren: Token,
    r_paren: Token,
}
*/

// use std::{marker::PhantomData, ops::Range};

// use crate::{
//     lexer,
//     scanner::Scanner,
//     syntax::{self, FnDecl, FnInputs},
// };

// struct RecoverableParser<T, P, R> {
//     parse: P,
//     recover: R,
//     ty: PhantomData<T>,
// }

// #[inline(always)]
// fn recoverable_parser<T, P: Fn(&mut Parser) -> Result<T, syntax::Error>, R>(
//     parse: P,
//     recover: R,
// ) -> RecoverableParser<T, P, R> {
//     RecoverableParser {
//         parse,
//         recover,
//         ty: PhantomData,
//     }
// }

// #[derive(PartialEq, Eq)]
// enum Recovery {
//     Consume,
//     Stop,
// }

// struct Parser<'src> {
//     scanner: Scanner<'src>,
//     tokens: Vec<syntax::Token>,
//     nodes: Vec<Result<syntax::Node, syntax::Error>>,
// }

// impl<'src> Parser<'src> {
//     fn new(src: &'src str) -> Self {
//         Self {
//             scanner: Scanner::new(src),
//             tokens: vec![],
//             nodes: vec![],
//         }
//     }

//     #[inline(always)]
//     fn parse<
//         T,
//         P: Fn(&mut Self) -> Result<T, syntax::Error>,
//         R: Fn(lexer::TokenKind) -> Option<Recovery>,
//     >(
//         &mut self,
//         parser: RecoverableParser<T, P, R>,
//     ) -> Result<T, syntax::Error> {
//         let result = (parser.parse)(self);

//         if result.is_err() {
//             self.recover(parser.recover);
//         }

//         result
//     }

//     fn recover<R: Fn(lexer::TokenKind) -> Option<Recovery>>(&mut self, recover: R) {
//         let mut recovery;
//         let mut token = self.scanner.peek(0);

//         loop {
//             recovery = if token.kind == lexer::TokenKind::Eoi {
//                 Some(Recovery::Consume)
//             } else {
//                 recover(token.kind)
//             };

//             if recovery.is_some() {
//                 break;
//             } else {
//                 self.emit_skipped(&token);
//                 token = self.scanner.next();
//             }
//         }

//         if recovery.unwrap() == Recovery::Consume {
//             if token.kind == lexer::TokenKind::Eoi {
//                 self.emit_token(syntax::TokenKind::UnexpectedEoi, &token.range);
//             } else {
//                 self.emit_token(syntax::TokenKind::Delimiter(token.kind), &token.range);
//                 self.scanner.eat();
//             }
//         }
//     }

//     fn peek(&mut self, expected: lexer::TokenKind) -> Result<lexer::Token, syntax::Error> {
//         let mut token = self.scanner.peek(0);

//         while token.kind == lexer::TokenKind::Whitespace {
//             self.emit_whitespace(&token.range);
//             token = self.scanner.next();
//         }

//         if token.kind != lexer::TokenKind::Eoi {
//             Ok(token)
//         } else {
//             self.emit_expected(expected, &token.range);
//             Err(syntax::Error::unexpected_eoi(token))
//         }
//     }

//     fn expect(
//         &mut self,
//         expected: lexer::TokenKind,
//         kind: syntax::TokenKind,
//     ) -> Result<lexer::Token, syntax::Error> {
//         let token = self.peek(expected)?;

//         if token.kind == expected {
//             self.emit_token(kind, &token.range);
//             self.scanner.eat();
//             Ok(token)
//         } else {
//             self.emit_expected(expected, &token.range);
//             Err(syntax::Error::expected(expected, token))
//         }
//     }

//     fn expect_delimiter(
//         &mut self,
//         expected: lexer::TokenKind,
//     ) -> Result<lexer::Token, syntax::Error> {
//         self.expect(expected, syntax::TokenKind::Delimiter(expected))
//     }

//     fn emit_token(&mut self, kind: syntax::TokenKind, range: &Range<usize>) {
//         self.tokens.push(syntax::Token {
//             kind,
//             range: range.clone(),
//             node: 0, // TODO.
//         })
//     }

//     fn emit_skipped(&mut self, token: &lexer::Token) {
//         self.emit_token(syntax::TokenKind::Skipped(token.kind), &token.range);
//     }

//     fn emit_whitespace(&mut self, range: &Range<usize>) {
//         self.emit_token(syntax::TokenKind::Whitespace, range);
//     }

//     fn emit_expected(&mut self, expected: lexer::TokenKind, range: &Range<usize>) {
//         self.emit_token(syntax::TokenKind::Expected(expected), range);
//     }

//     fn emit_node(&mut self, node: Result<syntax::Node, syntax::Error>) {
//         self.nodes.push(node);
//     }
// }

// pub fn parse_internal(src: &str) -> syntax::Tree {
//     let mut parser = Parser::new(src);

//     let fn_decl = recoverable_parser(
//         |parser| {
//             let fn_name = recoverable_parser(
//                 |parser| {
//                     let token =
//                         parser.expect(lexer::TokenKind::Ident, syntax::TokenKind::FnName)?;

//                     Ok(parser.scanner.text(&token).to_string())
//                 },
//                 |token| match token {
//                     lexer::TokenKind::LParen => Some(Recovery::Stop),
//                     _ => None,
//                 },
//             );

//             let fn_inputs = recoverable_parser(
//                 |parser| {
//                     parser.expect_delimiter(lexer::TokenKind::LParen)?;

//                     // TODO: Parse the inputs.

//                     parser.expect_delimiter(lexer::TokenKind::RParen)?;

//                     Ok(FnInputs)
//                 },
//                 |token| match token {
//                     lexer::TokenKind::RParen => Some(Recovery::Consume),
//                     // TODO: ThinArrow.
//                     lexer::TokenKind::FatArrow => Some(Recovery::Stop),
//                     lexer::TokenKind::LBrace => Some(Recovery::Stop),
//                     _ => None,
//                 },
//             );

//             parser.expect(
//                 lexer::TokenKind::Fn,
//                 syntax::TokenKind::Keyword(lexer::TokenKind::Fn),
//             )?;

//             let name = parser.parse(fn_name);
//             let inputs = parser.parse(fn_inputs);

//             Ok(FnDecl { name, inputs })
//         },
//         |_| None,
//     );

//     let fn_decl = parser.parse(fn_decl);
//     parser.emit_node(fn_decl.map(syntax::Node::FnDecl));

//     syntax::Tree {
//         tokens: parser.tokens,
//         nodes: parser.nodes,
//     }
// }
