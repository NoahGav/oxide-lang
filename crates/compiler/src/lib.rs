// TODO: The compiler will use an incremental computation framework
// TODO: to drastically speed up incremental compilation. It will also
// TODO: expose a very simple, intuitive, and powerful api. The api
// TODO: will be completely thread-safe and a lot of the computation
// TODO: will be done in parallel (e.g. parsing of documents can utilize
// TODO: all cores). It will work by using Snapshots. Snapshots will
// TODO: contain an immutable snapshot of the compiler's state, plus
// TODO: the incremental computation associated with that snapshot.

use blinc::Graph;
pub use document::*;
use oxide_parser::syntax;
use query::{Query, QueryResult};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

mod document;
mod query;

pub struct Compiler {
    snapshot: Arc<Snapshot>,
}

pub struct Snapshot {
    state: Arc<CompilerState>,
    graph: Arc<blinc::Graph<Query, QueryResult>>,
}

#[derive(Default, Clone)]
pub struct CompilerState {
    documents: HashMap<PathBuf, Document>,
}

impl Default for Compiler {
    fn default() -> Self {
        let state = Arc::new(CompilerState::default());

        Self {
            snapshot: Arc::new(Snapshot {
                state: state.clone(),
                graph: Graph::new(state),
            }),
        }
    }
}

impl Compiler {
    pub fn mutate<F: FnOnce(&mut CompilerState)>(&mut self, mutation: F) {
        let mut state = self.snapshot.state.as_ref().clone();

        mutation(&mut state);

        let state = Arc::new(state);

        self.snapshot = Arc::new(Snapshot {
            state: state.clone(),
            graph: self.snapshot.graph.increment(state),
        });
    }

    pub fn snapshot(&self) -> Arc<Snapshot> {
        self.snapshot.clone()
    }
}

impl CompilerState {
    pub fn add(&mut self, document: Document) {
        self.documents.insert(document.path.clone(), document);
    }
}

impl Snapshot {
    pub fn get_syntax_tree(&self, path: &str) -> Arc<syntax::Tree> {
        let result = self.graph.query(Query::SyntaxTree(path.into()));
        result.as_syntax_tree().unwrap().clone()
    }
}
