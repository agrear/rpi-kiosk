use std::env;

#[tauri::command]
fn get_url() -> String {
    let args: Vec<String> = env::args().collect();

    String::from(&args[1])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
