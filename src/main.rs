use gtk::prelude::*;
use gtk::{gio, glib, Application, ApplicationWindow, Box, Orientation, Paned, CssProvider};
use std::path::PathBuf;
use std::fs;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;

const APP_ID: &str = "com.vortex.FileManager";
const CONFIG_DIR: &str = ".local/config/vortex";
const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VortexConfig {
    single_click_to_open: bool,
    show_hidden_files: bool,
    default_view_mode: String, // "grid" or "list"
    window_width: i32,
    window_height: i32,
    sidebar_width: i32,
}

impl Default for VortexConfig {
    fn default() -> Self {
        Self {
            single_click_to_open: true,
            show_hidden_files: false,
            default_view_mode: "grid".to_string(),
            window_width: 1200,
            window_height: 800,
            sidebar_width: 250,
        }
    }
}

impl VortexConfig {
    fn load() -> Self {
        let config_path = Self::get_config_path();
        
        if let Ok(config_data) = fs::read_to_string(&config_path) {
            if let Ok(config) = serde_json::from_str(&config_data) {
                return config;
            }
        }
        
        // If loading fails, create default config
        let default_config = Self::default();
        let _ = default_config.save();
        default_config
    }
    
    fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path();
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let config_data = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, config_data)?;
        Ok(())
    }
    
    fn get_config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        PathBuf::from(home).join(CONFIG_DIR).join(CONFIG_FILE)
    }
}

