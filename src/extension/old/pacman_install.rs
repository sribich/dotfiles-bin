
use std::{process::Command, io::{Read, BufReader, BufRead}};

use serde::Deserialize;

use super::Extension;

#[derive(Clone, Deserialize, Debug)]
struct Packages {
    required: Vec<String>,
    build: Vec<String>,
    optional: Vec<String>,
}

pub struct PacmanInstall {

}

impl PacmanInstall {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extension for PacmanInstall {
    fn run(&self, arguments: String, contents: String) {
        let packages: Packages = toml::from_str(&contents[..]).unwrap();

        let mut command = Command::new("pacman");

        command
            .arg("-S")
            .arg("--needed")
            .arg("--noconfirm");

        for item in packages.required.iter() {
            command.arg(item);
        }
        for item in packages.build.iter() {
            command.arg(item);
        }

        let mut child = command.spawn().unwrap();

        // {
        //     let stdout = child.stdout.as_mut().unwrap();
        //     let stdout_reader = BufReader::new(stdout);
        //     let stdout_lines = stdout_reader.lines();
        //
        //     for line in stdout_lines {
        //         println!("Read: {:?}", line);
        //     }
        // }

        child.wait().unwrap();
    }
}
