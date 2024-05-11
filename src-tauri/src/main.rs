#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod backend_opearation; 
mod hotkey_watcher;

use std::sync::{Arc, Mutex};
use tauri::Manager;

use backend_opearation::{
    self as bo,
    mapping_api as bo_mappings
};
use libs::{
    crud::connect_db,
    utils
};
use hotkey_watcher::start_clipboard_key_listener;

fn main() {

    // 处理自定义协议
    let args: Vec<String> = std::env::args().collect();
    bo::process_software_params::handle_custom_protocol(args);

    // 程序运行
    let _app = tauri::Builder::default()
        .setup(|app|{
            let window = app.get_window("main").expect("Failed to get main window");
            let connection = Arc::new(Mutex::new(connect_db()));

            // 创建 AppState 实例并将其存储在 Tauri 的状态管理中
            let app_state = utils::AppState {
                conn: connection,
                window: window.clone(), // 这里假设你需要克隆窗口对象，如果是单一窗口管理则直接传递 window
            };

            app.manage(app_state); // 将 AppState 存储在 Tauri 状态中

            // 开始监听剪贴板
            start_clipboard_key_listener();

            Ok(())
        }) 
        .invoke_handler(tauri::generate_handler![
            bo_mappings::handle_selected_path, 
            bo_mappings::get_mappings, 
            bo_mappings::delete_mapping,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}