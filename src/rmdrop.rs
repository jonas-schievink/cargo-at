//! Automatically delete a file when dropped

use std::path::{Path, PathBuf};
use std::fs::remove_file;

/// Removes a path when dropped
pub struct RemoveOnDrop(PathBuf);

impl RemoveOnDrop {
    pub fn new<P: AsRef<Path>>(path: P) -> RemoveOnDrop {
        RemoveOnDrop(path.as_ref().to_path_buf())
    }
}

impl Drop for RemoveOnDrop {
    fn drop(&mut self) {
        println!("removing {}", self.0.display());
        remove_file(&self.0).ok();  // (ignores errors)
    }
}
