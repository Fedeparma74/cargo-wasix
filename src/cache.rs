use anyhow::Result;
use std::path::{Path, PathBuf};

pub struct Cache {
    all_versions_root: PathBuf,
    root: PathBuf,
}

impl Cache {
    pub fn new(all_versions_root: PathBuf) -> Result<Cache> {
        let root = all_versions_root.join(env!("CARGO_PKG_VERSION"));
        Ok(Cache {
            all_versions_root,
            root,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the path that all versions of `cargo-wasix` store their cache at,
    /// for cleaning.
    pub fn all_versions_root(&self) -> &Path {
        &self.all_versions_root
    }
}
