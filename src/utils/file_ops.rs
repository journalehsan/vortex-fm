use std::process::Command;
use std::path::PathBuf;
use anyhow::Result;

pub fn open_with_system(path: &PathBuf) -> Result<()> {
    let path_str = path.to_string_lossy().to_string();
    
    // Use xdg-open to open files with the system default application
    let output = Command::new("xdg-open")
        .arg(&path_str)
        .output()?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to open file: {}", error_msg));
    }
    
    Ok(())
}

pub fn get_file_info(path: &PathBuf) -> Result<FileInfo> {
    let metadata = std::fs::metadata(path)?;
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();
    
    let file_type = if metadata.is_dir() {
        "Directory"
    } else {
        "File"
    };
    
    let size = metadata.len();
    let modified = metadata.modified()?;
    
    Ok(FileInfo {
        name: file_name,
        file_type: file_type.to_string(),
        size,
        modified,
        path: path.clone(),
    })
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub file_type: String,
    pub size: u64,
    pub modified: std::time::SystemTime,
    pub path: PathBuf,
}
