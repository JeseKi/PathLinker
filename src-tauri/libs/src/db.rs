use rusqlite::{params, Connection, Result};

pub fn connect() -> Result<Connection, rusqlite::Error> { // 修正函数名和Result类型
    let conn = Connection::open("../pathlinker.db")?;

    // 创建一个新表，如果它还不存在的话
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mapping (
            id INTEGER PRIMARY KEY,
            file_name TEXT NOT NULL,
            path TEXT UNIQUE NOT NULL,
            url TEXT UNIQUE NOT NULL
         )",
        params![],
    )?;

    Ok(conn)
}

pub mod base_crud {
    use rusqlite::{params, Connection, Result, OptionalExtension};

    // 删除映射
    pub fn delete_mapping_by_url(conn: &Connection, url: &str) -> Result<()> {
        conn.execute("DELETE FROM mapping WHERE url = ?1", params![url])?;
        Ok(())
    }

    // 更新映射
    pub fn update_mapping(conn: &Connection, id: i32, file_name: Option<&str>, path: Option<&str>) -> Result<()> {
        if let Some(file_name) = file_name {
            conn.execute("UPDATE mapping SET file_name = ?1 WHERE id = ?2", params![file_name, id])?;
        }
        if let Some(path) = path {
            conn.execute("UPDATE mapping SET path = ?1 WHERE id = ?2", params![path, id])?;
        }
        Ok(())
    }

    // 根据文件名获取id
    pub fn get_id_by_file_name(conn: &Connection, file_name: &str) -> Result<Option<i32>> {
        conn.query_row("SELECT id FROM mapping WHERE file_name = ?1", params![file_name], |row| row.get(0)).optional()
    }

    // 根据url获取路径
    pub fn get_path_by_url(conn: &Connection, url: &str) -> Result<Option<String>> {
        conn.query_row(
            "SELECT path FROM mapping WHERE url = ?1",
            params![url],
            |row| row.get(0),
        ).optional()
    }
    
    // 根据id获取映射
    #[derive(Clone, Debug)]
    pub struct Mapping {
        pub id: i32,
        pub file_name: String,
        pub path: String,
        pub url: String,
    }

    pub fn get_mapping_by_id(conn: &Connection, mapping_id: i32) -> Result<Option<Mapping>> {
        conn.query_row("SELECT id, file_name, path, url FROM mapping WHERE id = ?1", params![mapping_id], |row| {
            Ok(Mapping {
                id: row.get(0)?,
                file_name: row.get(1)?,
                path: row.get(2)?,
                url: row.get(3)?,
            })
        }).optional()
    }

    // 获取全部映射
    pub fn get_all_mappings(conn: &Connection) -> Result<Vec<Mapping>> {
        let mut stmt = conn.prepare("SELECT id, file_name, path, url FROM mapping")?;
        let mapping_iter = stmt.query_map(params![], |row| {
            Ok(Mapping {
                id: row.get(0)?,
                file_name: row.get(1)?,
                path: row.get(2)?,
                url: row.get(3)?,
            })
        })?;
    
        let mut mappings = Vec::new();
        for mapping in mapping_iter {
            mappings.push(mapping?);
        }
        Ok(mappings)
    }
    
}