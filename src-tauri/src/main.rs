#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use open;

use libs::{utils , db};
use db::crud as crud;

#[tauri::command]
fn handle_selected_path(selected: Vec<String>) {
    println!("Received selected path: {:?}", selected);
    for path in selected {
        if let Err(e) = open::that(path) {
            eprintln!("Failed to open the file: {}", e);
        }
    }
    let random_url = utils::generate_random_url();
    println!("Generated URL: {}", random_url);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_selected_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}