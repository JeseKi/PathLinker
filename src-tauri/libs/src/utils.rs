use rand::{distributions::Alphanumeric, Rng};
use std::fs::OpenOptions;
use std::io::Write;

pub fn generate_random_url() -> String {
    let random_path: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    format!("pathlinker://{}", random_path)
}

pub fn log_to_file(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .expect("Failed to open log file");

    writeln!(file, "{}", message).expect("Failed to write to log file");
}