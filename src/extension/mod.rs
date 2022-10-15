use anyhow::Result;

use crate::{filesystem::Filesystem, lockfile::Lockfile, interpolate::Interpolator};

use self::{package_install::PackageInstall, source_install::SourceInstall, symlink::Symlink};

pub mod package_install;
pub mod source_install;
pub mod symlink;

pub trait Extension {
    fn run(
        &self,
        arguments: String,
        contents: String,
        filesystem: &Filesystem,
        lockfile: &mut Lockfile,
        interpolator: &Interpolator,
    ) -> Result<()>;
}

pub fn get_extension(language: &str) -> Option<Box<dyn Extension>> {
    match language {
        "package_install" => Some(Box::new(PackageInstall::new())),
        "source_install" => Some(Box::new(SourceInstall::new())),
        "symlink" => Some(Box::new(Symlink::new())),
        _ => None,
    }
}

pub enum ExtensionError {
    Invalid,
}
