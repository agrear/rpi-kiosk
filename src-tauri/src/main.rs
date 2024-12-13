// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

#[tauri::command]
fn get_url() -> String {
    let args: Vec<String> = env::args().collect();

    String::from(&args[1])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
