use crate::error::{Context, Result};
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

/// Create a new directory. Does nothing if it already exists.
pub fn mkdir(path: &Path) -> Result<()> {
    match fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(e).with_context(|| format!("creating directory {path:?}")),
    }
}
