mod folder_organize;
mod helpers;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_root_path() -> String {
    let os = std::env::consts::OS;

    if os == "linux" {
        "/home".to_string()
    } else if os == "macos" {
        "/Users".to_string()
    } else {
        "C:\\".to_string()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_root_path,
            folder_organize::get_list_of_files_in_folder,
            folder_organize::organize_folder,
            folder_organize::search_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
