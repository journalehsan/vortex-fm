use gtk::prelude::*;
use gtk::{gio, glib, Application, ApplicationWindow, Box, Orientation, Paned, CssProvider, PopoverMenu, MenuButton, MenuItem};
use std::path::PathBuf;
use std::fs;
use anyhow::Result;

const APP_ID: &str = "com.vortex.FileManager";

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
}

impl FileManagerState {
    fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        Self {
            current_path: PathBuf::from(&home),
            history: vec![PathBuf::from(home)],
            history_index: 0,
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
}

fn build_ui(app: &Application) {
    // Create main window with split panes
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Vortex FM")
        .default_width(1200)
        .default_height(800)
        .build();

    // Create file manager state
    let state = FileManagerState::new();
    
    // Create the split pane layout (like Windows Explorer!)
    let main_paned = Paned::new(Orientation::Horizontal);
    
    // Left sidebar (20%)
    let sidebar = create_sidebar(&state);
    main_paned.set_start_child(Some(&sidebar));
    
    // Main content area (80%)
    let content_area = create_content_area(&state);
    main_paned.set_end_child(Some(&content_area));
    
    main_paned.set_position(250); // 250px sidebar width

    window.set_child(Some(&main_paned));
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
    
    sidebar.append(&desktop_btn);
    sidebar.append(&documents_btn);
    sidebar.append(&downloads_btn2);
    sidebar.append(&pictures_btn2);
    sidebar.append(&music_btn2);
    sidebar.append(&videos_btn2);
    
    sidebar
}

fn create_content_area(state: &FileManagerState) -> Box {
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
    
    content
}

fn create_path_bar(state: &FileManagerState) -> Box {
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
    
    // Create a grid for file icons (like Windows Explorer)
    let grid = gtk::Grid::new();
    grid.set_row_spacing(12);
    grid.set_column_spacing(12);
    grid.set_margin_start(12);
    grid.set_margin_end(12);
    grid.set_margin_top(12);
    grid.set_margin_bottom(12);
    
    // Read actual files from the current directory
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&state.current_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
            
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
    
    let mut row = 0;
    let mut col = 0;
    const ITEMS_PER_ROW: i32 = 6;
    
    for (icon, name, file_type, path) in files {
        let file_box = create_file_item(icon, &name, file_type, path);
        grid.attach(&file_box, col, row, 1, 1);
        
        col += 1;
        if col >= ITEMS_PER_ROW {
            col = 0;
            row += 1;
        }
    }
    
    scrolled.set_child(Some(&grid));
    scrolled
}

fn create_file_item(icon: &str, name: &str, _file_type: &str, path: PathBuf) -> gtk::Button {
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
    
    // Connect click handler
    let _name_clone = name.to_string();
    let path_clone = path.clone();
    button.connect_clicked(move |_| {
        if path_clone.is_dir() {
            println!("📁 Opening directory: {}", path_clone.display());
            // TODO: Navigate to directory
        } else {
            println!("📄 Opening file: {}", path_clone.display());
            // TODO: Open file with default application
        }
    });
    
    // Return the button
    button
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
