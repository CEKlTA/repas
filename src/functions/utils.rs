use colored::*;
use std::{
    env, fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

pub fn get_track_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("NO HOME ENV VAR?? DUDE WTF");

    let raw_track_path = env::var("REPAS_TRACK_FOLDER").unwrap_or(home_dir + "/.repas");

    PathBuf::from(&raw_track_path)
}

pub fn resolve_path(path: &Path) -> std::io::Result<PathBuf> {
    let parent = path
        .parent()
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Invalid path provided"))?;

    let parent_path = if parent.as_os_str().is_empty() {
        Path::new(".")
    } else {
        parent
    };

    let resolved_parent = fs::canonicalize(parent_path)?;

    Ok(match path.file_name() {
        Some(file_name) => resolved_parent.join(file_name),
        None => resolved_parent,
    })
}

pub fn log_skipping(path: &Path) {
    println!(
        "{} {} {}",
        "Skipping".yellow().bold(),
        path.display().to_string().yellow(),
        "because it is already a symlink and may cause issues".yellow()
    );
}

pub fn log_creating_dir(path: &Path) {
    println!(
        "{} {}",
        "Creating directory:".yellow().bold(),
        path.display()
    );
}

pub fn log_moving(origin: &Path, destination: &Path) {
    println!(
        "\n{} {} {} {}",
        "Moving".cyan().bold(),
        origin.display(),
        "to".cyan(),
        destination.display()
    );
}

pub fn log_creating_symlink(origin: &Path, destination: &Path) {
    println!(
        "{} {} {} {}",
        "Creating symlink from".cyan().bold(),
        origin.display(),
        "to".cyan(),
        destination.display()
    );
}

pub fn log_restoring(origin: &Path, destination: &Path) {
    println!(
        "{} {} {} {}",
        "Restoring".green().bold(),
        origin.display(),
        "to".green(),
        destination.display()
    );
}

pub fn log_removing(path: &Path) {
    println!(
        "\n{} {}",
        "Removing symlink:".green().bold(),
        path.display()
    );
}

pub fn log_no_files_provided() {
    println!("{}", "No files provided".yellow().bold());
}

pub fn log_track_error(path: &Path, error: &Error) {
    eprintln!("\n{}", "Failed to track file".red().bold());
    eprintln!("  Path : {}", path.display());
    eprintln!("  Error: {}", error.to_string().red());
}

pub fn log_restore_error(path: &Path, error: &Error) {
    eprintln!("\n{}", "Failed to restore file".red().bold());
    eprintln!("  Path : {}", path.display());
    eprintln!("  Error: {}", error.to_string().red());
}

pub fn log_path_resolution_error(path: &Path, error: &Error) {
    eprintln!("\n{}", "Could not resolve path".red().bold());
    eprintln!("  Path : {}", path.display());
    eprintln!("  Error: {}", error.to_string().red());
}