fn main() -> glib::ExitCode {
    // Create application
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_startup(|_| {
        // load_css(); // Disabled for now due to GTK issues
        println!("🚀 Vortex FM starting up...");
    });

    app.connect_activate(build_ui);
    app.connect_open(|app, files, _hint| {
        // Handle opening files/folders
        if let Some(file) = files.first() {
            if let Some(path) = file.path() {
                println!("📁 Opening: {}", path.display());
            }
        }
        build_ui(app);
    });

    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    let css = include_str!("../resources/style.css");
    
    provider.load_from_data(css);
    
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

#[derive(Clone)]
struct FileManagerState {
    current_path: PathBuf,
    history: Vec<PathBuf>,
    history_index: usize,
    config: VortexConfig,
    file_list_widget: Option<gtk::ScrolledWindow>,
    path_label: Option<gtk::Label>,
    status_bar: Option<gtk::Box>,
}

// Global state for navigation
static mut GLOBAL_STATE: Option<Rc<RefCell<FileManagerState>>> = None;

fn set_global_state(state: Rc<RefCell<FileManagerState>>) {
    unsafe {
        GLOBAL_STATE = Some(state);
    }
}

fn get_global_state() -> Option<Rc<RefCell<FileManagerState>>> {
    unsafe {
        GLOBAL_STATE.clone()
    }
}

fn navigate_to_directory(path: PathBuf) {
    if let Some(state_rc) = get_global_state() {
        let mut state = state_rc.borrow_mut();
        state.navigate_to(path);
        state.refresh_ui();
    }
}

impl FileManagerState {
    fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        Self {
            current_path: PathBuf::from(&home),
            history: vec![PathBuf::from(home)],
            history_index: 0,
            config: VortexConfig::load(),
            file_list_widget: None,
            path_label: None,
            status_bar: None,
        }
    }
    
    fn navigate_to(&mut self, path: PathBuf) {
        if path.exists() && path.is_dir() {
            self.current_path = path.clone();
            // Add to history if it's different from current
            if self.history.get(self.history_index) != Some(&path) {
                self.history.truncate(self.history_index + 1);
                self.history.push(path);
                self.history_index = self.history.len() - 1;
            }
        }
    }
    
    fn can_go_back(&self) -> bool {
        self.history_index > 0
    }
    
    fn can_go_forward(&self) -> bool {
        self.history_index < self.history.len() - 1
    }
    
    fn go_back(&mut self) {
        if self.can_go_back() {
            self.history_index -= 1;
            self.current_path = self.history[self.history_index].clone();
        }
    }
    
    fn go_forward(&mut self) {
        if self.can_go_forward() {
            self.history_index += 1;
            self.current_path = self.history[self.history_index].clone();
        }
    }
    
    fn go_up(&mut self) {
        if let Some(parent) = self.current_path.parent() {
            self.navigate_to(parent.to_path_buf());
        }
    }
    
    fn refresh_ui(&self) {
        // Update path label
        if let Some(path_label) = &self.path_label {
            let current_path_str = self.current_path.to_string_lossy().to_string();
            path_label.set_text(&current_path_str);
        }
        
        // Update file list
        if let Some(file_list_widget) = &self.file_list_widget {
            self.update_file_list(file_list_widget);
        }
        
        // Update status bar
        if let Some(status_bar) = &self.status_bar {
            self.update_status_bar(status_bar);
        }
    }
    
    fn update_file_list(&self, scrolled: &gtk::ScrolledWindow) {
        // Clear existing content
        if let Some(child) = scrolled.child() {
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
        
        if let Ok(entries) = fs::read_dir(&self.current_path) {
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
            let file_box = create_file_item(icon, &name, file_type, path, &self.config);
            grid.attach(&file_box, col, row, 1, 1);
            
            col += 1;
            if col >= ITEMS_PER_ROW {
                col = 0;
                row += 1;
            }
        }
        
        scrolled.set_child(Some(&grid));
    }
    
    fn update_status_bar(&self, status_bar: &gtk::Box) {
        // Count actual items in directory
        let item_count = if let Ok(entries) = fs::read_dir(&self.current_path) {
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
            if let Some(label) = child.downcast_ref::<gtk::Label>() {
                label.set_text(&items_text);
            }
        }
    }
}

// File operations
fn copy_file(from: &PathBuf, to: &PathBuf) -> Result<()> {
    if from.is_dir() {
        copy_dir_all(from, to)?;
    } else {
        fs::copy(from, to)?;
    }
    Ok(())
}

fn move_file(from: &PathBuf, to: &PathBuf) -> Result<()> {
    fs::rename(from, to)?;
    Ok(())
}

fn delete_file(path: &PathBuf) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn rename_file(path: &PathBuf, new_name: &str) -> Result<()> {
    let parent = path.parent().unwrap();
    let new_path = parent.join(new_name);
    fs::rename(path, &new_path)?;
    Ok(())
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn build_ui(app: &Application) {
    // Create main window with split panes
    let state = Rc::new(RefCell::new(FileManagerState::new()));
    set_global_state(state.clone());
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Vortex FM")
        .default_width(state.borrow().config.window_width)
        .default_height(state.borrow().config.window_height)
        .build();

    // Create the split pane layout (like Windows Explorer!)
    let main_paned = Paned::new(Orientation::Horizontal);
    
    // Left sidebar (20%)
    let sidebar = create_sidebar(&state.borrow());
    main_paned.set_start_child(Some(&sidebar));
    
    // Main content area (80%)
    let content_area = create_content_area(&mut state.borrow_mut());
    main_paned.set_end_child(Some(&content_area));
    
    main_paned.set_position(state.borrow().config.sidebar_width);

    window.set_child(Some(&main_paned));
    
    // Add keyboard shortcuts
    setup_keyboard_shortcuts(&window);
    
    window.present();
}

fn create_sidebar(_state: &FileManagerState) -> Box {
    let sidebar = Box::new(Orientation::Vertical, 12);
    sidebar.set_margin_start(12);
    sidebar.set_margin_end(12);
    sidebar.set_margin_top(12);
    sidebar.set_margin_bottom(12);
    sidebar.add_css_class("sidebar");
    
    // Quick access section
    let quick_access_label = gtk::Label::new(Some("Quick Access"));
    quick_access_label.add_css_class("title-4");
    sidebar.append(&quick_access_label);
    
    // Quick access buttons
    let home_btn = gtk::Button::with_label("🏠 Home");
    let docs_btn = gtk::Button::with_label("📄 Documents");
    let downloads_btn = gtk::Button::with_label("📥 Downloads");
    let pictures_btn = gtk::Button::with_label("🖼️ Pictures");
    let music_btn = gtk::Button::with_label("🎵 Music");
    let videos_btn = gtk::Button::with_label("🎬 Videos");
    
    // Connect navigation handlers
    let home_path = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
    
    home_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            navigate_to_directory(PathBuf::from(&home_path));
        }
    });
    
    docs_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let docs_path = PathBuf::from(&home_path).join("Documents");
            navigate_to_directory(docs_path);
        }
    });
    
    downloads_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let downloads_path = PathBuf::from(&home_path).join("Downloads");
            navigate_to_directory(downloads_path);
        }
    });
    
    pictures_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let pictures_path = PathBuf::from(&home_path).join("Pictures");
            navigate_to_directory(pictures_path);
        }
    });
    
    music_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let music_path = PathBuf::from(&home_path).join("Music");
            navigate_to_directory(music_path);
        }
    });
    
    videos_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let videos_path = PathBuf::from(&home_path).join("Videos");
            navigate_to_directory(videos_path);
        }
    });
    
    sidebar.append(&home_btn);
    sidebar.append(&docs_btn);
    sidebar.append(&downloads_btn);
    sidebar.append(&pictures_btn);
    sidebar.append(&music_btn);
    sidebar.append(&videos_btn);
    
    // Add separator
    let separator = gtk::Separator::new(Orientation::Horizontal);
    sidebar.append(&separator);
    
    // This PC section
    let this_pc_label = gtk::Label::new(Some("This PC"));
    this_pc_label.add_css_class("title-4");
    sidebar.append(&this_pc_label);
    
    let desktop_btn = gtk::Button::with_label("🖥️ Desktop");
    let documents_btn = gtk::Button::with_label("📁 Documents");
    let downloads_btn2 = gtk::Button::with_label("📥 Downloads");
    let pictures_btn2 = gtk::Button::with_label("🖼️ Pictures");
    let music_btn2 = gtk::Button::with_label("🎵 Music");
    let videos_btn2 = gtk::Button::with_label("🎬 Videos");
    
    // Connect navigation handlers for This PC section
    let home_path = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
    
    desktop_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let desktop_path = PathBuf::from(&home_path).join("Desktop");
            navigate_to_directory(desktop_path);
        }
    });
    
    documents_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let docs_path = PathBuf::from(&home_path).join("Documents");
            navigate_to_directory(docs_path);
        }
    });
    
    downloads_btn2.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let downloads_path = PathBuf::from(&home_path).join("Downloads");
            navigate_to_directory(downloads_path);
        }
    });
    
    pictures_btn2.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let pictures_path = PathBuf::from(&home_path).join("Pictures");
            navigate_to_directory(pictures_path);
        }
    });
    
    music_btn2.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let music_path = PathBuf::from(&home_path).join("Music");
            navigate_to_directory(music_path);
        }
    });
    
    videos_btn2.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let videos_path = PathBuf::from(&home_path).join("Videos");
            navigate_to_directory(videos_path);
        }
    });
    
    sidebar.append(&desktop_btn);
    sidebar.append(&documents_btn);
    sidebar.append(&downloads_btn2);
    sidebar.append(&pictures_btn2);
    sidebar.append(&music_btn2);
    sidebar.append(&videos_btn2);
    
    sidebar
}

