use std::ops::Range;

use syntax::FnInputs;

use crate::scanner::Scanner;

mod lexer;
mod scanner;
mod syntax;

pub struct Parser<'src> {
    scanner: Scanner<'src>,
    tokens: Vec<syntax::Token>,
    nodes: Vec<syntax::Node>,
}

impl<'src> Parser<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            scanner: Scanner::new(src),
            tokens: vec![],
            nodes: vec![],
        }
    }

    pub fn parse(mut self) -> syntax::Tree {
        loop {
            let token = self.scanner.peek(0);

            match token.kind {
                lexer::TokenKind::Fn => self.fn_decl(),
                lexer::TokenKind::Eoi => break,
                _ => todo!("{:?}", token.kind),
            }
        }

        syntax::Tree {
            tokens: self.tokens,
            nodes: self.nodes,
        }
    }

    fn ident(
        scanner: &mut ScannerAndEmitter,
        kind: syntax::TokenKind,
    ) -> Result<String, syntax::Error> {
        let token = scanner.peek();

        if let lexer::TokenKind::Ident = token.kind {
            scanner.eat(kind);
            Ok(scanner.text(&token).into())
        } else {
            Err(scanner.missing(&token, kind))
        }
    }

    fn fn_decl(&mut self) {
        let mut scanner = ScannerAndEmitter::new(&mut self.scanner);

        scanner.eat(syntax::TokenKind::FnKeyword);

        let ident = Parse::with_recovery(
            &mut scanner,
            |scanner| Parser::ident(scanner, syntax::TokenKind::FnName),
            |token| token == lexer::TokenKind::LParen,
            false,
        );

        let inputs = Parse::with_recovery(
            &mut scanner,
            Parser::fn_inputs,
            |token| token == lexer::TokenKind::RParen,
            true,
        );

        println!("{:#?}", ident);
        println!("{:#?}", inputs);

        println!("{:#?}", scanner.tokens);

        // TODO: Create node and convert scanner tokens in syntax tokens.
    }

    fn fn_inputs(scanner: &mut ScannerAndEmitter) -> Result<FnInputs, syntax::Error> {
        let token = scanner.peek();

        if token.kind == lexer::TokenKind::LParen {
            scanner.eat(syntax::TokenKind::Delimiter(lexer::TokenKind::LParen));
        } else {
            return Err(scanner.missing(
                &token,
                syntax::TokenKind::Delimiter(lexer::TokenKind::LParen),
            ));
        }

        todo!()
    }
}

struct Parse;

impl Parse {
    #[inline(always)]
    fn with_recovery<
        T,
        P: FnOnce(&mut ScannerAndEmitter) -> Result<T, syntax::Error>,
        R: Fn(lexer::TokenKind) -> bool,
    >(
        scanner: &mut ScannerAndEmitter,
        parser: P,
        is_recovery_delimiter: R,
        consume: bool,
    ) -> Result<T, syntax::Error> {
        let result = parser(scanner);

        if result.is_err() {
            loop {
                let token = scanner.peek_and_skip();

                if token.kind == lexer::TokenKind::Eoi {
                    // TODO: scanner.missing(&token, kind);
                    break;
                } else if is_recovery_delimiter(token.kind.clone()) {
                    if consume {
                        scanner.eat(syntax::TokenKind::Delimiter(token.kind));
                    }

                    break;
                } else {
                    scanner.skip();
                }
            }
        }

        result
    }
}

struct ScannerAndEmitter<'scanner, 'src> {
    scanner: &'scanner mut Scanner<'src>,
    tokens: Vec<ParseToken>,
}

impl<'scanner, 'src> ScannerAndEmitter<'scanner, 'src> {
    fn new(scanner: &'scanner mut Scanner<'src>) -> Self {
        Self {
            scanner,
            tokens: vec![],
        }
    }

    fn text(&self, token: &lexer::Token) -> &str {
        self.scanner.text(token)
    }

    fn peek(&mut self) -> lexer::Token {
        let mut token = self.scanner.peek(0);

        while token.kind == lexer::TokenKind::Whitespace {
            self.eat(syntax::TokenKind::Whitespace);
            token = self.scanner.peek(0);
        }

        self.scanner.peek(0)
    }

    fn peek_and_skip(&mut self) -> lexer::Token {
        let mut token = self.scanner.peek(0);

        while token.kind == lexer::TokenKind::Whitespace {
            self.eat(syntax::TokenKind::Skipped(lexer::TokenKind::Whitespace));
            token = self.scanner.peek(0);
        }

        self.scanner.peek(0)
    }

    fn emit(&mut self, kind: syntax::TokenKind, range: Range<usize>) {
        self.tokens.push(ParseToken { kind, range });
    }

    fn eat(&mut self, kind: syntax::TokenKind) {
        let token = self.scanner.eat();
        self.emit(kind, token.range);
    }

    fn missing(&mut self, token: &lexer::Token, kind: syntax::TokenKind) -> syntax::Error {
        self.emit(
            syntax::TokenKind::Missing(Box::new(kind.clone())),
            token.range.start..token.range.start,
        );

        syntax::Error::missing(kind, token.range.clone())
    }

    fn skip(&mut self) {
        let token = self.scanner.eat();
        self.emit(syntax::TokenKind::Skipped(token.kind), token.range);
    }
}

#[derive(Debug, Clone)]
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
