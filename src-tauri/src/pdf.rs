use lopdf::{Document, Object, dictionary};

#[tauri::command]
pub fn get_pdf_page_count(file_path: &str) -> Result<i32, String> {
    if !std::path::Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let doc: lopdf::Document = lopdf::Document::load(file_path)
        .map_err(|e| format!("Failed to load PDF '{}': {}", file_path, e))?;
    let page_count = doc.get_pages().len() as i32;

    Ok(page_count)
}

#[tauri::command]
pub fn do_split(file_path: &str, split_options: Vec<&str>) -> Result<String, String> {
    if !std::path::Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let doc: Document = Document::load(file_path)
        .map_err(|e| format!("Failed to load PDF '{}': {}", file_path, e))?;

    let pages = doc.get_pages();

    for item in split_options {
        if item.contains('-') {
            let parts: Vec<&str> = item.split('-').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid page range format: {}", item));
            }

            let start: u32 = parts[0].parse().map_err(|e| format!("{}", e))?;
            let end: u32 = parts[1].parse().map_err(|e| format!("{}", e))?;
            
            if start == 0 || end == 0 || start > end {
                return Err(format!("Invalid page range: {}-{}", start, end));
            }

            let mut new_doc = Document::with_version("1.5");
            
            let mut kids = vec![];

            for page_num in start..=end {
                let page_id = *pages.get(&(page_num - 1)).ok_or(format!("Page {} not found", page_num))?;
                
                let page_obj = doc.get_object(page_id).unwrap().clone();
                
                let new_page_id = new_doc.add_object(page_obj.clone());
                
                kids.push(Object::Reference(new_page_id));
            }

            let pages_id = new_doc.new_object_id();
            
            for kid in &kids {
                if let Object::Reference(id) = kid {
                    if let Some(page_dict) = new_doc.objects.get_mut(id) {
                        if let Ok(dict) = page_dict.as_dict_mut() {
                            dict.set("Parent", Object::Reference(pages_id));
                        }
                    }
                }
            }

            new_doc.objects.insert(
                pages_id,
                Object::Dictionary(dictionary! {
                    "Type" => "Pages",
                    "Kids" => kids,
                    "Count" => (end - start + 1) as i64,
                }),
            );

            let catalog_id = new_doc.new_object_id();
            new_doc.objects.insert(
                catalog_id,
                Object::Dictionary(dictionary! {
                    "Type" => "Catalog",
                    "Pages" => Object::Reference(pages_id),
                }),
            );

            new_doc.trailer.set("Root", catalog_id);
            
            new_doc.prune_objects();

            let output_path = format!("output_{}_to_{}.pdf", start, end);
            
            new_doc.save(&output_path)
                .map_err(|e| format!("Failed to save {}: {}", output_path, e))?;
        }
    }

    Ok("PDF split completed successfully".into())
}
