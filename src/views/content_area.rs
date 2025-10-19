use gtk::prelude::*;
use gtk::{Box, Orientation, ScrolledWindow};
use crate::core::file_manager::FileManagerState;
use crate::views::path_bar::create_path_bar;
use crate::views::status_bar::create_status_bar;

pub fn create_content_area(state: &mut FileManagerState) -> Box {
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

fn create_file_list(state: &FileManagerState) -> ScrolledWindow {
    let scrolled = ScrolledWindow::new();
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);
    
    // Use the state's method to populate the file list
    state.update_file_list(&scrolled);
    
    scrolled
}
