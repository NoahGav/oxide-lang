use crate::SourceFile;

#[salsa::tracked]
pub fn parse(db: &dyn crate::Db, source: SourceFile) {
    let text = source.text(db);
    println!("{}", text);
}
