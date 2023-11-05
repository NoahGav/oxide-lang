// TODO: When using incremental computation, if tokens include their span in the
// TODO: source file, wouldn't that cause recomputation if nothing actually changed,
// TODO: by the location did (whitespace or comments)? It would be better in this
// TODO: regard if there was a way to specify equality or something with tokens to
// TODO: only compare the parts we are interested in and not the span.

use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use dashmap::{DashMap, DashSet};
use lazy_static::lazy_static;
use notify::{Error, Event, ReadDirectoryChangesWatcher, RecursiveMode, Watcher};

pub use jar::*;
use parking_lot::RwLock;
use salsa::ParallelDatabase;
use walkdir::WalkDir;

mod db;
mod jar;
mod parser;

lazy_static! {
    pub static ref COMPILER: Arc<Compiler> = Arc::new(Compiler::new());
}

pub struct Compiler {
    db: RwLock<db::Database>,
    files: DashMap<PathBuf, SourceFile>,
    watcher: OnceLock<Option<ReadDirectoryChangesWatcher>>,
    open_files: DashSet<PathBuf>,
}

unsafe impl Sync for Compiler {}

pub struct InitializeOptions {
    /// Whether or not the compiler should watch the filesystem for changes.
    pub watch: Watch,

    /// Whether or not this compiler instance should block others from running.
    pub block: Block,
}

#[derive(PartialEq, Eq)]
pub enum Watch {
    /// The compiler will watch the filesystem for changes.
    Yes,
    /// The compiler will not to watch the filesystem for changes.
    No,
}

#[derive(PartialEq, Eq)]
pub enum Block {
    /// The compiler will retain a lock on the lock file,
    /// blocking other compiler instances from running.
    Yes,
    /// The compiler will not retain a lock on the lock file,
    /// allowing other compiler instances to run.
    No,
}

pub struct Snapshot {
    db: salsa::Snapshot<db::Database>,
    files: DashMap<PathBuf, SourceFile>,
}

#[salsa::input]
pub struct SourceFile {
    #[return_ref]
    pub text: String,
}

impl Compiler {
    fn new() -> Self {
        Self {
            db: RwLock::new(db::Database::default()),
            files: DashMap::new(),
            watcher: OnceLock::new(),
            open_files: DashSet::new(),
        }
    }

    pub fn initialize(self: &Arc<Self>, path: impl Into<PathBuf>, options: InitializeOptions) {
        // TODO: If the lock behavior is block, then attempt to obtain a lock on the lock file.

        let path = path.into();

        if options.watch == Watch::Yes {
            let this = self.clone();

            let mut watcher =
                notify::recommended_watcher(move |event| this.watch_event(event)).unwrap();

            watcher.watch(&path, RecursiveMode::Recursive).unwrap();

            self.watcher.set(Some(watcher)).unwrap();
        }

        // Synchronize with the filesystem (regardless of if we are watching it).
        let db = self.db.read();

        WalkDir::new(path).into_iter().for_each(|entry| {
            if let Ok(entry) = entry {
                if let Some("ox") = entry.path().extension().and_then(|ext| ext.to_str()) {
                    let text = std::fs::read_to_string(entry.path()).unwrap();
                    let source = SourceFile::new(db.deref(), text);

                    self.files.insert(entry.path().into(), source);
                }
            }
        });
    }

    pub fn snapshot(self: &Arc<Self>) -> Snapshot {
        Snapshot {
            db: self.db.read().deref().snapshot(),
            files: self.files.clone(),
        }
    }

    /// Marks a file as "open" for editing in an editor. While a file is "open," the compiler
    /// will not listen to file system events for it. Instead, it will rely on events generated
    /// by the `change_file` method to track modifications to the file.
    pub fn open_file(&self, path: impl Into<PathBuf>, text: &str) {
        let path = path.into();

        self.open_files.insert(path.clone());

        let source = self.files.get(&path);

        if let Some(source) = source {
            source.set_text(self.db.write().deref_mut()).to(text.into());
        } else {
            let source = SourceFile::new(self.db.read().deref(), text.into());
            self.files.insert(path.clone(), source);
        }
    }

    // pub fn change_file(&self, path: impl Into<PathBuf>) {
    //     TODO: If the file is not "open", panic.
    //     TODO: This should incrementally update the file.
    // }

    /// Closes a previously "open" file in the editor. When a file is closed, the compiler
    /// will revert to listening to file system events for it. The compiler will no longer
    /// rely on events from the `change_file` method to track modifications to the file.
    pub fn close_file(&self, path: impl Into<PathBuf>) {
        self.open_files.remove(&path.into());
        // TODO: Resync with the filesystem (if the file still exists, get it's text,
        // TODO: if not, remove the file).
    }

    fn watch_event(&self, event: Result<Event, Error>) {
        if let Ok(event) = event {
            match event.kind {
                notify::EventKind::Create(_) => self.modify_event(event),
                notify::EventKind::Modify(_) => self.modify_event(event),
                notify::EventKind::Remove(_) => self.remove_event(event),
                _ => {}
            }
        }
    }

    fn modify_event(&self, event: Event) {
        if !event
            .paths
            .iter()
            .any(|path| self.open_files.contains(path))
        {
            println!("modify: {:?}", event);

            let path = &event.paths[0];

            if let Some("ox") = path.extension().and_then(|ext| ext.to_str()) {
                let text = std::fs::read_to_string(path);

                if let Ok(text) = text {
                    let source = self.files.get(path);

                    if let Some(source) = source {
                        source.set_text(self.db.write().deref_mut()).to(text);
                    } else {
                        let source = SourceFile::new(self.db.read().deref(), text);
                        self.files.insert(path.clone(), source);
                    }
                }
            }
        }
    }

    fn remove_event(&self, event: Event) {
        if !event
            .paths
            .iter()
            .any(|path| self.open_files.contains(path))
        {
            println!("remove: {:?}", event);

            let path = &event.paths[0];

            if let Some("ox") = path.extension().and_then(|ext| ext.to_str()) {
                self.files.remove(path);
            }
        }
    }
}

impl Snapshot {
    pub fn parse(&self, path: impl Into<PathBuf>) {
        let source = self.files.get(&path.into()).unwrap();

        parser::parse(self.db.deref(), *source);
    }
}
