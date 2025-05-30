mod functions;
use std::{
    env,
    path::PathBuf,
};
use clap::{Parser, ValueEnum};
use colored::*;
use functions::{track_path, restore, expand_tilde};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    operation: Operation,
    files: Vec<PathBuf>
}

#[derive(Debug, Clone, ValueEnum)]
enum Operation {
    Track,
    Restore
}

const APP_PATH: &str = "~/.repas";

fn main() {
    let raw_path = env::var("APP_PATH").unwrap_or_else(|_| APP_PATH.to_string());
    let app_path = expand_tilde(&raw_path);

    let args: Args = Args::parse();

    for path in &args.files {
        match args.operation {
            Operation::Track => {
                if let Err(error) = track_path(&path, &app_path) {
                    eprintln!("{}", "Failed to track path:".red().bold());
                    eprintln!("  {}", path.display());
                    eprintln!("{}", format!("  {}", error).red());
                };
            },
            Operation::Restore => {
                if let Err(error) = restore(&path) {
                    eprintln!("{}", "Failed to restore path:".red().bold());
                    eprintln!("  {}", path.display());
                    eprintln!("{}", format!("  {}", error).red());
                };
            },
        }
    }
}
