mod args;
mod functions;

use args::Operation;
use functions::utils::{get_track_path, log_creating_dir};

use clap::Parser;
use std::fs;

fn main() {
    let track_path = get_track_path();

    if !track_path.exists() {
        log_creating_dir(&track_path);
        fs::create_dir_all(&track_path).expect("Couldn't create tracking folder");
    }

    Operation::parse().execute(&track_path);
}
