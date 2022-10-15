use std::{fs, path::Path};

use anyhow::Result;
use clap::{arg, Parser};
use orgize::{elements::SourceBlock, Element, Event, Org};

use crate::{
    extension::get_extension, filesystem::Filesystem, interpolate::get_interpolator,
    lockfile::Lockfile,
};

#[derive(Debug, Parser)]
#[command()]
pub struct DeployArgs {
    #[arg(long, name = "dotfiles-dir")]
    dotfiles_dir: String,
}

pub fn deploy(args: DeployArgs) -> Result<()> {
    let dotfiles_root = Path::new(&args.dotfiles_dir[..]);
    let dotfiles_file = dotfiles_root.join("README.org");

    if !dotfiles_file.exists() {
        panic!(
            "Could not find README.org at {}",
            dotfiles_file.to_str().unwrap()
        );
    }

    let interpolator = get_interpolator(&dotfiles_root.canonicalize().unwrap());
    let filesystem = Filesystem::new();
    let mut lockfile = Lockfile::new(filesystem.clone()).unwrap();

    let content = fs::read_to_string(dotfiles_file).unwrap();

    for event in Org::parse(&content[..]).iter() {
        if let Event::Start(Element::SourceBlock(SourceBlock {
            arguments,
            contents,
            language,
            post_blank,
        })) = event
        {
            let extension = get_extension(language)
                .unwrap_or_else(|| panic!("No extension exists for {}", language));
            extension.run(
                arguments.to_string(),
                contents.to_string(),
                &filesystem,
                &mut lockfile,
                &interpolator,
            )?;
        }
    }

    Ok(())
}
