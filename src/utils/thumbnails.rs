use std::path::PathBuf;
use std::fs;
use image::ImageFormat;
use anyhow::Result;

const THUMBNAIL_SIZE: u32 = 128;
const THUMBNAIL_CACHE_DIR: &str = ".cache/vortex/thumbnails";

pub struct ThumbnailManager {
    cache_dir: PathBuf,
}

impl ThumbnailManager {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        let cache_dir = PathBuf::from(home).join(THUMBNAIL_CACHE_DIR);
        
        // Create cache directory if it doesn't exist
        if !cache_dir.exists() {
            let _ = fs::create_dir_all(&cache_dir);
        }
        
        Self { cache_dir }
    }
    
    pub fn get_thumbnail_path(&self, file_path: &PathBuf) -> PathBuf {
        // Create a hash-based filename for the thumbnail
        let hash = format!("{:x}", md5::compute(file_path.to_string_lossy().as_bytes()));
        self.cache_dir.join(format!("{}.png", hash))
    }
    
    pub fn has_thumbnail(&self, file_path: &PathBuf) -> bool {
        self.get_thumbnail_path(file_path).exists()
    }
    
    pub fn generate_thumbnail(&self, file_path: &PathBuf) -> Result<PathBuf> {
        let thumbnail_path = self.get_thumbnail_path(file_path);
        
        // Check if thumbnail already exists
        if thumbnail_path.exists() {
            return Ok(thumbnail_path);
        }
        
        // Load and resize the image
        let img = image::open(file_path)?;
        let thumbnail = img.thumbnail(THUMBNAIL_SIZE, THUMBNAIL_SIZE);
        
        // Save as PNG
        thumbnail.save_with_format(&thumbnail_path, ImageFormat::Png)?;
        
        Ok(thumbnail_path)
    }
    
    pub fn is_image_file(&self, file_path: &PathBuf) -> bool {
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                match ext_str.to_lowercase().as_str() {
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "ico" => true,
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    
    pub fn get_thumbnail_or_placeholder(&self, file_path: &PathBuf) -> Option<PathBuf> {
        if self.is_image_file(file_path) {
            match self.generate_thumbnail(file_path) {
                Ok(path) => Some(path),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

// Global thumbnail manager
static mut GLOBAL_THUMBNAIL_MANAGER: Option<ThumbnailManager> = None;

pub fn get_global_thumbnail_manager() -> &'static ThumbnailManager {
    unsafe {
        if GLOBAL_THUMBNAIL_MANAGER.is_none() {
            GLOBAL_THUMBNAIL_MANAGER = Some(ThumbnailManager::new());
        }
        GLOBAL_THUMBNAIL_MANAGER.as_ref().unwrap()
    }
}
