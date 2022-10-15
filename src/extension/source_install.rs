use std::{
    ffi::{OsStr, OsString},
    process::{Command, Stdio},
};

use anyhow::Result;
use clap::Parser;
use git2::{build::CheckoutBuilder, Repository};
use serde::Deserialize;

use crate::{filesystem::Filesystem, lockfile::Lockfile, interpolate::Interpolator};

use super::Extension;

#[derive(Debug, Parser)]
#[command(name = "package_install")]
#[command(about = "", long_about = None)]
struct Args {
    #[arg(long)]
    from: String,
}

pub struct SourceInstall {}

impl SourceInstall {
    pub fn new() -> Self {
        Self {}
    }

    fn parse_args(string_args: String) -> Result<Args> {
        let mut split_args = shlex::split(string_args.trim());

        if let None = split_args {
            unimplemented!();
        }

        let mut fake_argv = vec!["".to_owned()];
        fake_argv.append(&mut split_args.unwrap());

        let args = Args::parse_from(fake_argv);

        Ok(args)
    }
}

impl Extension for SourceInstall {
    fn run(
        &self,
        string_args: String,
        contents: String,
        filesystem: &Filesystem,
        lockfile: &mut Lockfile,
        interpolator: &Interpolator,
    ) -> Result<()> {
        let args = Self::parse_args(string_args)?;

        match &args.from[..] {
            "git" => {
                install_git(contents, filesystem, lockfile)?;
            }
            _ => {
                unimplemented!();
            }
        };

        Ok(())
    }
}

/*
repository = "https://github.com/riverwm/river"
tag = "0.1.3"
commands = [
    "git submodule update --init",
    "zig build -Drelease-safe -Dxwayland --prefix /usr install"
]
executables = [
    "river",
    "riverctl",
    "rivertile",
]
*/

#[derive(Clone, Deserialize, Debug)]
struct GitInstall {
    repository: String,
    tag: String,
    commands: Vec<String>,
}

fn install_git(contents: String, filesystem: &Filesystem, lockfile: &mut Lockfile) -> Result<()> {
    let content: GitInstall = toml::from_str(&contents[..]).unwrap();
    let dirname = content.repository.split('/').last().unwrap();

    filesystem.with_cache(dirname, |path| {
        // Clone Repo
        let repo = match Repository::open(path.clone()) {
            Ok(repo) => repo,
            Err(e) => match e.code() {
                git2::ErrorCode::NotFound => {
                    Repository::clone(&content.repository[..], path.clone())?
                }
                _ => {
                    panic!("{:?}", e);
                }
            },
        };

        // Checkout Tag
        let tag = format!("refs/tags/{}", content.tag);

        repo.set_head(&tag[..])?;
        repo.checkout_head(None)?;

        // Run Commands
        for command in content.commands {
            let pieces = shlex::split(&command[..]).unwrap();

            let mut cmd = Command::new(&pieces[0]);
            cmd.args(&pieces[1..]);
            cmd.current_dir(path.clone());

            let mut child = cmd.spawn().unwrap();
            child.wait().unwrap();
        }

        Ok(())
    })?;

    lockfile.test();

    // if let Some(entry) = self.cache.get(content.repository) {

    // }

    // self.cache.

    // // Repository::clone(commands.repository);

    Ok(())
}
