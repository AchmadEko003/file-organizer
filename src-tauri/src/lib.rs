mod folder_organize;
mod helpers;
mod pdf;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_root_path() -> String {
    let os = std::env::consts::OS;

    os.to_string()
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
            pdf::get_pdf_page_count,
            pdf::do_split,
            pdf::do_delete_pages,
            pdf::do_merge_pdfs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
