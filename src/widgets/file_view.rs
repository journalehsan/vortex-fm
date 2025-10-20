use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Widget};

use crate::core::file_manager::FileManagerState;
use std::fs;
use gtk::{ListBox, ListBoxRow, FlowBox, FlowBoxChild, Label};

// Adapter trait: each view implements this to provide its widget and lifecycle hooks
pub trait FileViewAdapter {
    fn build(&mut self, state: &FileManagerState) -> Widget;
    fn refresh(&mut self, state: &FileManagerState);
    fn set_icon_size(&mut self, _size: i32) { let _ = _size; }
}

// Base FileView container that hosts the active adapter's widget
pub struct FileView {
    container: GtkBox,
    adapter: Option<std::boxed::Box<dyn FileViewAdapter>>,
}

impl FileView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 0);
        FileView { container, adapter: None }
    }

    pub fn widget(&self) -> &GtkBox { &self.container }

    pub fn set_adapter(&mut self, mut adapter: std::boxed::Box<dyn FileViewAdapter>, state: &FileManagerState) {
        if let Some(child) = self.container.first_child() { self.container.remove(&child); }
        let view = adapter.build(state);
        self.container.append(&view);
        self.adapter = Some(adapter);
    }

    pub fn refresh(&mut self, state: &FileManagerState) {
        if let Some(adapter) = self.adapter.as_mut() {
            // Rebuild the adapter view to reflect current directory/state
            if let Some(child) = self.container.first_child() { self.container.remove(&child); }
            let view = adapter.build(state);
            self.container.append(&view);
        }
    }

    pub fn set_icon_size(&mut self, size: i32) {
        if let Some(adapter) = self.adapter.as_mut() { adapter.set_icon_size(size); }
    }
}

// Placeholder ListView adapter
pub struct ListViewAdapter {
    root: Option<GtkBox>,
}

impl ListViewAdapter { pub fn new() -> Self { Self { root: None } } }

impl FileViewAdapter for ListViewAdapter {
    fn build(&mut self, state: &FileManagerState) -> Widget {
        let root = GtkBox::new(Orientation::Vertical, 0);
        root.set_css_classes(&["fileview-list"]);
        let list = ListBox::new();
        list.set_selection_mode(gtk::SelectionMode::None);

        // Read and sort items like existing view: dirs first, then files
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(state.current_path()) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();
                if !state.config.show_hidden_files && name.starts_with('.') { continue; }
                files.push((name, path));
            }
        }
        files.sort_by(|a, b| {
            let a_is_dir = a.1.is_dir();
            let b_is_dir = b.1.is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.0.cmp(&b.0),
            }
        });

        // Create simple rows with name labels (clicks handled by file_item in grid; here we keep minimal)
        for (name, _path) in files {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 8);
            let name_label = Label::new(Some(&name));
            name_label.set_xalign(0.0);
            name_label.set_hexpand(true);
            row_box.append(&name_label);
            row.set_child(Some(&row_box));
            list.append(&row);
        }

        root.append(&list);
        let w: Widget = root.clone().upcast();
        self.root = Some(root);
        w
    }

    fn refresh(&mut self, _state: &FileManagerState) {
        // no-op placeholder
    }
}

// Placeholder GridView adapter
pub struct GridViewAdapter {
    root: Option<GtkBox>,
    icon_size: i32,
}

impl GridViewAdapter { pub fn new() -> Self { Self { root: None, icon_size: 64 } } }

impl FileViewAdapter for GridViewAdapter {
    fn build(&mut self, state: &FileManagerState) -> Widget {
        let root = GtkBox::new(Orientation::Vertical, 0);
        root.set_css_classes(&["fileview-grid"]);
        let flow = FlowBox::new();
        flow.set_selection_mode(gtk::SelectionMode::None);
        flow.set_row_spacing(12);
        flow.set_column_spacing(12);
        flow.set_margin_start(12);
        flow.set_margin_end(12);
        flow.set_margin_top(12);
        flow.set_margin_bottom(12);

        // Gather files similar to existing implementation
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(state.current_path()) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();
                if !state.config.show_hidden_files && name.starts_with('.') { continue; }
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
        files.sort_by(|a, b| {
            let a_is_dir = a.3.is_dir();
            let b_is_dir = b.3.is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.1.cmp(&b.1),
            }
        });

        for (icon, name, file_type, path) in files {
            let btn = crate::widgets::file_item::create_file_item(icon, &name, file_type, path, &state.config);
            let child = FlowBoxChild::new();
            child.set_child(Some(&btn));
            // Insert at end (-1) for gtk4-rs 0.7
            flow.insert(&child, -1);
        }

        root.append(&flow);
        let w: Widget = root.clone().upcast();
        self.root = Some(root);
        w
    }

    fn refresh(&mut self, _state: &FileManagerState) { }

    fn set_icon_size(&mut self, size: i32) { self.icon_size = size; }
}


