//! Lockfile
//!
//! # Structure
//!
//! [[source_install]]
//! source = git
//!
//! [[source_install.metadata]]
//! repository = "..."
//! tag = "..."
//!
//!

use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::filesystem::Filesystem;

// pub struct Entry {
// }

pub struct SourceBuild {}

pub enum Entry {
    SourceBuild(SourceBuild),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LockfileData {}

pub struct Lockfile {
    filesystem: Filesystem,
    // entries: Option<HashMap<String, Entry>>,
}

impl Lockfile {
    pub fn test(&self) {}
}

impl Lockfile {
    pub fn new(filesystem: Filesystem) -> Result<Self> {
        let lockfile_path = filesystem.get_lockfile_path();
        let lockfile_content = Self::read_lockfile(&lockfile_path)?;

        let deserialized: LockfileData = toml::from_str(&lockfile_content[..])?;

        println!("{:?}", deserialized);

        Ok(Self { filesystem })
    }

    fn read_lockfile(path: &PathBuf) -> Result<String> {
        if !path.exists() {
            create_dir_all(path.parent().unwrap()).unwrap();

            let mut file = File::create(path)?;
            file.write_all("".as_bytes())?;
        }

        Ok(read_to_string(path)?)
    }
}