fn create_content_area(state: &mut FileManagerState) -> Box {
    let content = Box::new(Orientation::Vertical, 0);
    
    // Path bar (like Windows Explorer)
    let path_bar = create_path_bar(state);
    content.append(&path_bar);
    
    // File list area
    let file_list = create_file_list(state);
    content.append(&file_list);
    
    // Status bar
    let status_bar = create_status_bar(state);
    content.append(&status_bar);
    
    // Store references for later updates
    state.file_list_widget = Some(file_list.clone());
    state.status_bar = Some(status_bar.clone());
    
    content
}

fn create_path_bar(state: &mut FileManagerState) -> Box {
    let path_bar = Box::new(Orientation::Horizontal, 8);
    path_bar.add_css_class("toolbar");
    path_bar.set_margin_start(8);
    path_bar.set_margin_end(8);
    path_bar.set_margin_top(8);
    path_bar.set_margin_bottom(8);
    
    // Navigation buttons
    let back_btn = gtk::Button::from_icon_name("go-previous-symbolic");
    let forward_btn = gtk::Button::from_icon_name("go-next-symbolic");
    let up_btn = gtk::Button::from_icon_name("go-up-symbolic");
    let refresh_btn = gtk::Button::from_icon_name("view-refresh-symbolic");
    
    // Disable buttons initially
    back_btn.set_sensitive(false);
    forward_btn.set_sensitive(false);
    up_btn.set_sensitive(false);
    
    path_bar.append(&back_btn);
    path_bar.append(&forward_btn);
    path_bar.append(&up_btn);
    path_bar.append(&refresh_btn);
    
    // Separator
    let separator = gtk::Separator::new(Orientation::Vertical);
    path_bar.append(&separator);
    
    // Path display
    let current_path_str = state.current_path.to_string_lossy().to_string();
    let path_label = gtk::Label::new(Some(&current_path_str));
    path_label.set_halign(gtk::Align::Start);
    path_label.set_hexpand(true);
    path_label.add_css_class("path-label");
    path_bar.append(&path_label);
    
    // Store reference for later updates
    state.path_label = Some(path_label.clone());
    
    // Search box
    let search_entry = gtk::SearchEntry::new();
    search_entry.set_placeholder_text(Some("Search files..."));
    search_entry.set_width_request(200);
    path_bar.append(&search_entry);
    
    path_bar
}

