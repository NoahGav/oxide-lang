use oxide_parser::Parser;

fn main() {
    let src = r#"let foo = 42;"#;

    Parser::new(src).parse();
}
