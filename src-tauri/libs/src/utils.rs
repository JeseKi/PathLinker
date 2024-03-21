use rand::{distributions::Alphanumeric, Rng};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;

pub fn generate_random_url() -> String {
    let random_path: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    format!("pathlinker://{}/", random_path)
}

/// Writes a message to a log file at the specified path, or to a default path if none is provided.
///
/// # Parameters
/// * `message` - The message to be written to the log file.
/// * `path` - An optional path for the log file. If None, the function uses the default path in the user's directory.
///
/// # Panics
/// This function will panic if it fails to open the log file or if it fails to write the message to the log file.
///
/// # Example
/// ```rust
/// use std::path::PathBuf;
///
/// log_to_file("Application started successfully", None); // Will use the default path
///
/// let custom_path = PathBuf::from("/path/to/custom/log.txt");
/// log_to_file("User logged in", Some(&custom_path)); // Will use the specified path
/// ```
pub fn log_to_file(message: &str, path: Option<&PathBuf>) {
    let now = Local::now();
    let log_file_path = if let Some(p) = path {
        p.clone()
    } else {
        let mut default_path = dirs::data_local_dir().expect("Failed to find home directory");
        default_path.push("pathlinker");
        // 确保日志文件目录存在
        std::fs::create_dir_all(&default_path).expect("Failed to create directory for log file");
        default_path.push("log.txt");
        default_path
    };

    // println!("log_path:{log_file_path:?}");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .expect("Failed to open log file");

    writeln!(file, "[INFO]\n{}\n[TIME]          {}\n", message, now.format("%Y-%m-%d %H:%M:%S")).expect("Failed to write to log file");
}