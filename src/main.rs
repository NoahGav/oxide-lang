use oxide_lang::parser::Lexer;

fn main() {
    let src = r#"let foo = 42;"#;
    let tokens = Lexer::new(src).tokenize();
    println!("{:#?}", tokens);
}
