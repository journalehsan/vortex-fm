use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use anyhow::Result;

const BOOKMARKS_FILE: &str = "bookmarks.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub name: String,
    pub path: PathBuf,
    pub icon: String,
    pub category: String,
}

impl Bookmark {
    pub fn new(name: String, path: PathBuf, icon: String, category: String) -> Self {
        Self {
            name,
            path,
            icon,
            category,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookmarksManager {
    pub bookmarks: Vec<Bookmark>,
}

impl Default for BookmarksManager {
    fn default() -> Self {
        let mut manager = Self {
            bookmarks: Vec::new(),
        };
        
        // Add default bookmarks
        if let Ok(home) = std::env::var("HOME") {
            let home_path = PathBuf::from(&home);
            
            manager.add_bookmark(Bookmark::new(
                "Home".to_string(),
                home_path.clone(),
                "🏠".to_string(),
                "Quick Access".to_string(),
            ));
            
            manager.add_bookmark(Bookmark::new(
                "Documents".to_string(),
                home_path.join("Documents"),
                "📄".to_string(),
                "Quick Access".to_string(),
            ));
            
            manager.add_bookmark(Bookmark::new(
                "Downloads".to_string(),
                home_path.join("Downloads"),
                "📥".to_string(),
                "Quick Access".to_string(),
            ));
            
            manager.add_bookmark(Bookmark::new(
                "Pictures".to_string(),
                home_path.join("Pictures"),
                "🖼️".to_string(),
                "Quick Access".to_string(),
            ));
            
            manager.add_bookmark(Bookmark::new(
                "Music".to_string(),
                home_path.join("Music"),
                "🎵".to_string(),
                "Quick Access".to_string(),
            ));
            
            manager.add_bookmark(Bookmark::new(
                "Videos".to_string(),
                home_path.join("Videos"),
                "🎬".to_string(),
                "Quick Access".to_string(),
            ));
            
            manager.add_bookmark(Bookmark::new(
                "Desktop".to_string(),
                home_path.join("Desktop"),
                "🖥️".to_string(),
                "Quick Access".to_string(),
            ));
        }
        
        manager
    }
}

impl BookmarksManager {
    pub fn load() -> Self {
        let bookmarks_path = Self::get_bookmarks_path();
        
        if let Ok(bookmarks_data) = fs::read_to_string(&bookmarks_path) {
            if let Ok(bookmarks) = serde_json::from_str(&bookmarks_data) {
                return bookmarks;
            }
        }
        
        // If loading fails, create default bookmarks
        let default_bookmarks = Self::default();
        let _ = default_bookmarks.save();
        default_bookmarks
    }
    
    pub fn save(&self) -> Result<()> {
        let bookmarks_path = Self::get_bookmarks_path();
        
        // Create config directory if it doesn't exist
        if let Some(parent) = bookmarks_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let bookmarks_data = serde_json::to_string_pretty(self)?;
        fs::write(&bookmarks_path, bookmarks_data)?;
        Ok(())
    }
    
    pub fn add_bookmark(&mut self, bookmark: Bookmark) {
        // Check if bookmark already exists
        if !self.bookmarks.iter().any(|b| b.path == bookmark.path) {
            self.bookmarks.push(bookmark);
        }
    }
    
    pub fn remove_bookmark(&mut self, path: &PathBuf) {
        self.bookmarks.retain(|b| b.path != *path);
    }
    
    pub fn get_bookmarks_by_category(&self, category: &str) -> Vec<&Bookmark> {
        self.bookmarks.iter()
            .filter(|b| b.category == category)
            .collect()
    }
    
    pub fn get_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self.bookmarks.iter()
            .map(|b| b.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        categories
    }
    
    fn get_bookmarks_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        PathBuf::from(home).join(".local/config/vortex").join(BOOKMARKS_FILE)
    }
}

// Global access for UI layers to mutate bookmarks
static mut GLOBAL_BOOKMARKS_MANAGER: Option<std::rc::Rc<std::cell::RefCell<BookmarksManager>>> = None;

pub fn set_global_bookmarks_manager(manager: std::rc::Rc<std::cell::RefCell<BookmarksManager>>) {
    unsafe { GLOBAL_BOOKMARKS_MANAGER = Some(manager); }
}

pub fn get_global_bookmarks_manager() -> Option<std::rc::Rc<std::cell::RefCell<BookmarksManager>>> {
    unsafe { GLOBAL_BOOKMARKS_MANAGER.as_ref().map(|m| m.clone()) }
}
