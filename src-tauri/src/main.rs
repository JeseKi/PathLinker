#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{os::unix::process, sync::{Arc, Mutex}};
use backend_opearation as bo;
use libs::crud;

fn main() {
    let connection = Arc::new(Mutex::new(crud::connect_db()));

    let app_state = bo::AppState {
        conn: connection,
    };

    // 处理自定义协议
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let url = &args[1];
        let conn = crud::connect_db();
        println!("url:{url}");
        let flag = bo::open_file(&conn, url);
        match flag {
            Ok(..) => std::process::exit(0),
            Err(..) => std::process::exit(1),
        }
    }

    // 程序运行
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![bo::handle_selected_path, bo::get_mappings, bo::delete_mapping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 与前端交互的一系列操作
mod backend_opearation {
    use open;
    use rusqlite::Connection;
    use tauri::State;
    use std::sync::{Arc, Mutex};
    
    use libs::{crud, db::{self, base_crud::Mapping}, utils};
    pub struct AppState {
        pub conn: Arc<Mutex<Connection>>,
    }

    // 创建新映射
    #[tauri::command]
    pub async fn handle_selected_path(state: State<'_, AppState>, selected: Vec<String>) -> Result<(), String>{
        println!("Received selected path: {:?}", selected);
    
        for path in selected.clone() {
            if let Some(file_name) = std::path::Path::new(&path).file_name().and_then(|f| f.to_str()) {
                let random_url = utils::generate_random_url();
                println!("Generated URL: {} for file: {}", random_url, file_name);
                // 获取数据库连接
                let conn = state.conn.lock().unwrap();
                // 创建映射
                match crud::create_mapping(&*conn, file_name, &path, &random_url) {
                    Ok(_) => println!("Mapping created for file: {}", file_name),
                    Err(e) => eprintln!("Failed to create mapping for file: {}: {}", file_name, e),
                }
            } else {
                eprintln!("Failed to extract file name from path: {}", path);
            }
        }
    
        Ok(())
    }
    
    // 获取全部映射
    #[tauri::command]
    pub async fn get_mappings(state: State<'_, AppState>) -> Result<Vec<Mapping>, String> {
        let conn = state.conn.lock().unwrap();
    
        let mappings = db::base_crud::get_all_mappings(&*conn);
    
        match mappings {
            Ok(..) => mappings,
            Err(e) => Err(e.to_string())
        }
    
    }

    // 删除映射
    #[tauri::command]
    pub async fn delete_mapping (state: State<'_, AppState> , url: String) -> Result<(), String> {
      let conn = state.conn.lock().unwrap();
      let flag = db::base_crud::delete_mapping_by_url(&*conn, &url);
      match flag {
        Ok(..) => Ok(()),
        Err(e) => Err(e.to_string())
      }
    }

    // 打开文件
    pub fn open_file(conn: &Connection, url: &String) -> Result<(), String> {
        let path = crud::get_filepath_by_url(conn, url);
        let flag = open::that(&path);

        match flag {
            Ok(..) => Ok(()),
            Err(e) => {
                println!("Failed to open the file: {}", e);
                Err(e.to_string())
            }
        }
    }
}
