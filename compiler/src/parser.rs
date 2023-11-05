use crate::SourceFile;

#[salsa::tracked]
pub fn parse(db: &dyn crate::Db, source: SourceFile) {
    let text = String::from_utf16(source.text(db)).unwrap();
    println!("{}", text);
}
