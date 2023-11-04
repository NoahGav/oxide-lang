use std::{path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct Document {
    pub path: PathBuf,
    pub content: Arc<String>,
}

impl Document {
    pub fn new(path: &str, content: &str) -> Self {
        Self {
            path: path.into(),
            content: Arc::new(content.into()),
        }
    }
}
