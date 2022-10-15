mod commands;
mod extension;
mod filesystem;
mod interpolate;
mod lockfile;

use anyhow::{Context, Result};
use clap::Parser;
use commands::DeployArgs;

#[derive(Debug, Parser)]
#[command(name = "dotfiles")]
enum Commands {
    Deploy(DeployArgs),
}

fn main() -> Result<()> {
    let command = Commands::parse();

    match command {
        Commands::Deploy(args) => commands::deploy(args),
    }
    .context("Failed to run command")?;

    Ok(())
}
