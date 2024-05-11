use std::fs::{OpenOptions , create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;
use std::sync::{Arc, Mutex};
use tauri::Window;
use rusqlite::Connection;

pub struct AppState {
    pub conn: Arc<Mutex<Connection>>,
    pub window: Window
}

pub enum LogType {
    Error,
    Warning,
    Debug,
    Info,
}

impl LogType {
    fn as_str(&self) -> &'static str {
        match self {
            LogType::Error => "[ERROR]",
            LogType::Warning => "[WARN]",
            LogType::Debug => "[DEBUG]",
            LogType::Info => "[INFO]",
        }
    }
}

/// 将日志记录到文件中
/// 
/// 参数:
/// - message: 要记录的消息
/// - path: 可选的日志文件路径, 如果不提供则使用默认路径
/// - log_type: 日志类型
///     - LogType::Error: 错误日志
///     - LogType::Warning: 警告日志
///     - LogType::Debug: 调试日志
///     - LogType::Info: 普通信息日志
pub fn log_to_file(message: &str, path: Option<&PathBuf>, log_type: LogType) {
    let now = Local::now();

    let log_file_path = if let Some(p) = path {
        p.clone()
    } else {
        let mut default_path = dirs::data_local_dir().expect("Failed to find home directory");
        default_path.push("pathlinker");
        create_dir_all(&default_path).expect("Failed to create directory for log file");
        default_path.push("log.txt");
        default_path
    };

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .expect("Failed to open log file");

    writeln!(file, "{} {}\n[TIME]  {}\n", log_type.as_str(), message, now.format("%Y-%m-%d %H:%M:%S")).expect("Failed to write to log file");
    println!("{} {}\n[TIME]  {}\n", log_type.as_str(), message, now.format("%Y-%m-%d %H:%M:%S"));
}
 
// 通过前端显示错误信息
// #[tauri::command]
// pub fn send_error_message(window: &Window, message: String) {
//     window.emit("error-message", &message).expect("failed to send error message");
// }
