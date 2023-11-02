use std::ops::Range;

use lexer::{Lexer, Token};

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

    fn peek(&self, offset: usize) -> Option<Token> {
        self.tokens.get(self.cursor + offset).cloned()
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

        while let Some(token) = self.scanner.peek(0) {
            let ctx = ParseContext::new(&mut self.scanner);

            let _ = match token.kind {
                lexer::TokenKind::Fn => ctx.fn_decl(),
                _ => todo!(),
            };
        }

        todo!()
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
