use lexer::Lexer;

mod lexer;

pub struct Parser<'src> {
    src: &'src str,
}

impl<'src> Parser<'src> {
    pub fn new(src: &'src str) -> Self {
        Self { src }
    }

    pub fn parse(self) {
        let tokens = Lexer::new(self.src).tokenize();
        println!("{:#?}", tokens);
    }
}
