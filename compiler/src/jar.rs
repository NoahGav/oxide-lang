#[salsa::jar(db = Db)]
pub struct Jar(crate::SourceFile, crate::ParsedFile, crate::parser::parse);

pub trait Db: salsa::DbWithJar<Jar> {}
impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