fn create_file_list(state: &FileManagerState) -> gtk::ScrolledWindow {
    let scrolled = gtk::ScrolledWindow::new();
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);
    
    // Use the state's method to populate the file list
    state.update_file_list(&scrolled);
    
    scrolled
}

fn create_file_item(icon: &str, name: &str, _file_type: &str, path: PathBuf, config: &VortexConfig) -> gtk::Button {
    let item_box = Box::new(Orientation::Vertical, 4);
    item_box.set_width_request(80);
    item_box.set_height_request(80);
    item_box.add_css_class("file-item");
    
    // File icon
    let icon_label = gtk::Label::new(Some(icon));
    icon_label.add_css_class("file-icon");
    icon_label.set_halign(gtk::Align::Center);
    item_box.append(&icon_label);
    
    // File name
    let name_label = gtk::Label::new(Some(name));
    name_label.add_css_class("file-name");
    name_label.set_halign(gtk::Align::Center);
    name_label.set_wrap(true);
    name_label.set_max_width_chars(10);
    item_box.append(&name_label);
    
    // Make it clickable
    let button = gtk::Button::new();
    button.set_child(Some(&item_box));
    
    // TODO: Add context menu support
    // Note: Context menus require more complex setup in GTK4
    
    // Connect click handler
    let _name_clone = name.to_string();
    let path_clone = path.clone();
    let _single_click = config.single_click_to_open;
    
    button.connect_clicked(move |_| {
        if path_clone.is_dir() {
            println!("📁 Opening directory: {}", path_clone.display());
            navigate_to_directory(path_clone.clone());
        } else {
            println!("📄 Opening file: {}", path_clone.display());
            // TODO: Open file with default application
        }
    });
    
    // TODO: Add double-click handler if single click is disabled
    // Note: GTK4 Button doesn't have connect_button_press_event, need different approach
    
    // Return the button
    button
}

// TODO: Implement context menu functionality
// This requires more complex GTK4 setup with proper menu handling

