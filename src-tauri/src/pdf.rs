use lopdf::{Dictionary, Document, Object};
use std::collections::BTreeMap;

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
pub fn do_split(
    file_path: &str,
    output_path: &str,
    split_options: Vec<&str>,
) -> Result<String, String> {
    if !std::path::Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    println!(
        "Splitting PDF: {} with options: {:?}",
        file_path, split_options
    );

    for item in split_options {
        println!("Processing split option: {}", item);
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

            match extract_multiple_pages(file_path, output_path, (start..=end).collect()) {
                Ok(()) => {}
                Err(e) => {
                    return Err(format!("Failed to extract pages: {}", e));
                }
            }
        } else {
            let page_num: u32 = item.parse().map_err(|e| format!("{}", e))?;
            if page_num == 0 {
                return Err(format!("Invalid page number: {}", page_num));
            }

            match extract_multiple_pages(file_path, output_path, vec![page_num]) {
                Ok(()) => {}
                Err(e) => {
                    return Err(format!("Failed to extract page {}: {}", page_num, e));
                }
            }
        }
    }

    Ok("PDF split completed successfully".into())
}

#[tauri::command]
pub fn do_delete_pages(
    file_path: &str,
    output_path: &str,
    selected_pages: Vec<u32>,
) -> Result<String, String> {
    if !std::path::Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let doc = Document::load(file_path)
        .map_err(|e| format!("Failed to load PDF '{}': {}", file_path, e))?;

    let mut new_pages: Vec<u32> = Vec::new();

    for page_num in 1..=(doc.get_pages().len() as u32) {
        if !selected_pages.contains(&page_num) {
            new_pages.push(page_num);
        }
    }

    match extract_multiple_pages(file_path, output_path, new_pages.clone()) {
        Ok(()) => {}
        Err(e) => {
            return Err(format!("Failed to extract pages {:?}: {}", new_pages, e));
        }
    }

    Ok("PDF delete completed successfully".into())
}

fn extract_multiple_pages(
    file_path: &str,
    output_path: &str,
    target_pages: Vec<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let doc = Document::load(file_path)
        .map_err(|e| format!("Failed to load PDF '{}': {}", file_path, e))?;
    let pages = doc.get_pages();

    if target_pages.is_empty() {
        return Err("Page number is out of range".into());
    }

    let mut new_doc = Document::with_version("1.5");

    let mut objects_to_copy = BTreeMap::new();
    let mut new_page_ids = Vec::new();

    for &page_num in &target_pages {
        if let Some(&page_id) = pages.get(&page_num) {
            new_page_ids.push(page_id);
            collect_page_content(&doc, page_id, &mut objects_to_copy)?;
        }
    }

    let mut id_map = BTreeMap::new();
    for (old_id, obj) in objects_to_copy {
        let new_id = new_doc.add_object(obj);
        id_map.insert(old_id, new_id);
    }

    // Update references in the new document
    update_references(&mut new_doc, &id_map)?;

    // Set up the document structure for multiple pages
    setup_multi_page_document_structure(&mut new_doc, &id_map, &new_page_ids)?;

    let name: String;

    if target_pages.len() > 1 {
        name = format!(
            "pages_{}-{}",
            target_pages.first().unwrap(),
            target_pages.last().unwrap()
        );
    } else {
        name = format!("page_{}", target_pages[0]);
    }

    std::fs::create_dir_all(output_path)?;

    new_doc.save(format!("{}/{}.pdf", output_path, name))?;

    Ok(())
}

fn collect_page_content(
    doc: &Document,
    page_id: (u32, u16),
    objects: &mut BTreeMap<(u32, u16), Object>,
) -> Result<(), Box<dyn std::error::Error>> {
    if objects.contains_key(&page_id) {
        return Ok(());
    }

    let obj = doc.get_object(page_id)?.clone();
    objects.insert(page_id, obj.clone());

    // Recursively collect referenced objects
    collect_referenced_content(doc, &obj, objects)?;

    Ok(())
}

