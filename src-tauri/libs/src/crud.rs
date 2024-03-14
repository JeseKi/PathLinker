use rusqlite::{params, Connection, Result, OptionalExtension};

    use crate::db::base_crud::Mapping;
    use super::{db , utils};

    // 连接到数据库
    pub fn connect_db () -> Connection{
        let conn = db::connect();
        match conn {
            Ok(conn) => return conn,
            Err(e) => panic!("Error: {}", e),
        }
    }

    // 根据url获取路径
    pub fn get_filepath_by_url(conn: &Connection, url: &str) -> String {
        match db::base_crud::get_path_by_url(conn, url) {
            Ok(Some(path)) => path,
            Ok(None) => "".to_string(),  // 当URL没有对应的路径时返回空字符串
            Err(_) => "查询过程中出现错误".to_string(),  // 当查询出现错误时返回错误信息
        }
    }
    
    // 获取全部映射
    pub fn get_mappings(conn: &Connection) -> Vec<Mapping> {
        match db::base_crud::get_all_mappings(conn) {
            Ok(mappings) => mappings,  // 如果成功获取映射，直接返回这些映射
            Err(_) => Vec::new(),      // 如果出现错误，返回一个空的向量
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