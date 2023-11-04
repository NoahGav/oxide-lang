use std::{
    cell::RefCell,
    fmt::Debug,
    hash::Hash,
    sync::{Arc, OnceLock},
};

use hashbrown::HashSet;
use map::ConcurrentMap;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

mod map;

/// The `Graph` struct represents a concurrent query dependency graph. It provides
/// the infrastructure for managing, resolving, and optimizing a wide range of
/// queries across a variety of applications, including but not limited to
/// concurrent incremental compilers.
///
/// # Overview
///
/// A `Graph` instance serves as the central data structure for tracking and managing
/// dependencies between queries (`Q`) and their respective results (`R`). This data
/// structure is completely agnostic to the specific use case, and its generic nature
/// makes it adaptable to a multitude of scenarios beyond compiler optimization.
///
/// # Query Resolution
///
/// The `Graph` type allows for concurrent query resolution, where queries can be
/// resolved in parallel, enhancing performance significantly. The `increment` method
/// does not block and returns a new iteration of the graph immediately. In fact,
/// the `Graph.query` method is designed to block as little as possible. In the
/// general case it will block extremely infrequently. This means that queries
/// will be able to continue chugging away as fast as possible with little to no
/// interruptions.
///
/// # Incremental Compilation
///
/// In the context of a concurrent incremental compiler, only queries that are
/// determined to be outdated are resolved again. This approach greatly reduces the
/// time required to resolve queries, making the example compiler incremental.
/// The semantic model is still built every time you mutate the compiler's state,
/// but most of the work is already done and can be reused, resulting in extremely
/// fast model building.
pub struct Graph<Q, R> {
    /// The new map is used for all the queries in this iteration.
    /// This map always starts empty.
    new: QueryNodeMap<Q, R>,
    /// The old map is used for validating queries from this iteration.
    /// It's just a reference to the map from the previous iteration and
    /// so is very efficient.
    old: QueryNodeMap<Q, R>,
    /// The resolver used to resolve queries. The resolver can have its
    /// own state as long as it's Sync + Send.
    resolver: Box<dyn ResolveQuery<Q, R>>,
}

#[derive(Debug)]
struct Node<Q, R> {
    result: R,
    changed: bool,
    edges_from: Arc<HashSet<Q>>,
}

type QueryNodeMap<Q, R> = Arc<ConcurrentMap<Q, Arc<OnceLock<Node<Q, R>>>>>;

impl<Q: Debug + Clone + Eq + Hash, R: Debug + Clone> Debug for Graph<Q, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Graph")
            .field("new", &self.new)
            .field("old", &self.old)
            .finish()
    }
}

impl<Q: Clone + Eq + Hash + Send + Sync, R: Clone + Eq + Send + Sync> Graph<Q, R> {
    pub fn new(resolver: impl ResolveQuery<Q, R> + 'static) -> Arc<Self> {
        Arc::new(Self {
            new: Arc::new(ConcurrentMap::new()),
            old: Arc::new(ConcurrentMap::new()),
            resolver: Box::new(resolver),
        })
    }

    pub fn query(self: &Arc<Self>, q: Q) -> R {
        let node = self.get_node(&q);
        let node = node.get_or_init(|| self.resolve(q));
        node.result.clone()
    }

    fn get_node(self: &Arc<Self>, q: &Q) -> Arc<OnceLock<Node<Q, R>>> {
        self.new
            .get_or_insert(q.clone(), || Arc::new(OnceLock::default()))
    }

    fn resolve(self: &Arc<Self>, q: Q) -> Node<Q, R> {
        if let Some(old) = self.old.get(&q) {
            // Since there was an old node we have to validate it.
            let old_node = old.get();

            if let Some(old_node) = old_node {
                if old_node.edges_from.len() == 0 {
                    // Since the node had no dependencies (a root node) we must
                    // resolve it again to see if it changed.
                    let resolver = Arc::new(QueryResolver::new(self.clone()));
                    let result = self.resolver.resolve(q, resolver.clone());

                    Node {
                        // This is very important and crucial to the whole system
                        // working. If the result is the same as the old result then
                        // changed must be false. This prevents nodes from needlessly
                        // being resolved again when their old values can be used
                        // instead.
                        changed: result != old_node.result,
                        result,
                        edges_from: Arc::new(resolver.edges_from.take()),
                    }
                } else {
                    let any_changed = old_node.edges_from.par_iter().any(|parent| {
                        let node = self.get_node(parent);
                        let node = node.get_or_init(|| self.resolve(parent.clone()));

                        node.changed
                    });

                    if any_changed {
                        // Since at least one dependency of this query has changed
                        // we have to resolve this query again.
                        let resolver = Arc::new(QueryResolver::new(self.clone()));
                        let result = self.resolver.resolve(q, resolver.clone());

                        Node {
                            // This is very important and crucial to the whole system
                            // working. If the result is the same as the old result then
                            // changed must be false. This prevents nodes from needlessly
                            // being resolved again when their old values can be used
                            // instead.
                            changed: result != old_node.result,
                            result,
                            edges_from: Arc::new(resolver.edges_from.take()),
                        }
                    } else {
                        // The old result is still valid so we just clone it.
                        Node {
                            result: old_node.result.clone(),
                            edges_from: old_node.edges_from.clone(),
                            changed: false,
                        }
                    }
                }
            } else {
                // Since the old node is not resolved yet we will just resolve
                // it from scratch.
                let resolver = Arc::new(QueryResolver::new(self.clone()));
                let result = self.resolver.resolve(q, resolver.clone());

                Node {
                    // We need to check again if the old node is still unresolved. Because
                    // if it isn't we can set changed to old_result != result. Otherwise,
                    // we always set changed to true.
                    changed: match old.get() {
                        Some(old_node) => result != old_node.result,
                        None => true,
                    },
                    result,
                    edges_from: Arc::new(resolver.edges_from.take()),
                }
            }
        } else {
            // Since the node isn't in the old map then the query is new and resolved
            // from scratch.
            let resolver = Arc::new(QueryResolver::new(self.clone()));
            let result = self.resolver.resolve(q, resolver.clone());

            Node {
                result,
                // Since this is a new node, changed is always false.
                changed: false,
                edges_from: Arc::new(resolver.edges_from.take()),
            }
        }
    }

    pub fn increment(self: &Arc<Self>, resolver: impl ResolveQuery<Q, R> + 'static) -> Arc<Self> {
        Arc::new(Self {
            new: Arc::new(ConcurrentMap::new()),
            old: self.new.clone(),
            resolver: Box::new(resolver),
        })
    }
}

pub struct QueryResolver<Q, R> {
    graph: Arc<Graph<Q, R>>,
    edges_from: RefCell<HashSet<Q>>,
}

unsafe impl<Q, R> Send for QueryResolver<Q, R> {}
unsafe impl<Q, R> Sync for QueryResolver<Q, R> {}

impl<Q: Clone + Eq + Hash + Send + Sync, R: Clone + Eq + Send + Sync> QueryResolver<Q, R> {
    fn new(graph: Arc<Graph<Q, R>>) -> Self {
        Self {
            graph,
            edges_from: RefCell::new(HashSet::new()),
        }
    }

    pub fn query(&self, q: Q) -> R {
        let result = self.graph.query(q.clone());
        self.edges_from.borrow_mut().insert(q);
        result
    }
}

pub trait ResolveQuery<Q, R>: Send + Sync {
    fn resolve(&self, q: Q, resolve: Arc<QueryResolver<Q, R>>) -> R;
}