fn collect_referenced_content(
    doc: &Document,
    obj: &Object,
    objects: &mut BTreeMap<(u32, u16), Object>,
) -> Result<(), Box<dyn std::error::Error>> {
    match obj {
        Object::Reference(ref_id) => {
            if !objects.contains_key(ref_id) {
                if let Ok(referenced_obj) = doc.get_object(*ref_id) {
                    objects.insert(*ref_id, referenced_obj.clone());
                    collect_referenced_content(doc, referenced_obj, objects)?;
                }
            }
        }
        Object::Dictionary(dict) => {
            for (_, value) in dict.iter() {
                collect_referenced_content(doc, value, objects)?;
            }
        }
        Object::Array(arr) => {
            for item in arr {
                collect_referenced_content(doc, item, objects)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn update_references(
    doc: &mut Document,
    id_map: &BTreeMap<(u32, u16), (u32, u16)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let object_ids: Vec<(u32, u16)> = doc.objects.keys().cloned().collect();

    for obj_id in object_ids {
        if let Ok(obj) = doc.get_object_mut(obj_id) {
            update_object_references(obj, id_map);
        }
    }

    Ok(())
}

fn update_object_references(obj: &mut Object, id_map: &BTreeMap<(u32, u16), (u32, u16)>) {
    match obj {
        Object::Reference(ref_id) => {
            if let Some(&new_id) = id_map.get(ref_id) {
                *ref_id = new_id;
            }
        }
        Object::Dictionary(dict) => {
            for (_, value) in dict.iter_mut() {
                update_object_references(value, id_map);
            }
        }
        Object::Array(arr) => {
            for item in arr {
                update_object_references(item, id_map);
            }
        }
        _ => {}
    }
}

// Function to set up document structure for multiple pages
fn setup_multi_page_document_structure(
    doc: &mut Document,
    id_map: &BTreeMap<(u32, u16), (u32, u16)>,
    original_page_ids: &[(u32, u16)],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut new_page_ids = Vec::new();
    for &original_id in original_page_ids {
        if let Some(&new_id) = id_map.get(&original_id) {
            new_page_ids.push(new_id);
        }
    }

    // Create pages dictionary
    let mut pages_dict = Dictionary::new();
    pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
    pages_dict.set("Count", Object::Integer(new_page_ids.len() as i64));

    let kids_array: Vec<Object> = new_page_ids
        .iter()
        .map(|&id| Object::Reference(id))
        .collect();
    pages_dict.set("Kids", Object::Array(kids_array));

    let pages_obj_id = doc.add_object(Object::Dictionary(pages_dict));

    // Update each page to reference the new pages object
    for &page_id in &new_page_ids {
        if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
            page_dict.set("Parent", Object::Reference(pages_obj_id));
        }
    }

    // Create catalog
    let mut catalog = Dictionary::new();
    catalog.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog.set("Pages", Object::Reference(pages_obj_id));

    let catalog_id = doc.add_object(Object::Dictionary(catalog));
    doc.trailer.set("Root", Object::Reference(catalog_id));

    Ok(())
}

#[tauri::command]
pub fn do_merge_pdfs(
    file_paths: Vec<&str>,
    output_path: &str,
) -> Result<String, String> {
    if file_paths.len() < 2 {
        return Err("At least 2 PDF files are required for merging".to_string());
    }

    // Check if all files exist
    for file_path in &file_paths {
        if !std::path::Path::new(file_path).exists() {
            return Err(format!("File not found: {}", file_path));
        }
    }

    merge_pdfs_using_extraction(&file_paths, output_path)
        .map_err(|e| format!("Failed to merge PDFs: {}", e))?;

    Ok(format!("Successfully merged {} PDFs into {}", file_paths.len(), output_path))
}

fn merge_pdfs_using_extraction(
    file_paths: &[&str],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut merged_doc = Document::with_version("1.5");
    let mut all_objects = BTreeMap::new();
    let mut all_page_ids = Vec::new();

    for file_path in file_paths {
        let doc = Document::load(file_path)?;
        let pages = doc.get_pages();

        // Get all page numbers in order
        let mut page_numbers: Vec<_> = pages.keys().cloned().collect();
        page_numbers.sort();

        // Collect content from all pages in this PDF
        for page_num in page_numbers {
            if let Some(&page_id) = pages.get(&page_num) {
                collect_page_content(&doc, page_id, &mut all_objects)?;
                all_page_ids.push(page_id);
            }
        }
    }

    // Copy all collected objects to the new document
    let mut id_map = BTreeMap::new();
    for (old_id, obj) in all_objects {
        let new_id = merged_doc.add_object(obj);
        id_map.insert(old_id, new_id);
    }

    update_references(&mut merged_doc, &id_map)?;
    setup_multi_page_document_structure(&mut merged_doc, &id_map, &all_page_ids)?;

    merged_doc.save(output_path)?;

    Ok(())
}
