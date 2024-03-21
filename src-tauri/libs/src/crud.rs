use rusqlite::{params, Connection, Result, OptionalExtension};
use std::path::PathBuf;
use super::{db , utils};

// 连接到数据库
pub fn connect_db () -> Connection{
    let conn = db::connect();
    match conn {
        Ok(conn) => return conn,
        Err(e) => panic!("Error: {}", e),
    }
}

/// Gets the corresponding path based on the URL.
///
/// # Parameters
/// * `conn` - A reference to the database.
/// * `url` - A reference to the URL.
///
/// # Returns
/// Returns one of the following three strings:
/// 1. The path corresponding to the URL (normal query).
/// 2. "log.txt" (if there is no path corresponding to the URL in the database).
/// 3. An error string (if an error occurs during the query).
/// # Example
/// ```
/// let conn = connect_db(); // Returns a `rusqlite::Connection` object
/// let url = "pathlinker://testurl" // Assume the database contains this URL
/// let path = get_filepath_by_url(&conn , &url); // Calls the function
/// println!("path:{}", path) // 1. "/home/test.txt" (assuming the URL corresponds to the path `/home/test.txt`) 2. "./log.txt" (assuming there is no corresponding path in the database) 3. "An error occurred during the query" (assuming an error occurs during the query)
/// ```
pub fn get_filepath_by_url(conn: &Connection, url: &str) -> String {
    let mut log_path = dirs::data_local_dir().unwrap_or_else(|| {
        eprintln!("Unable to find data local directory");
        PathBuf::new()
    });
    log_path.push("pathlinker");
    std::fs::create_dir_all(&log_path).expect("Failed to create directory for log file");
    log_path.push("log.txt");
    match db::base_crud::get_path_by_url(conn, url) {
        Ok(Some(path)) => path,
        Ok(None) => {
            // 写入日志
            utils::log_to_file(&format!("Error: No filepath found for url: {}", url), None);
            log_path.to_str().unwrap_or_else(|| "无法转换路径为字符串").to_string()
        },
        
        Err(_) => {
            utils::log_to_file("Error: Failed to get filepath by url", None);
            "An error occurred during the query".to_string()
        },  // Returns an error message when an error occurs during the query
    }
}

// 创建映射
pub fn create_mapping(conn: &Connection, file_name: &str, path: &str, url: &str) -> Result<bool> {

    // 检查路径是否已存在
    let existing_path: Option<String> = conn.query_row(
        "SELECT path FROM mapping WHERE path = ?1",
        params![path],
        |row| row.get(0),
    ).optional()?;

    if existing_path.is_some() {
        // 如果路径已存在，返回错误
        return Ok(false); // 在实际应用中，你可能想返回一个错误而不是打印一条消息
    }

    // 检查URL是否已存在
    let mut final_url: String = url.to_string();
    let existing_url: Option<String> = conn.query_row(
        "SELECT url FROM mapping WHERE url = ?1",
        params![url],
        |row| row.get(0),
    ).optional()?;

    if existing_url.is_some() {
        // 如果URL已存在，则生成一个新的URL
        final_url = utils::generate_random_url();
    }

    // 创建新的映射并保存到数据库
    conn.execute(
        "INSERT INTO mapping (file_name, path, url) VALUES (?1, ?2, ?3)",
        params![file_name, path, &final_url],
    )?;

    Ok(true)
}