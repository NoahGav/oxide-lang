use std::ops::Range;

use lexer::Lexer;
use syntax::{FnBody, FnDecl, FnInputs, Type};

mod lexer;
mod syntax;

pub struct Parser<'src> {
    scanner: Scanner<'src>,
}

struct Scanner<'src> {
    src: &'src str,
    tokens: Vec<lexer::Token>,
    cursor: usize,
}

impl<'src> Scanner<'src> {
    fn new(src: &'src str) -> Self {
        Self {
            src,
            tokens: Lexer::new(src).tokenize(),
            cursor: 0,
        }
    }

    fn peek(&self, offset: usize) -> lexer::Token {
        let index = (self.cursor + offset).min(self.src.len() - 1);
        self.tokens[index].clone()
    }

    fn eat(&mut self) -> lexer::Token {
        let token = self.peek(0);
        self.cursor += 1;
        token
    }

    fn text(&self, token: &lexer::Token) -> &str {
        &self.src[token.range.clone()]
    }
}

impl<'src> Parser<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            scanner: Scanner::new(src),
        }
    }

    pub fn parse(mut self) -> syntax::Tree {
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

        let mut tokens = vec![];
        let mut nodes = vec![];

        loop {
            let token = self.scanner.peek(0);

            if token.kind != lexer::TokenKind::Eoi {
                let ctx = ParseContext::new(&mut self.scanner);

                let (node, parse_tokens) = match token.kind {
                    lexer::TokenKind::Fn => ctx.fn_decl(),
                    _ => todo!(),
                };

                nodes.push(node);

                let node = nodes.len() - 1;

                tokens.extend(parse_tokens.iter().cloned().map(|token| token.into(node)));
            } else {
                break;
            }
        }

        syntax::Tree { tokens, nodes }
    }
}

struct ParseContext<'src, 'scanner> {
    scanner: &'scanner mut Scanner<'src>,
    tokens: Vec<ParseToken>,
}

impl<'src, 'scanner> ParseContext<'src, 'scanner> {
    fn new(scanner: &'scanner mut Scanner<'src>) -> Self {
        Self {
            scanner,
            tokens: vec![],
        }
    }

    fn emit(&mut self, kind: syntax::TokenKind, range: Range<usize>) {
        self.tokens.push(ParseToken { kind, range });
    }

    fn eat(&mut self, kind: syntax::TokenKind) {
        let token = self.scanner.eat();
        self.emit(kind, token.range);
    }

    fn missing(&mut self, kind: syntax::TokenKind) {
        let token = self.scanner.peek(0);

        self.emit(
            syntax::TokenKind::Missing(Box::new(kind)),
            token.range.start..token.range.start,
        );
    }

    fn skip_whitespace(&mut self) {
        loop {
            let token = self.scanner.peek(0);

            if token.kind == lexer::TokenKind::Whitespace {
                self.eat(syntax::TokenKind::Whitespace);
            } else {
                break;
            }
        }
    }

    fn fn_decl(mut self) -> (syntax::Node, Vec<ParseToken>) {
        // This is the entry point of parsing a fn decl. While
        // parsing we will emit parse tokens. This includes all
        // tokens, including skipped and missing tokens. When
        // we are done, we return the parsed syntax::Node and
        // the emitted parse tokens. The main parser then appends
        // the node to the tree (obtaining it's index) and then
        // converts the parse tokens to syntax tokens and extends
        // the tree tokens.

        self.eat(syntax::TokenKind::FnKeyword);

        let name = self.ident(syntax::TokenKind::FnName);

        // TODO: inputs, outputs, and body.

        let node = syntax::Node::FnDecl(FnDecl {
            name,
            inputs: Ok(FnInputs),
            output: Ok(Type),
            body: Ok(FnBody),
        });

        (node, self.tokens)
    }

    fn ident(&mut self, kind: syntax::TokenKind) -> Result<String, syntax::Error> {
        self.skip_whitespace();

        let token = self.scanner.peek(0);

        if let lexer::TokenKind::Ident = token.kind {
            self.eat(kind);
            Ok(self.scanner.text(&token).into())
        } else {
            self.missing(kind.clone());
            Err(syntax::Error::missing(kind, token.range))
        }
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
