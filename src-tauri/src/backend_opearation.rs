use rusqlite::Connection;
use tauri::State;
use std::sync::{Arc, Mutex};

use libs::{crud, db::{self, base_crud::Mapping}, utils, hard_link_create};
pub struct AppState {
    pub conn: Arc<Mutex<Connection>>,
}

// 创建新映射
#[tauri::command]
pub async fn handle_selected_path(state: State<'_, AppState>, selected: Vec<String>) -> Result<(), String>{
    // 用于对前端所选中的文件进行提取路径，并生成随机的URL来进行路径的映射
    println!("Received selected path: {:?}", selected);

    for path in selected.clone() {
        // 获取文件路径和随机URL
        if let Some(file_name) = std::path::Path::new(&path).file_name().and_then(|f| f.to_str()) {
            let random_url = utils::generate_random_url();
            let mut _hard_link = String::from("");
            match hard_link_create(&path) {
                Ok(link) => {
                    println!("Hard link created for file: {}", file_name);
                    _hard_link = link.clone()
                },
                Err(e) => return Err(format!("Failed to create hard link: {}", e)),
            }
            println!("Generated URL: {} for file: {}", random_url, file_name);
            let conn = state.conn.lock().unwrap();
            // 创建映射
            match crud::create_mapping(&*conn, file_name, &path, &random_url, &_hard_link){
                Ok(_) => println!("Mapping created for file: {}", file_name),
                Err(e) => eprintln!("Failed to create mapping for file: {}: {}", file_name, e),
            }
        } else {
            eprintln!("Failed to extract file name from path: {}", path);
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

// 根据URL删除映射
#[tauri::command]
pub async fn delete_mapping (state: State<'_, AppState> , url: String) -> Result<(), String> {
  let conn = state.conn.lock().unwrap();
  let flag = db::base_crud::delete_mapping_by_url(&*conn, &url);
  match flag {
    Ok(..) => Ok(()),
    Err(e) => Err(e.to_string())
  }
}