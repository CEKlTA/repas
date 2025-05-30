use std::{
    env,
    fs,
    os::unix::fs::symlink,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};
use colored::*;

pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(stripped);
        }
    }

    PathBuf::from(path)
}

pub fn track_path(origin_path: &Path, destination_path: &Path) -> Result<(), Error> {
    if origin_path.is_symlink() {
        println!(
            "{} {:?} {}",
            "Skipping".yellow().bold(),
            origin_path,
            "as it is already a symlink and it can cause problems".yellow()
        );
        return Ok(());
    }

    let Some(filename) = origin_path.file_name() else {
        return Err(Error::new(ErrorKind::Other, "Failed to get filename"));
    };

    let new_location = destination_path.join(filename);

    if !destination_path.exists() {
        println!(
            "{} {}",
            "Creating directory".yellow().bold(),
            destination_path.display()
        );
        fs::create_dir_all(destination_path)?;
    }

    println!(
        "\n{} {:?} {} {:?}",
        "Moving".cyan().bold(),
        origin_path,
        "to".cyan(),
        new_location
    );
    fs::rename(origin_path, &new_location)?;

    println!(
        "{} {:?} {} {:?}",
        "Creating symlink from".cyan().bold(),
        origin_path,
        "to".cyan(),
        new_location
    );
    symlink(&new_location, origin_path)?;

    Ok(())
}

pub fn restore(symlink_path: &Path) -> Result<(), Error> {
    if symlink_path.is_symlink() {
        let target = fs::read_link(symlink_path)?;

        if !target.exists() {
            return Err(Error::new(ErrorKind::NotFound, format!("Symlink target {} does not exist", target.display())));
        }

        println!(
            "\n{} {:?}",
            "Removing symlink".magenta().bold(),
            symlink_path
        );
        fs::remove_file(symlink_path)?;

        println!(
            "{} {:?} {} {:?}",
            "Restoring".green().bold(),
            target,
            "to".green(),
            symlink_path
        );
        fs::rename(&target, symlink_path)?;
    }

    Ok(())
}