use std::marker::PhantomData;

use scanner::Scanner;
use syntax::FnInputs;

use crate::syntax::FnDecl;

mod lexer;
mod parser;
mod scanner;
mod syntax;

struct Parser<'src> {
    scanner: Scanner<'src>,
    nodes: Vec<Result<syntax::Node, syntax::Error>>,
}

impl<'src> Parser<'src> {
    fn new(src: &'src str) -> Self {
        Self {
            scanner: Scanner::new(src),
            nodes: vec![],
        }
    }
}

struct RecoverableParser<T, P, R> {
    parse: P,
    recover: R,
    ty: PhantomData<T>,
}

#[derive(PartialEq, Eq)]
enum Recovery {
    /// Skip the token upon recovery.
    Skip,
    /// Stop on that token upon recovery.
    Stop,
}

struct ParseContext<'p, 'src, T> {
    parser: &'p mut Parser<'src>,
    tokens: Vec<syntax::Token>,
    ty: PhantomData<T>,
}

fn recoverable<T, P, R>(parse: P, recover: R) -> RecoverableParser<T, P, R> {
    RecoverableParser {
        parse,
        recover,
        ty: PhantomData,
    }
}

impl<
        T,
        P: Fn(&mut ParseContext<T>) -> syntax::Result<T>,
        R: Fn(lexer::TokenKind) -> Option<Recovery>,
    > RecoverableParser<T, P, R>
{
    fn execute(&self, parser: &mut Parser) -> syntax::Result<T> {
        let mut context = ParseContext {
            parser,
            tokens: vec![],
            ty: PhantomData,
        };

        let result = (self.parse)(&mut context);

        result.map_err(|mut error| {
            let mut recovery;
            let mut token = context.peek();

            loop {
                if token.kind == lexer::TokenKind::Eoi {
                    recovery = Some(Recovery::Stop);
                    break;
                }

                recovery = (self.recover)(token.kind);

                if recovery.is_some() {
                    break;
                } else {
                    context.skip(&token);
                    token = context.parser.scanner.next();
                }
            }

            if recovery.unwrap() == Recovery::Skip {
                context.skip(&token);
            }

            error.tokens = context.tokens;
            error
        })
    }
}

impl<'p, 'src, T> ParseContext<'p, 'src, T> {
    // TODO: This should return a syntax::Token with trivia (the skipped stuff).
    fn peek(&mut self) -> lexer::Token {
        let mut token = self.parser.scanner.peek(0);

        while token.kind == lexer::TokenKind::Whitespace {
            // TODO: Token trivia.
            token = self.parser.scanner.next();
        }

        token
    }

    fn expect(&mut self, expected: lexer::TokenKind, kind: syntax::TokenKind) -> syntax::Token {
        let token = self.peek();

        let token = if token.kind == expected {
            syntax::Token {
                kind,
                range: self.parser.scanner.eat().range,
            }
        } else {
            syntax::Token::missing(kind, &token)
        };

        self.tokens.push(token.clone());
        token
    }

    fn expect_delimiter(&mut self, delimiter: lexer::TokenKind) -> syntax::Token {
        self.expect(delimiter, syntax::TokenKind::Delimiter(delimiter))
    }

    fn expect_keyword(&mut self, keyword: lexer::TokenKind) -> syntax::Token {
        self.expect(keyword, syntax::TokenKind::Keyword(keyword))
    }

    fn expect_ident(&mut self) -> syntax::Token {
        self.expect(lexer::TokenKind::Ident, syntax::TokenKind::Ident)
    }

    fn panic(&mut self, error: syntax::Error) -> syntax::Result<T> {
        Err(error)
    }

    fn skip(&mut self, token: &lexer::Token) {
        self.tokens.push(syntax::Token::skip(token));
    }
}

pub fn parse(src: &str) -> syntax::Tree {
    let mut parser = Parser::new(src);

    let fn_inputs = recoverable(
        |parser: &mut ParseContext<'_, '_, _>| {
            let l_paren = parser.expect_delimiter(lexer::TokenKind::LParen);
            let r_paren = parser.expect_delimiter(lexer::TokenKind::RParen);

            Ok(FnInputs { l_paren, r_paren })
        },
        |token| match token {
            lexer::TokenKind::RParen => Some(Recovery::Skip),
            // TODO: ThinArrow.
            lexer::TokenKind::FatArrow => Some(Recovery::Stop),
            lexer::TokenKind::LBrace => Some(Recovery::Stop),
            _ => None,
        },
    );

    let fn_decl = recoverable(
        |parser: &mut ParseContext<'_, '_, _>| {
            let fn_keyword = parser.expect_keyword(lexer::TokenKind::Fn);
            let name = parser.expect_ident();
            let inputs = fn_inputs.execute(parser.parser);

            Ok(FnDecl {
                fn_keyword,
                name,
                inputs,
            })
        },
        |_| None,
    );

    let result = fn_decl.execute(&mut parser);
    parser.nodes.push(result.map(syntax::Node::FnDecl));

    syntax::Tree {
        nodes: parser.nodes,
    }
}
