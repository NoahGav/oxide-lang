use lexer::TokenStream;

mod lexer;

pub struct Parser<'src> {
    src: &'src str,
}

impl<'src> Parser<'src> {
    pub fn new(src: &'src str) -> Self {
        Self { src }
    }

    pub fn parse(self) {
        let tokens: TokenStream = self.src.into();

        for token in tokens {
            println!("{:?}", token);
        }
    }
}
