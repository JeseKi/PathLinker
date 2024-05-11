use rusqlite::Connection;

use crate::{
    db,
    utils::{LogType , log_to_file}
};

// 连接到数据库
pub fn connect_db () -> Connection{
    let conn = db::connect();
    match conn {
        Ok(conn) => return conn,
        Err(e) => {
            log_to_file(
                &format!("严重错误！！！数据库连接失败:{}", e), 
                None, 
                LogType::Error
            );
            panic!("Error: {}", e)
        },
    }
}

pub mod mappings{
    use rusqlite::{params, Connection, Result, OptionalExtension};
    use std::path::PathBuf;

    use crate::{db, path_struct, utils::{log_to_file , LogType}};

    /// Gets the corresponding hard_link based on the URL.
    ///
    /// # Parameters
    /// * `conn` - A reference to the database.
    /// * `url` - A reference to the URL.
    ///
    /// # Returns
    /// Returns one of the following three strings:
    /// 1. The hard_link corresponding to the URL (normal query).
    /// 2. "log.txt" (if there is no hard_link corresponding to the URL in the database).
    /// 3. An error string (if an error occurs during the query).
    /// # Example
    /// ```
    /// let conn = connect_db(); // Returns a `rusqlite::Connection` object
    /// let url = "pathlinker://testurl" // Assume the database contains this URL
    /// let hard_link = get_hard_link_by_url(&conn , &url); // Calls the function
    /// println!("hard_link:{}", hard_link) // 1. "/home/test.txt" (assuming the URL corresponds to the hard_link `/home/test.txt`) 2. "./log.txt" (assuming there is no corresponding hard_link in the database) 3. "An error occurred during the query" (assuming an error occurs during the query)
    /// ```

    pub fn get_hard_link_by_url(conn: &Connection, url: &str) -> String {

        // 定义特殊日志路径
        let mut log_path = dirs::data_local_dir().unwrap_or_else(|| {
            eprintln!("Unable to find data local directory");
            PathBuf::new()
        });

        log_path.push("pathlinker");
        std::fs::create_dir_all(&log_path).expect("Failed to create directory for log file");
        log_path.push("log.txt");

        // 根据URL获取硬链接
        match db::mapping_base_crud::get_hard_link_by_url(conn, url) {
            Ok(Some(hard_link)) => hard_link,
            Ok(None) => {
                // 写入日志
                log_to_file(
                    &format!("Error: No hard_link found for url: {}", url),
                    None , 
                    LogType::Info
                );
                log_path.to_str().unwrap_or_else(|| "无法转换路径为字符串").to_string()
            },
            
            Err(_) => {
                log_to_file("Error: Failed to get hard_link by url", None , LogType::Warning);
                "An error occurred during the query".to_string()
            },  // Returns an error message when an error occurs during the query
        }

    }

    /// Creates a mapping between a file name, path, URL , and hard_link in the database.
    ///
    /// # Parameters
    /// - `conn`: The database connection handle.
    /// - `file_name`: The name of the file to be mapped.
    /// - `path`: The local file system path to be associated with the file.
    /// - `url`: The URL to be associated with the file.
    /// - `hard_link` The hard_link to orign file
    ///
    /// # Returns
    /// - `Ok()`: If the mapping is created successfully.
    ///
    /// # Errors
    /// This function will return an error if any SQLite operation fails.
    ///
    /// # Example
    /// ```
    /// use rusqlite::Connection;
    ///
    /// let connection: Connection = connect_db();
    /// let file_name = "example.txt";
    /// let path = "/files/example.txt";
    /// let url = "pathlinker://test";
    ///
    /// match create_mapping(&connection, file_name, path, url) {
    ///     Ok() => println!("Mapping created successfully."),
    ///     Err(e) => println!("Error creating mapping: {}", e),
    /// }
    /// ```

    pub fn create_mapping(conn: &Connection, file_name: &str, origin_path: &str, url: &str, hard_link: &str) -> Result<()> {

        let mut mapping_url: String = url.to_string();
        let existing_url: Option<String> = conn.query_row(
            "SELECT url FROM mapping WHERE url = ?1",
            params![url],
            |row| row.get(0),
        ).optional()?;

        if existing_url.is_some() {
            mapping_url = path_struct::generate_random_url();
        }

        // 创建新的映射并保存到数据库
        conn.execute(
            "INSERT INTO mapping (file_name, origin_path, url, hard_link) VALUES (?1, ?2, ?3, ?4)",
            params![file_name, origin_path, &mapping_url, hard_link],
        )?;

        Ok(())
    }
}