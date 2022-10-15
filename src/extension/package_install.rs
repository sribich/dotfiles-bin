use std::{
    io::{BufRead, BufReader, Read},
    process::{Command, Stdio},
};

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;

use crate::{filesystem::Filesystem, lockfile::Lockfile, interpolate::Interpolator};

use super::Extension;

#[derive(Debug, Parser)]
#[command(name = "package_install")]
#[command(about = "", long_about = None)]
struct Args {
    package_manager: String,
}

#[derive(Clone, Deserialize, Debug)]
struct Packages {
    required: Vec<String>,
    #[serde(default)]
    build: Vec<String>,
    #[serde(default)]
    optional: Vec<String>,
}

pub struct PackageInstall {}

impl PackageInstall {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extension for PackageInstall {
    fn run(
        &self,
        arguments: String,
        contents: String,
        filesystem: &Filesystem,
        lockfile: &mut Lockfile,
        interpolator: &Interpolator,
    ) -> Result<()> {
        let mut split = shlex::split(arguments.trim()).unwrap();
        let mut args = vec!["".to_owned()];

        args.append(&mut split);

        let args = Args::parse_from(args);

        let packages: Packages = toml::from_str(&contents[..]).unwrap();

        let mut command = Command::new("sudo");

        command
            .arg("pacman")
            .arg("-S")
            .arg("--needed")
            .arg("--noconfirm");

        for item in packages.required.iter() {
            command.arg(item);
        }
        for item in packages.build.iter() {
            command.arg(item);
        }

        let mut status = command.status().unwrap();
        // {
        //     let stdout = child.stdout.as_mut().unwrap();
        //     let stdout_reader = BufReader::new(stdout);
        //     let stdout_lines = stdout_reader.lines();
        //
        //     for line in stdout_lines {
        //         println!("Read: {:?}", line);
        //     }
        // }

        // child.wait().unwrap();
        //println!("{:?}", "here??");
        Ok(())
    }
}
