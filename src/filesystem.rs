use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Filesystem {}

impl Filesystem {
    pub fn new() -> Self {
        let home = dirs::home_dir();
        let cache = dirs::cache_dir();
        let data = dirs::data_dir();

        Self {}
    }

    pub fn with_cache<F: FnOnce(PathBuf) -> Result<()>>(&self, dir: &str, func: F) -> Result<()> {
        let mut path = dirs::cache_dir().unwrap();
        path.push("dotfiles");
        path.push(dir);

        create_dir_all(path.clone())?;

        func(path)?;

        Ok(())
    }

    pub fn get_lockfile_path(&self) -> PathBuf {
        dirs::data_dir().unwrap().join("/dotfiles/dotfiles.lock")
    }
}
