use std::path::PathBuf;
use std::fs;
use gtk::prelude::*;
use gtk::{ScrolledWindow, Label, Box as GtkBox, Button};
use crate::core::config::VortexConfig;
use crate::core::navigation_history::NavigationHistory;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ViewMode {
    List,
    Grid,
}

impl ViewMode {
    pub fn as_str(&self) -> &'static str {
        match self { ViewMode::List => "list", ViewMode::Grid => "grid" }
    }
    pub fn from_str(s: &str) -> Self {
        match s { "grid" => ViewMode::Grid, _ => ViewMode::List }
    }
}

#[derive(Clone)]
pub struct FileManagerState {
    pub navigation_history: NavigationHistory,
    pub config: VortexConfig,
    pub current_view_mode: ViewMode,
    pub file_list_widget: Option<ScrolledWindow>,
    pub path_label: Option<Label>,
    pub status_bar: Option<GtkBox>,
    pub back_button: Option<Button>,
    pub forward_button: Option<Button>,
    pub up_button: Option<Button>,
}

impl FileManagerState {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        let home_path = PathBuf::from(&home);
        let cfg = VortexConfig::load();
        Self {
            navigation_history: NavigationHistory::new(home_path),
            config: cfg.clone(),
            current_view_mode: ViewMode::from_str(&cfg.default_view_mode),
            file_list_widget: None,
            path_label: None,
            status_bar: None,
            back_button: None,
            forward_button: None,
            up_button: None,
        }
    }
    
    pub fn navigate_to(&mut self, path: PathBuf) {
        if path.exists() && path.is_dir() {
            self.navigation_history.navigate_to(path);
            self.update_navigation_buttons();
            self.refresh_ui();
        }
    }
    
    pub fn go_back(&mut self) {
        if let Some(_path) = self.navigation_history.go_back() {
            self.update_navigation_buttons();
            self.refresh_ui();
        }
    }
    
    pub fn go_forward(&mut self) {
        if let Some(_path) = self.navigation_history.go_forward() {
            self.update_navigation_buttons();
            self.refresh_ui();
        }
    }
    
    pub fn go_up(&mut self) {
        if let Some(_path) = self.navigation_history.go_up() {
            self.update_navigation_buttons();
            self.refresh_ui();
        }
    }
    
    pub fn can_go_back(&self) -> bool {
        self.navigation_history.can_go_back()
    }
    
    pub fn can_go_forward(&self) -> bool {
        self.navigation_history.can_go_forward()
    }
    
    pub fn can_go_up(&self) -> bool {
        self.navigation_history.can_go_up()
    }
    
    pub fn current_path(&self) -> &PathBuf {
        self.navigation_history.current()
    }
    
    fn update_navigation_buttons(&self) {
        // Update back button
        if let Some(back_btn) = &self.back_button {
            back_btn.set_sensitive(self.can_go_back());
        }
        
        // Update forward button
        if let Some(forward_btn) = &self.forward_button {
            forward_btn.set_sensitive(self.can_go_forward());
        }
        
        // Update up button
        if let Some(up_btn) = &self.up_button {
            up_btn.set_sensitive(self.can_go_up());
        }
    }
    
    pub fn refresh_ui(&self) {
        // Update path label
        if let Some(path_label) = &self.path_label {
            let current_path_str = self.current_path().to_string_lossy().to_string();
            path_label.set_text(&current_path_str);
        }
        
        // Refresh active adapter-based file view (if present)
        crate::views::content_area::refresh_active_view();
        
        // Update status bar
        if let Some(status_bar) = &self.status_bar {
            self.update_status_bar(status_bar);
        }
    }

    pub fn set_view_mode(&mut self, mode: ViewMode) {
        self.current_view_mode = mode;
        let mut cfg = self.config.clone();
        cfg.default_view_mode = mode.as_str().to_string();
        let _ = cfg.save();
        self.config = cfg;
    }
    
    pub fn update_file_list(&self, scrolled: &ScrolledWindow) {
        // Clear existing content
        if let Some(_child) = scrolled.child() {
            scrolled.set_child(None::<&gtk::Widget>);
        }
        
        // Create new grid
        let grid = gtk::Grid::new();
        grid.set_row_spacing(12);
        grid.set_column_spacing(12);
        grid.set_margin_start(12);
        grid.set_margin_end(12);
        grid.set_margin_top(12);
        grid.set_margin_bottom(12);
        
        // Read files from current directory
        let mut files = Vec::new();
        
        if let Ok(entries) = fs::read_dir(self.current_path()) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();
                
                // Skip hidden files if not configured to show them
                if !self.config.show_hidden_files && name.starts_with('.') {
                    continue;
                }
                
                let (icon, file_type) = if path.is_dir() {
                    ("📁", "Folder")
                } else {
                    match path.extension().and_then(|s| s.to_str()) {
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
                
                files.push((icon, name, file_type, path));
            }
        }
        
        // Sort files: directories first, then files, both alphabetically
        files.sort_by(|a, b| {
            let a_is_dir = a.3.is_dir();
            let b_is_dir = b.3.is_dir();
            
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.1.cmp(&b.1),
            }
        });
        
        // Add files to grid
        let mut row = 0;
        let mut col = 0;
        const ITEMS_PER_ROW: i32 = 6;
        
        for (icon, name, file_type, path) in files {
            let file_box = crate::widgets::file_item::create_file_item(icon, &name, file_type, path, &self.config);
            grid.attach(&file_box, col, row, 1, 1);
            
            col += 1;
            if col >= ITEMS_PER_ROW {
                col = 0;
                row += 1;
            }
        }
        
        scrolled.set_child(Some(&grid));
    }
    
    pub fn update_status_bar(&self, status_bar: &GtkBox) {
        // Count actual items in directory
        let item_count = if let Ok(entries) = fs::read_dir(self.current_path()) {
            if self.config.show_hidden_files {
                entries.count()
            } else {
                entries.filter(|entry| {
                    entry.as_ref()
                        .map(|e| e.file_name().to_str().map(|name| !name.starts_with('.')).unwrap_or(false))
                        .unwrap_or(false)
                })
                .count()
            }
        } else {
            0
        };
        
        let items_text = if item_count == 1 {
            "1 item".to_string()
        } else {
            format!("{} items", item_count)
        };
        
        // Update the items label in status bar
        if let Some(child) = status_bar.last_child() {
            if let Some(label) = child.downcast_ref::<Label>() {
                label.set_text(&items_text);
            }
        }
    }
}
