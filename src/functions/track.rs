use crate::functions::utils::{log_creating_symlink, log_moving, log_skipping};

use std::{
    fs,
    io::{Error, ErrorKind},
    os::unix::fs::symlink,
    path::Path,
};

pub fn track_handler(from: &Path, to: &Path) -> Result<(), Error> {
    if from.is_symlink() {
        log_skipping(from);
        return Ok(());
    }

    track_path(from, to)
}

fn track_path(origin: &Path, destination: &Path) -> Result<(), Error> {
    let Some(filename) = origin.file_name() else {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Could not extract filename from path: {}", origin.display()),
        ));
    };

    let new_location = destination.join(filename);

    if new_location.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("Target path {} already exists", new_location.display()),
        ));
    }

    log_moving(origin, &new_location);
    fs::rename(origin, &new_location)?;

    log_creating_symlink(origin, &new_location);
    symlink(&new_location, origin)?;

    Ok(())
}
