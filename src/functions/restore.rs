use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

use crate::functions::utils::{log_removing, log_restoring};

pub fn restore_handler(symlink: &Path) -> Result<(), Error> {
    if !symlink.is_symlink() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "Expected a symlink at {}, but found a regular file or directory",
                symlink.display()
            ),
        ));
    }

    restore(symlink)
}

fn restore(symlink_path: &Path) -> Result<(), Error> {
    let target = fs::read_link(symlink_path)?;

    if !target.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Symlink target {} does not exist", target.display()),
        ));
    }

    log_removing(symlink_path);
    fs::remove_file(symlink_path)?;

    log_restoring(&target, symlink_path);
    fs::rename(&target, symlink_path)?;

    Ok(())
}
