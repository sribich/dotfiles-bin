use std::{
    collections::HashMap,
    fs::{self, create_dir_all, metadata, read_link, symlink_metadata},
    os::unix::fs::symlink,
    path::PathBuf,
};

use anyhow::Result;
use serde::Deserialize;

use crate::{filesystem::Filesystem, interpolate::Interpolator, lockfile::Lockfile};

use super::Extension;

#[derive(Clone, Deserialize, Debug)]
struct Link {
    from: String,
    to: String,
}

pub struct Symlink {}

impl Symlink {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extension for Symlink {
    fn run(
        &self,
        arguments: String,
        contents: String,
        filesystem: &Filesystem,
        lockfile: &mut Lockfile,
        interpolator: &Interpolator,
    ) -> Result<()> {
        let map: HashMap<String, Link> = toml::from_str(&contents[..]).unwrap();

        for link in map.values() {
            let from = interpolator.interpolate(link.from.as_str());
            let to = interpolator.interpolate(link.to.as_str());

            let path = PathBuf::from(to.clone());
            create_dir_all(path.parent().unwrap()).unwrap();

            let meta = symlink_metadata(path.clone());

            if let Ok(data) = meta {
                if !data.is_symlink() {
                    panic!(
                        "File {} already exists and is not a symlink!",
                        path.to_string_lossy()
                    )
                }

                let real_path = read_link(path.clone()).unwrap();

                if real_path != PathBuf::from(from.clone()) {
                    panic!(
                        "File {} has a symlink that is not pointing to our input",
                        path.to_string_lossy()
                    );
                }

                continue;
            }

            symlink(from, to).unwrap();
        }

        Ok(())
    }
}
