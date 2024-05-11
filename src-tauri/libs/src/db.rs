use rusqlite::{Connection, params,Result};
use std::error::Error;
use dirs;

pub fn connect() -> Result<Connection, Box<dyn Error>> {
    let mut db_path = dirs::data_local_dir().ok_or("Unable to find data local directory")?;
    db_path.push("pathlinker");
    db_path.push("pathlinker.db");

    if let Err(e) = std::fs::create_dir_all(db_path.parent().unwrap()) {
        eprintln!("Failed to create directory: {}", e);
        return Err(Box::new(e));
    }
    println!("Database path: {:?}", db_path);

    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mapping (
            id INTEGER PRIMARY KEY,
            file_name TEXT NOT NULL,
            origin_path TEXT NOT NULL,
            url TEXT UNIQUE NOT NULL,
            hard_link TEXT UNIQUE NOT NULL
         )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mapping_folders (
            mapping_id INTEGER,
            folder_name TEXT UNIQUE NOT NULL,
            FOREIGN KEY(mapping_id) REFERENCES mapping(id)
        )",
        params![],
    )?;    

    Ok(conn)

}

pub mod mapping_base_crud {
    use rusqlite::{params, Connection, Result, OptionalExtension};
    use serde::{Serialize, Deserialize};

    use crate::utils::{log_to_file , LogType};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Mapping {
        pub id: i32,
        pub file_name: String,
        pub origin_path: String,
        pub url: String,
        pub hand_link: String,
    }

    // 根据URL删除映射
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

    // 根据url获取硬链接
    pub fn get_hard_link_by_url(conn: &Connection, url: &str) -> Result<Option<String>> {
        // debgug: 在查找时获取全部路径
        let db_mappings = super::mapping_base_crud::get_all_mappings(conn);
        match db_mappings {
            Ok(mappings) => {
                log_to_file(
                    &format!("mapping_base_crud_get_filepath_by_url: Found mappings:{:?}", mappings), 
                    None, 
                    LogType::Info
                );
            }
            
            Err(e) => {
                log_to_file(
                    &format!("Failed to get mappings: {}", e), 
                    None, 
                    LogType::Warning
                );
            }
        }
        log_to_file(
            &format!("Getting filepath by url: {}\nlen:{}", url, url.len()), 
            None, 
            LogType::Info
        );
        conn.query_row(
            "SELECT hard_link FROM mapping WHERE url = ?1",
            params![url],
            |row| row.get(0),
        ).optional()
    }
    
    // 根据id获取映射
    pub fn get_mapping_by_id(conn: &Connection, mapping_id: i32) -> Result<Option<Mapping>> {
        conn.query_row("SELECT id, file_name, origin_path, url , hand_link FROM mapping WHERE id = ?1", params![mapping_id], |row| {
            Ok(Mapping {
                id: row.get(0)?,
                file_name: row.get(1)?,
                origin_path: row.get(2)?,
                url: row.get(3)?,
                hand_link: row.get(4)?,
            })
        }).optional()
    }

    // 获取全部映射
    pub fn get_all_mappings(conn: &Connection) -> Result<Vec<Mapping>, String> {
        let mut stmt = conn.prepare("SELECT id, file_name, origin_path, url , hard_link FROM mapping").expect("表格获取失败");
        let mapping_iter = stmt.query_map(params![], |row| {
            Ok(Mapping {
                id: row.get(0).expect("ID获取失败"),
                file_name: row.get(1).expect("文件名获取失败"),
                origin_path: row.get(2).expect("路径获取失败"),
                url: row.get(3).expect("URL获取失败"),
                hand_link: row.get(4).expect("硬链接获取失败"),
            })
        }).expect("获取数据库映射失败！");
    
        let mut mappings = Vec::new();
        for mapping in mapping_iter {
            mappings.push(mapping.expect("映射添加失败"));
        }
        Ok(mappings)
    }
    
}

pub mod folder_base_crud {
    use rusqlite::{params, Connection, Result};
    use serde::{Serialize, Deserialize};

    #[derive(Debug , Serialize, Deserialize)]
    pub struct Folders {
        pub mapping_id: Option<i32>,
        pub folder_name: String,
    }

