// utils.rs
// Utility/helper functions for MetaSort_v1.0.0 – Google Photos Takeout Organizer 

use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use chrono::Local;

/// Appends a timestamped log entry to a log file in the logs folder inside the given directory.
pub fn log_to_file(log_dir: &Path, log_name: &str, message: &str) {
    let _ = create_dir_all(log_dir);
    let log_path = log_dir.join(log_name);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .expect("Unable to open log file");
    let now = Local::now().format("[%Y-%m-%d %H:%M:%S]");
    let _ = writeln!(file, "{} {}", now, message);
} 