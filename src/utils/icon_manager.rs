use std::path::PathBuf;
use gtk::prelude::*;
use gtk::{Image, gio};
use std::collections::HashMap;
use std::sync::{Once, Mutex};
use std::cell::RefCell;

/// IconManager facade for handling system icons and MIME types
pub struct IconManager {
    icon_cache: RefCell<HashMap<String, gtk::IconPaintable>>,
    icon_theme: gtk::IconTheme,
    mime_type_cache: RefCell<HashMap<String, String>>,
}

impl IconManager {
    pub fn new() -> Self {
        let icon_theme = gtk::IconTheme::for_display(&gtk::gdk::Display::default().unwrap());
        Self {
            icon_cache: RefCell::new(HashMap::new()),
            icon_theme,
            mime_type_cache: RefCell::new(HashMap::new()),
        }
    }

    /// Get system icon for a file path
    pub fn get_file_icon(&self, path: &PathBuf, size: i32) -> Option<gtk::IconPaintable> {
        let mime_type = self.get_mime_type(path);
        let icon_name = self.get_icon_name_for_mime_type(&mime_type, path.is_dir());
        
        {
            let cache = self.icon_cache.borrow();
            if let Some(cached) = cache.get(&format!("{}_{}", icon_name, size)) {
                return Some(cached.clone());
            }
        }

        let icon_paintable = self.icon_theme.lookup_icon(
            &icon_name,
            &[],
            size,
            1,
            gtk::TextDirection::Ltr,
            gtk::IconLookupFlags::empty(),
        );

        // IconPaintable is always returned, just use it
        {
            let mut cache = self.icon_cache.borrow_mut();
            cache.insert(format!("{}_{}", icon_name, size), icon_paintable.clone());
        }
        Some(icon_paintable)
    }

    /// Get MIME type for a file
    fn get_mime_type(&self, path: &PathBuf) -> String {
        {
            let cache = self.mime_type_cache.borrow();
            if let Some(cached) = cache.get(path.to_str().unwrap_or("")) {
                return cached.clone();
            }
        }

        let mime_type = if path.is_dir() {
            "inode/directory".to_string()
        } else {
            gio::content_type_guess(Some(path.to_str().unwrap_or("")), &[])
                .0
                .to_string()
        };

        {
            let mut cache = self.mime_type_cache.borrow_mut();
            cache.insert(path.to_str().unwrap_or("").to_string(), mime_type.clone());
        }
        mime_type
    }

    /// Get icon name for MIME type
    fn get_icon_name_for_mime_type(&self, mime_type: &str, is_dir: bool) -> String {
        if is_dir {
            return "folder".to_string();
        }

        match mime_type {
            // Text files
            t if t.starts_with("text/") => "text-x-generic".to_string(),
            t if t == "application/x-shellscript" => "application-x-executable".to_string(),
            t if t == "application/x-executable" => "application-x-executable".to_string(),
            
            // Images
            t if t.starts_with("image/") => {
                match t {
                    "image/png" => "image-png".to_string(),
                    "image/jpeg" => "image-jpeg".to_string(),
                    "image/gif" => "image-gif".to_string(),
                    "image/svg+xml" => "image-svg+xml".to_string(),
                    _ => "image-x-generic".to_string(),
                }
            },
            
            // Audio
            t if t.starts_with("audio/") => "audio-x-generic".to_string(),
            
            // Video
            t if t.starts_with("video/") => "video-x-generic".to_string(),
            
            // Archives
            t if t == "application/zip" => "application-zip".to_string(),
            t if t == "application/x-tar" => "application-x-tar".to_string(),
            t if t == "application/x-7z-compressed" => "application-x-7z-compressed".to_string(),
            t if t == "application/x-rar" => "application-x-rar".to_string(),
            
            // Documents
            t if t == "application/pdf" => "application-pdf".to_string(),
            t if t == "application/vnd.oasis.opendocument.text" => "application-vnd.oasis.opendocument.text".to_string(),
            t if t == "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => "application-vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
            
            // Code files
            t if t == "text/x-rust" => "text-x-rust".to_string(),
            t if t == "text/x-python" => "text-x-python".to_string(),
            t if t == "text/x-javascript" => "text-x-javascript".to_string(),
            t if t == "text/x-c" => "text-x-c".to_string(),
            t if t == "text/x-c++" => "text-x-c++".to_string(),
            
            _ => "text-x-generic".to_string(),
        }
    }

    /// Get fallback icon when system icon is not available
    fn get_fallback_icon(&self, size: i32) -> Option<gtk::IconPaintable> {
        let icon = self.icon_theme.lookup_icon(
            "text-x-generic",
            &[],
            size,
            1,
            gtk::TextDirection::Ltr,
            gtk::IconLookupFlags::empty(),
        );
        Some(icon)
    }

    /// Create an Image widget with the appropriate icon
    pub fn create_icon_widget(&self, path: &PathBuf, size: i32) -> gtk::Widget {
        let image = Image::new();
        
        if let Some(icon_paintable) = self.get_file_icon(path, size) {
            image.set_from_paintable(Some(&icon_paintable));
        } else {
            // Ultimate fallback to emoji - create a label instead
            let fallback_icon = if path.is_dir() {
                "📁"
            } else {
                "📄"
            };
            let label = gtk::Label::new(Some(fallback_icon));
            label.set_css_classes(&["file-icon"]);
            label.set_halign(gtk::Align::Center);
            label.set_valign(gtk::Align::Center);
            // Return a container with the label instead of image
            let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
            container.append(&label);
            return container.upcast();
        }
        
        image.set_pixel_size(size);
        image.set_css_classes(&["file-icon"]);
        image.upcast()
    }
}

// Global instance using Once and Mutex for thread safety
static INIT: Once = Once::new();
static mut ICON_MANAGER: Option<Mutex<IconManager>> = None;

pub fn get_global_icon_manager() -> &'static Mutex<IconManager> {
    unsafe {
        INIT.call_once(|| {
            ICON_MANAGER = Some(Mutex::new(IconManager::new()));
        });
        ICON_MANAGER.as_ref().unwrap()
    }
}
