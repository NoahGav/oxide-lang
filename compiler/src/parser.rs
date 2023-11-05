#[salsa::input]
pub struct SourceFile {
    #[return_ref]
    pub text: Vec<u16>,
}

#[salsa::tracked]
pub struct ParsedFile {
    pub foo: String,
}

#[salsa::tracked]
pub fn parse(db: &dyn crate::Db, source: SourceFile) -> ParsedFile {
    let _text = source.text(db);

    ParsedFile::new(db, "bar".into())
}
