#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use open;
use rusqlite::Connection;
use tauri::State;
use std::sync::{Arc, Mutex};

use libs::{utils ,crud};

struct AppState {
    conn: Arc<Mutex<Connection>>,
}

#[tauri::command]
fn handle_selected_path(state: State<AppState>, selected: Vec<String>) {
    println!("Received selected path: {:?}", selected);

    for path in selected {
        if let Some(file_name) = std::path::Path::new(&path).file_name().and_then(|f| f.to_str()) {
            let random_url = utils::generate_random_url();
            println!("Generated URL: {} for file: {}", random_url, file_name);

            // 获取数据库连接
            let conn = state.conn.lock().unwrap();

            // 创建映射
            if let Err(e) = crud::create_mapping(&*conn, file_name, &path, &random_url) {
                eprintln!("Failed to create mapping for file: {}: {}", file_name, e);
            }
        } else {
            eprintln!("Failed to extract file name from path: {}", path);
        }

        // 尝试打开文件（如果需要）
        if let Err(e) = open::that(&path) {
            eprintln!("Failed to open the file: {}", e);
        }
    }
}


fn main() {
    let connection = Arc::new(Mutex::new(crud::connect_db()));

    let app_state = AppState {
        conn: connection,
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![handle_selected_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
