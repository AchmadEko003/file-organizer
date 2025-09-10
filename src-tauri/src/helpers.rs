#[tauri::command]
pub fn is_image_file(ext: &str) -> bool {
    match ext {
        "png" | "jpg" | "jpeg" | "jfif" | "gif" | "bmp" | "svg" | "svgz" | "webp" | "tiff"
        | "tif" | "ico" | "jp2" | "j2k" | "jpc" | "jpf" | "jpx" | "jpm" | "heic" | "heif"
        | "avif" | "exr" | "hdr" | "cr2" | "cr3" | "crw" | "nef" | "arw" | "dng" | "raf"
        | "rw2" | "sr2" | "srw" | "orf" | "pef" | "mrw" | "raw" | "psd" | "psb" | "xcf" | "ai"
        | "eps" | "cdr" | "pcx" | "tga" | "sgi" | "ras" | "pbm" | "pgm" | "ppm" | "wmf" | "emf" => {
            true
        }
        _ => false,
    }
}

#[tauri::command]
pub fn is_document_file(ext: &str) -> bool {
    match ext {
        "pdf" | "doc" | "docx" | "docm" | "dot" | "dotx" | "xls" | "xlsx" | "xlsm" | "xlsb"
        | "ppt" | "pptx" | "pptm" | "odt" | "ott" | "ods" | "ots" | "odp" | "otp" | "txt"
        | "md" | "markdown" | "mdown" | "rst" | "adoc" | "asciidoc" | "html" | "htm" | "xhtml"
        | "xml" | "json" | "rtf" | "chm" | "tex" | "epub" | "mobi" | "azw3" | "lit" | "pub"
        | "pages" | "wps" | "msg" | "eml" | "xps" | "ps" | "csv" | "xsl" => true,
        _ => false,
    }
}

#[tauri::command]
pub fn is_video_file(ext: &str) -> bool {
    match ext {
        "mp4" | "mkv" | "webm" | "avi" | "mov" | "wmv" | "flv" | "m4v" | "mpg" | "mpeg"
        | "3gp" | "3g2" | "f4v" | "f4p" | "f4a" | "f4b" => true,
        _ => false,
    }
}