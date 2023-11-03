use std::ops::Range;

use lexer::Lexer;
use syntax::{FnBody, FnDecl, FnInputs, Type};

mod lexer;
mod syntax;

/// The [Parser] is responsible for parsing source code and constructing a [syntax::Tree].
///
/// It uses a [Scanner] to tokenize the input source code, and based on the identified tokens,
/// it determines which language constructs to parse. The [Parser] follows a systematic process,
/// inspecting tokens, selecting the appropriate parsing context, and parsing various language
/// constructs.
///
/// The parser handles the parsing of different language constructs, such as function declarations
/// ([FnDecl]), and divides them into sections like name, inputs, output, and body. When parsing each
/// section, the parser can either succeed or encounter errors. If an error occurs, the parser skips
/// all tokens between the error and the ending delimiter for that section, and then returns the specific
/// error encountered. Each parsed section can only have one error.
///
/// The [Parser] captures all tokens emitted by the lexer, including all types of tokens, such as
/// skipped and missing tokens. It emits information about each token's kind, range, and [syntax::TokenKind],
/// which are collected as parse tokens.
///
/// Once a language construct is fully parsed, even in the presence of errors, the [Parser] constructs
/// the [syntax::Token]s, associating them with the appropriate syntax node. These syntax tokens are then
/// combined to build the final [syntax::Tree].
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
