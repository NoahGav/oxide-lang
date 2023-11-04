// TODO: I should create a macro system that will make this far easier.
// TODO: There a quite a few issues.
// TODO: 1). You have to maintain the Query and QueryResult enums.
// TODO: 2). You have to write all the queries in one place.
// TODO: 3). You cannot easily resolve a query inside a query as you
// TODO:     have to call resolve.query and then unwrap the QueryResult
// TODO:     despite knowing what it is.

use std::{path::PathBuf, sync::Arc};

use blinc::ResolveQuery;
use enum_as_inner::EnumAsInner;
use oxide_parser::syntax;

use crate::CompilerState;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Query {
    GetDocumentContent(PathBuf),
    SyntaxTree(PathBuf),
}

#[derive(Clone, EnumAsInner)]
pub enum QueryResult {
    GetDocumentContent(Arc<String>),
    SyntaxTree(Arc<syntax::Tree>),
}

impl PartialEq for QueryResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::GetDocumentContent(l0), Self::GetDocumentContent(r0)) => Arc::ptr_eq(l0, r0),
            (Self::SyntaxTree(l0), Self::SyntaxTree(r0)) => Arc::ptr_eq(l0, r0),
            _ => false,
        }
    }
}

impl Eq for QueryResult {}

impl ResolveQuery<Query, QueryResult> for Arc<CompilerState> {
    fn resolve(
        &self,
        q: Query,
        resolve: Arc<blinc::QueryResolver<Query, QueryResult>>,
    ) -> QueryResult {
        match q {
            Query::GetDocumentContent(path) => QueryResult::GetDocumentContent({
                let document = self.documents.get(&path).unwrap();
                document.content.clone()
            }),
            Query::SyntaxTree(path) => QueryResult::SyntaxTree({
                // TODO: Make a macro that simplifies this please.
                let src = resolve.query(Query::GetDocumentContent(path));
                let src = src.as_get_document_content().unwrap();

                Arc::new(oxide_parser::parse(src.as_ref()))
            }),
        }
    }
}
