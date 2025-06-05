use crate::functions::{
    restore::restore_handler,
    track::track_handler,
    utils::{
        log_no_files_provided, log_path_resolution_error, log_restore_error, log_track_error,
        resolve_path,
    },
};

use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Operation {
    pub operation_type: OperationType,
    pub files: Vec<PathBuf>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OperationType {
    Track,
    Restore,
}

impl Operation {
    pub fn execute(&self, application_path: &Path) {
        if self.files.is_empty() {
            log_no_files_provided();
            return;
        }

        for file_path in &self.files {
            match resolve_path(file_path) {
                Ok(resolved_path) => {
                    self.operation_type.apply(&resolved_path, application_path);
                }
                Err(error) => {
                    log_path_resolution_error(file_path, &error);
                }
            }
        }
    }
}

impl OperationType {
    fn apply(&self, file_path: &Path, folder: &Path) {
        match self {
            OperationType::Track => {
                if let Err(error) = track_handler(file_path, folder) {
                    log_track_error(file_path, &error);
                }
            }
            OperationType::Restore => {
                if let Err(error) = restore_handler(file_path) {
                    log_restore_error(file_path, &error);
                }
            }
        }
    }
}