    // 创建文件夹
    pub fn create_folder(conn: &Connection, folder_name: &str, mapping_id: Option<i32>) -> Result<usize> {
        println!("获取到文件夹名称:{}", folder_name);
        // 根据mapping_id是否存在，执行不同的插入操作
        match mapping_id {
            // 如果mapping_id存在，则插入到mapping_folders表中，mapping_id和folder_name作为参数传入
            Some(id) => {
                conn.execute(
                    "INSERT INTO mapping_folders (mapping_id, folder_name) VALUES (?1, ?2)",
                    params![id, folder_name],
                )
            }
            // 否则，只插入folder_name
            None => {
                conn.execute(
                    "INSERT INTO mapping_folders (folder_name) VALUES (?1)",
                    params![folder_name],
                )
            }
        }
    }
    
    // 根据文件夹名称获取映射
    pub fn get_mappings_for_folder(conn: &Connection, folder_name: &str) -> Result<Vec<i32>> {
        // 准备查询语句，获取文件夹名称对应的映射id
        let mut stmt = conn.prepare("SELECT mapping_id FROM mapping_folders WHERE folder_name = ?1")?;
        // 查询，获取结果集，并将其转换为Result<Vec<i32>>
        let mapping_ids = stmt.query_map(params![folder_name], |row| row.get(0))?
                             .collect::<Result<Vec<i32>, _>>()?;
        // 返回结果集
        Ok(mapping_ids)
    }
    
    // 根据映射id获取所在的全部文件夹
    pub fn get_folders_for_mapping(conn: &Connection, mapping_id: i32) -> Result<Vec<String>> {
        // 准备查询语句，获取指定映射id的文件夹名称
        let mut stmt = conn.prepare("SELECT folder_name FROM mapping_folders WHERE mapping_id = ?1")?;
        // 查询，获取结果集，并将其转换为Result<Vec<String>>
        let folders = stmt.query_map(params![mapping_id], |row| row.get(0))?
                          .collect::<Result<Vec<String>, _>>()?;
        // 返回结果集
        Ok(folders)
    }

    // 从文件夹中移除映射
    pub fn remove_mappings_from_folder(conn: &Connection, folder_name: &str, mapping_ids: Vec<i32>) -> Result<usize> {
        // 移除的映射数量
        let mut removed_count = 0;
        // 遍历指定的映射id，从文件夹中移除
        for mapping_id in mapping_ids {
            // 执行删除语句，从数据库中移除
            removed_count += conn.execute(
                "DELETE FROM mapping_folders WHERE folder_name = ?1 AND mapping_id = ?2",
                params![folder_name, mapping_id],
            )?;
        }
        // 返回移除的数量
        Ok(removed_count)
    }

    // 从映射中移除文件夹
    pub fn remove_folders_from_mapping(conn: &Connection, mapping_id: i32, folders: Vec<String>) -> Result<usize> {
        // 移除的文件夹数量
        let mut removed_count = 0;
        // 遍历指定的文件夹，从映射中移除
        for folder in folders {
            // 执行删除语句，从数据库中移除
            removed_count += conn.execute(
                "DELETE FROM mapping_folders WHERE mapping_id = ?1 AND folder_name = ?2",
                params![mapping_id, folder],
            )?;
        }
        // 返回移除的数量
        Ok(removed_count)
    }

    // 删除文件夹
    pub fn delete_folder(conn: &Connection, folder_name: &str) -> Result<usize> {
        // 执行删除语句，从数据库中移除
        conn.execute("DELETE FROM mapping_folders WHERE folder_name = ?1", params![folder_name])
    }

    // 获取全部文件夹数据
    pub fn get_all_folders(conn: &Connection) -> Result<Vec<Folders>> {
        let mut stmt = conn.prepare("SELECT mapping_id, folder_name FROM mapping_folders")?;
        let folder_mappings = stmt.query_map([], |row| {
            Ok(Folders {
                mapping_id: row.get(0)?,
                folder_name: row.get(1)?
            })
        })?.collect::<Result<Vec<Folders>, _>>()?;

        Ok(folder_mappings)
    }

    // tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use rusqlite::{Connection, Result};
        use std::collections::HashSet;

        #[test]
        fn test_create_folder() -> Result<()> {
            let conn = Connection::open_in_memory()?;
            conn.execute(
                "CREATE TABLE mapping_folders (
                    mapping_id INTEGER,
                    folder_name TEXT
                )",
                [],
            )?;
    
            assert_eq!(create_folder(&conn, "folder1", None)?, 1);
            assert_eq!(create_folder(&conn, "folder2", Some(1))?, 1);
    
