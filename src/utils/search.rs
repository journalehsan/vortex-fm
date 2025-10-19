use std::path::PathBuf;
use std::fs;
use crate::core::config::VortexConfig;

pub fn filter_files_in_directory(path: &PathBuf, filter: &str, config: &VortexConfig) -> Vec<FileEntry> {
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            let name = file_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
            
            // Skip hidden files if not configured to show them
            if !config.show_hidden_files && name.starts_with('.') {
                continue;
            }
            
            // Apply filter (case-insensitive)
            if filter.is_empty() || name.to_lowercase().contains(&filter.to_lowercase()) {
                let (icon, file_type) = if file_path.is_dir() {
                    ("📁", "Folder")
                } else {
                    match file_path.extension().and_then(|s| s.to_str()) {
                        Some("txt") | Some("md") | Some("log") => ("📄", "Text File"),
                        Some("jpg") | Some("jpeg") | Some("png") | Some("gif") | Some("bmp") => ("🖼️", "Image File"),
                        Some("mp3") | Some("wav") | Some("flac") | Some("ogg") => ("🎵", "Audio File"),
                        Some("mp4") | Some("avi") | Some("mkv") | Some("mov") => ("🎬", "Video File"),
                        Some("zip") | Some("tar") | Some("gz") | Some("rar") => ("📦", "Archive File"),
                        Some("sh") | Some("py") | Some("js") | Some("rs") | Some("c") | Some("cpp") => ("💻", "Script File"),
                        Some("pdf") => ("📕", "PDF File"),
                        Some("doc") | Some("docx") => ("📘", "Document File"),
                        Some("xls") | Some("xlsx") => ("📊", "Spreadsheet File"),
                        Some("ppt") | Some("pptx") => ("📽️", "Presentation File"),
                        _ => ("📄", "File"),
                    }
                };
                
                files.push(FileEntry {
                    icon: icon.to_string(),
                    name,
                    file_type: file_type.to_string(),
                    path: file_path,
                });
            }
        }
    }
    
    // Sort files: directories first, then files, both alphabetically
    files.sort_by(|a, b| {
        let a_is_dir = a.path.is_dir();
        let b_is_dir = b.path.is_dir();
        
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });
    
    files
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub icon: String,
    pub name: String,
    pub file_type: String,
    pub path: PathBuf,
}
