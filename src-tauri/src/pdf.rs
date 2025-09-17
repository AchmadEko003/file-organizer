#[tauri::command]
pub fn get_pdf_page_count(file_path: &str) -> Result<i32, String> {
    if !std::path::Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let doc = lopdf::Document::load(file_path)
        .map_err(|e| format!("Failed to load PDF '{}': {}", file_path, e))?;
    let page_count = doc.get_pages().len() as i32;
    
    Ok(page_count)
}