            Ok(())
        }
    
        #[test]
        fn test_get_mappings_for_folder() -> Result<()> {
            let conn = Connection::open_in_memory()?;
            conn.execute(
                "CREATE TABLE mapping_folders (
                    mapping_id INTEGER,
                    folder_name TEXT
                )",
                [],
            )?;
        
            create_folder(&conn, "folder1", None)?;
            create_folder(&conn, "folder1", Some(1))?;
            create_folder(&conn, "folder1", Some(2))?;
        
            let mapping_ids = get_mappings_for_folder(&conn, "folder1")?;
            let mut expected_ids: HashSet<i32> = HashSet::new();
            expected_ids.insert(0);
            expected_ids.insert(1);
            expected_ids.insert(2);
        
            assert_eq!(mapping_ids.len(), expected_ids.len());
            for id in mapping_ids {
                assert!(expected_ids.contains(&id));
            }
        
            Ok(())
        }        
    
        #[test]
        fn test_get_folders_for_mapping() -> Result<()> {
            let conn = Connection::open_in_memory()?;
            conn.execute(
                "CREATE TABLE mapping_folders (
                    mapping_id INTEGER,
                    folder_name TEXT
                )",
                [],
            )?;
    
            create_folder(&conn, "folder1", Some(1))?;
            create_folder(&conn, "folder2", Some(1))?;
            create_folder(&conn, "folder3", Some(2))?;
    
            let folders = get_folders_for_mapping(&conn, 1)?;
            assert_eq!(folders.len(), 2);
            assert!(folders.contains(&"folder1".to_string()));
            assert!(folders.contains(&"folder2".to_string()));
    
            let folders = get_folders_for_mapping(&conn, 2)?;
            assert_eq!(folders.len(), 1);
            assert!(folders.contains(&"folder3".to_string()));
    
            Ok(())
        }
    
        #[test]
        fn test_remove_mappings_from_folder() -> Result<()> {
            let conn = Connection::open_in_memory()?;
            conn.execute(
                "CREATE TABLE mapping_folders (
                    mapping_id INTEGER,
                    folder_name TEXT
                )",
                [],
            )?;
    
            create_folder(&conn, "folder1", Some(1))?;
            create_folder(&conn, "folder1", Some(2))?;
            create_folder(&conn, "folder1", Some(3))?;
    
            assert_eq!(remove_mappings_from_folder(&conn, "folder1", vec![1, 3])?, 2);
    
            let mapping_ids = get_mappings_for_folder(&conn, "folder1")?;
            assert_eq!(mapping_ids.len(), 1);
            assert!(mapping_ids.contains(&2));
    
            Ok(())
        }
    
        #[test]
        fn test_remove_folders_from_mapping() -> Result<()> {
            let conn = Connection::open_in_memory()?;
            conn.execute(
                "CREATE TABLE mapping_folders (
                    mapping_id INTEGER,
                    folder_name TEXT
                )",
                [],
            )?;
    
            create_folder(&conn, "folder1", Some(1))?;
            create_folder(&conn, "folder2", Some(1))?;
            create_folder(&conn, "folder3", Some(1))?;
    
            assert_eq!(remove_folders_from_mapping(&conn, 1, vec!["folder1".to_string(), "folder3".to_string()])?, 2);
    
            let folders = get_folders_for_mapping(&conn, 1)?;
            assert_eq!(folders.len(), 1);
            assert!(folders.contains(&"folder2".to_string()));
    
            Ok(())
        }
    
        #[test]
        fn test_delete_folder() -> Result<()> {
            let conn = Connection::open_in_memory()?;
            conn.execute(
                "CREATE TABLE mapping_folders (
                    mapping_id INTEGER,
                    folder_name TEXT
                )",
                [],
            )?;
    
            create_folder(&conn, "folder1", Some(1))?;
            create_folder(&conn, "folder2", Some(2))?;
    
            assert_eq!(delete_folder(&conn, "folder1")?, 1);
    
            let folders = get_folders_for_mapping(&conn, 1)?;
            assert!(folders.is_empty());
    
            let folders = get_folders_for_mapping(&conn, 2)?;
            assert_eq!(folders.len(), 1);
            assert!(folders.contains(&"folder2".to_string()));
    
            Ok(())
        }  

        // 函数来设置测试用的数据库环境
        fn setup_test_db() -> Result<Connection> {
            let conn = Connection::open_in_memory()?;  // 创建内存中的数据库
            conn.execute(
                "CREATE TABLE mapping_folders (
                    mapping_id INTEGER,
                    folder_name TEXT UNIQUE NOT NULL,
                    FOREIGN KEY(mapping_id) REFERENCES mapping(id)
                )",
                [],
            )?;
            Ok(conn)
        }

    }
    
}