fn setup_keyboard_shortcuts(window: &ApplicationWindow) {
    // Create action group
    let action_group = gio::SimpleActionGroup::new();
    
    // Copy action (Ctrl+C)
    let copy_action = gio::SimpleAction::new("copy", None);
    copy_action.connect_activate(|_, _| {
        println!("📋 Copy action triggered (Ctrl+C)");
        // TODO: Implement copy functionality
    });
    action_group.add_action(&copy_action);
    
    // Cut action (Ctrl+X)
    let cut_action = gio::SimpleAction::new("cut", None);
    cut_action.connect_activate(|_, _| {
        println!("✂️ Cut action triggered (Ctrl+X)");
        // TODO: Implement cut functionality
    });
    action_group.add_action(&cut_action);
    
    // Paste action (Ctrl+V)
    let paste_action = gio::SimpleAction::new("paste", None);
    paste_action.connect_activate(|_, _| {
        println!("📋 Paste action triggered (Ctrl+V)");
        // TODO: Implement paste functionality
    });
    action_group.add_action(&paste_action);
    
    // Delete action (Delete key)
    let delete_action = gio::SimpleAction::new("delete", None);
    delete_action.connect_activate(|_, _| {
        println!("🗑️ Delete action triggered (Delete)");
        // TODO: Implement delete functionality
    });
    action_group.add_action(&delete_action);
    
    // Rename action (F2)
    let rename_action = gio::SimpleAction::new("rename", None);
    rename_action.connect_activate(|_, _| {
        println!("✏️ Rename action triggered (F2)");
        // TODO: Implement rename functionality
    });
    action_group.add_action(&rename_action);
    
    // Refresh action (F5)
    let refresh_action = gio::SimpleAction::new("refresh", None);
    refresh_action.connect_activate(|_, _| {
        println!("🔄 Refresh action triggered (F5)");
        // TODO: Implement refresh functionality
    });
    action_group.add_action(&refresh_action);
    
    // Add action group to window
    window.insert_action_group("file", Some(&action_group));
    
    // Create application actions
    let app = window.application().unwrap();
    
    // New folder action (Ctrl+Shift+N)
    let new_folder_action = gio::SimpleAction::new("new-folder", None);
    new_folder_action.connect_activate(|_, _| {
        println!("📁 New folder action triggered (Ctrl+Shift+N)");
        // TODO: Implement new folder functionality
    });
    app.add_action(&new_folder_action);
}

fn create_status_bar(state: &FileManagerState) -> Box {
    let status_bar = Box::new(Orientation::Horizontal, 12);
    status_bar.add_css_class("toolbar");
    status_bar.add_css_class("status-bar");
    status_bar.set_margin_start(4);
    status_bar.set_margin_end(4);
    status_bar.set_margin_top(4);
    status_bar.set_margin_bottom(4);
    
    let status_label = gtk::Label::new(Some("Ready"));
    status_label.set_halign(gtk::Align::Start);
    status_label.set_hexpand(true);
    status_bar.append(&status_label);
    
    // Count actual items in directory
    let item_count = if let Ok(entries) = fs::read_dir(&state.current_path) {
        entries.count()
    } else {
        0
    };
    
    let items_text = if item_count == 1 {
        "1 item".to_string()
    } else {
        format!("{} items", item_count)
    };
    
    let items_label = gtk::Label::new(Some(&items_text));
    status_bar.append(&items_label);
    
    // Add view mode buttons
    let view_box = Box::new(Orientation::Horizontal, 4);
    let list_view_btn = gtk::Button::from_icon_name("view-list-symbolic");
    let grid_view_btn = gtk::Button::from_icon_name("view-grid-symbolic");
    grid_view_btn.add_css_class("suggested-action");
    
    view_box.append(&list_view_btn);
    view_box.append(&grid_view_btn);
    status_bar.append(&view_box);
    
    status_bar
}
