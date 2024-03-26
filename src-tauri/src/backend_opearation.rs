use rusqlite::Connection;
use tauri::State;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;

use libs::{crud, db::{self, base_crud::Mapping}, utils, hard_link_create};
pub struct AppState {
    pub conn: Arc<Mutex<Connection>>,
}

// 创建新映射
#[tauri::command]
pub async fn handle_selected_path(state: State<'_, AppState>, selected: Vec<String>) -> Result<(), String>{
    // 用于对前端所选中的文件进行提取路径，并生成随机的URL来进行路径的映射
    println!("收到文件: {:?}", selected);

    for path in selected.clone() {
        // 获取文件路径和随机URL
        if let Some(file_name) = std::path::Path::new(&path).file_name().and_then(|f| f.to_str()) {
            let random_url = utils::generate_random_url();
            let mut _hard_link = String::from("");
            match hard_link_create(&path) {
                Ok(link) => {
                    println!("已为该文件创建硬链接: {}", file_name);
                    _hard_link = link.clone()
                },
                Err(e) => return Err(format!("Failed to create hard link: {}", e)),
            }
            println!("生成随机URL: {} for file: {}", random_url, file_name);
            let conn = state.conn.lock().unwrap();
            // 创建映射
            match crud::create_mapping(&*conn, file_name, &path, &random_url, &_hard_link){
                Ok(_) => println!("已创建映射文件: {}", file_name),
                Err(e) => eprintln!("创建映射文件失败: {}: {}", file_name, e),
            }
        } else {
            eprintln!("无法从路径中提取文件名: {}", path);
        }
    }

    Ok(())
}

// 用于让前端获取全部映射
#[tauri::command]
pub async fn get_mappings(state: State<'_, AppState>) -> Result<Vec<Mapping>, String> {
    let conn = state.conn.lock().unwrap();

    let mappings = db::base_crud::get_all_mappings(&*conn);

    match mappings {
        Ok(..) => mappings,
        Err(e) => Err(e.to_string())
    }

}

// 根据URL删除映射和对应的硬链接
#[tauri::command]
pub async fn delete_mapping(state: State<'_, AppState>, url: String) -> Result<(), String> {
    let conn = state.conn.lock().unwrap();

    // 首先尝试获取硬链接的路径
    let hard_link_path = match db::base_crud::get_hard_link_by_url(&*conn, &url) {
        Ok(path) => path,
        Err(e) => return Err(e.to_string()),
    };

    // 删除数据库中的映射
    match db::base_crud::delete_mapping_by_url(&*conn, &url) {
        Ok(..) => (),
        Err(e) => return Err(e.to_string()),
    };

    // 然后删除硬链接文件
    if let Some(path) = hard_link_path {
        let path = PathBuf::from(path);
        if path.exists() {
            if let Err(e) = fs::remove_file(&path) {
                return Err(e.to_string());
            }
            else {
                println!("文件删除成功: {} ", path.display())
            }
        }
    }

    Ok(())
}