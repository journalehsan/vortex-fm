use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Widget};

use crate::core::file_manager::FileManagerState;
use crate::utils::search::{filter_files_in_directory, FileEntry};
use std::fs;
use gtk::{ListBox, ListBoxRow, FlowBox, FlowBoxChild, Label};

// Adapter trait: each view implements this to provide its widget and lifecycle hooks
pub trait FileViewAdapter {
    fn build(&mut self, state: &FileManagerState) -> Widget;
    fn refresh(&mut self, state: &FileManagerState);
    fn set_icon_size(&mut self, _size: i32) { let _ = _size; }
    fn update(&mut self, state: &FileManagerState) {
        // Default: do full refresh. Adapters can override for in-place updates.
        self.refresh(state);
    }
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

    pub fn update(&mut self, state: &FileManagerState) {
        if let Some(adapter) = self.adapter.as_mut() {
            adapter.update(state);
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

        // Use the search utility to get filtered files
        let files = filter_files_in_directory(&state.current_path(), &state.current_filter, &state.config);

        // Create simple rows with name labels (clicks handled by file_item in grid; here we keep minimal)
        for file_entry in files {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 8);
            let name_label = Label::new(Some(&file_entry.name));
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

    fn update(&mut self, state: &FileManagerState) {
        // In-place update: rebuild the list without replacing the entire widget
        if let Some(root) = &self.root {
            // Clear existing list
            while let Some(child) = root.first_child() {
                root.remove(&child);
            }
            
            let list = ListBox::new();
            list.set_selection_mode(gtk::SelectionMode::None);

            // Use the search utility to get filtered files
            let files = filter_files_in_directory(&state.current_path(), &state.current_filter, &state.config);

            for file_entry in files {
                let row = ListBoxRow::new();
                let row_box = GtkBox::new(Orientation::Horizontal, 8);
                let name_label = Label::new(Some(&file_entry.name));
                name_label.set_xalign(0.0);
                name_label.set_hexpand(true);
                row_box.append(&name_label);
                row.set_child(Some(&row_box));
                list.append(&row);
            }

            root.append(&list);
        }
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

        // Use the search utility to get filtered files
        let files = filter_files_in_directory(&state.current_path(), &state.current_filter, &state.config);

        for file_entry in files {
            let btn = crate::widgets::file_item::create_file_item(&file_entry.icon, &file_entry.name, &file_entry.file_type, file_entry.path, &state.config);
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

    fn refresh(&mut self, _state: &FileManagerState) {
        // no-op placeholder
    }

    fn update(&mut self, state: &FileManagerState) {
        // In-place update: clear and rebuild flowbox without replacing entire widget
        if let Some(root) = &self.root {
            // Clear existing children
            while let Some(child) = root.first_child() {
                root.remove(&child);
            }
            
            let flow = FlowBox::new();
            flow.set_selection_mode(gtk::SelectionMode::None);
            flow.set_row_spacing(12);
            flow.set_column_spacing(12);
            flow.set_margin_start(12);
            flow.set_margin_end(12);
            flow.set_margin_top(12);
            flow.set_margin_bottom(12);

            // Use the search utility to get filtered files
            let files = filter_files_in_directory(&state.current_path(), &state.current_filter, &state.config);

            for file_entry in files {
                let btn = crate::widgets::file_item::create_file_item(&file_entry.icon, &file_entry.name, &file_entry.file_type, file_entry.path, &state.config);
                let child = FlowBoxChild::new();
                child.set_child(Some(&btn));
                flow.insert(&child, -1);
            }

            root.append(&flow);
        }
    }
}


