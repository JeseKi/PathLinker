#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod backend_opearation;

use std::sync::{Arc, Mutex};
use backend_opearation as bo;
use libs::{crud, utils};
use rusqlite::Connection;

fn main() {

    let connection = Arc::new(Mutex::new(crud::connect_db()));

    let app_state = bo::AppState {
        conn: connection,
    };

    // 处理自定义协议
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let url = &args[1];
        // utils::log_to_file(&format!("Received URL: {}", url));
        let conn = crud::connect_db();
        // println!("url:{url}");
        // utils::log_to_file(&format!("Attempting to open URL: {}", url));
        let flag = open_file(&conn, url);
        match flag {
            Ok(..) => {
                utils::log_to_file("URL opened successfully.", None);
                std::process::exit(0);
            },
            Err(..) => {
                utils::log_to_file("Failed to open URL.", None);
                std::process::exit(1);
            },
        }
    }

    // 程序运行
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![bo::handle_selected_path, bo::get_mappings, bo::delete_mapping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 根据URL打开对应映射的文件
fn open_file(conn: &Connection, url: &String) -> Result<(), String> {
    let path = crud::get_filepath_by_url(conn, url);
    // utils::log_to_file(&format!("Attempting to open file: {}", &path));
    let flag = open::that(&path);

    match flag {
        Ok(..) => Ok(()),
        Err(e) => {
            println!("Failed to open the file: {}", e);
            Err(e.to_string())
        }
    }
}