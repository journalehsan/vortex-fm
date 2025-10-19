use gtk::prelude::*;
use gtk::{Box, Orientation, Label, ScrolledWindow, Separator, Image, Picture};
use std::path::PathBuf;
use crate::utils::file_ops::get_file_info;
use crate::utils::thumbnails::get_global_thumbnail_manager;

pub fn create_details_panel() -> Box {
    let details_panel = Box::new(Orientation::Vertical, 8);
    details_panel.add_css_class("details-panel");
    details_panel.set_width_request(250);
    details_panel.set_margin_start(8);
    details_panel.set_margin_end(8);
    details_panel.set_margin_top(8);
    details_panel.set_margin_bottom(8);

    // Header
    let header_label = Label::new(Some("Details"));
    header_label.add_css_class("details-header");
    header_label.set_halign(gtk::Align::Start);
    details_panel.append(&header_label);

    // Separator
    let separator = Separator::new(Orientation::Horizontal);
    details_panel.append(&separator);

    // Scrollable content area
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hscrollbar_policy(gtk::PolicyType::Never);
    details_panel.append(&scrolled);

    // Content container
    let content_box = Box::new(Orientation::Vertical, 12);
    content_box.set_margin_start(8);
    content_box.set_margin_end(8);
    content_box.set_margin_top(8);
    content_box.set_margin_bottom(8);
    scrolled.set_child(Some(&content_box));

    // Default "No selection" message
    let no_selection_label = Label::new(Some("No file selected"));
    no_selection_label.add_css_class("details-no-selection");
    no_selection_label.set_halign(gtk::Align::Center);
    no_selection_label.set_vexpand(true);
    content_box.append(&no_selection_label);

    details_panel
}

pub fn update_details_panel(details_panel: &Box, file_path: Option<&PathBuf>) {
    crate::utils::simple_debug::debug_debug("DETAILS_PANEL", &format!("Updating details panel with: {:?}", 
        file_path.map(|p| p.display().to_string())));
    
    // Find the content box (scrolled window's child)
    if let Some(scrolled) = details_panel.last_child() {
        crate::utils::simple_debug::debug_trace("DETAILS_PANEL", &format!("Found scrolled window: {}", scrolled.type_()));
        
        if let Some(scrolled_window) = scrolled.downcast_ref::<ScrolledWindow>() {
            crate::utils::simple_debug::debug_trace("DETAILS_PANEL", "ScrolledWindow downcast successful");
            
            if let Some(viewport) = scrolled_window.child() {
                crate::utils::simple_debug::debug_trace("DETAILS_PANEL", &format!("Found viewport: {}", viewport.type_()));
                
                if let Some(viewport_widget) = viewport.downcast_ref::<gtk::Viewport>() {
                    crate::utils::simple_debug::debug_trace("DETAILS_PANEL", "Viewport downcast successful");
                    
                    if let Some(content_box) = viewport_widget.child() {
                        crate::utils::simple_debug::debug_trace("DETAILS_PANEL", &format!("Found content box: {}", content_box.type_()));
                        
                        if let Some(content) = content_box.downcast_ref::<Box>() {
                            crate::utils::simple_debug::debug_trace("DETAILS_PANEL", "Content box downcast successful");
                            
                            // Clear existing content
                            crate::utils::simple_debug::debug_trace("DETAILS_PANEL", "Clearing existing content");
                            
                            while let Some(child) = content.first_child() {
                                content.remove(&child);
                            }
                            
                            crate::utils::simple_debug::debug_trace("DETAILS_PANEL", "Content cleared successfully");

                            if let Some(path) = file_path {
                                crate::utils::simple_debug::debug_info("DETAILS_PANEL", &format!("Processing file: {}", path.display()));
                                
                                // Show file details
                                if let Ok(file_info) = get_file_info(path) {
                                    crate::utils::simple_debug::debug_trace("DETAILS_PANEL", "File info retrieved successfully");
                                    create_file_details(content, &file_info);
                                } else {
                                    crate::utils::simple_debug::debug_error("DETAILS_PANEL", &format!("Failed to get file info for: {}", path.display()));
                                    create_error_details(content, "Could not read file information");
                                }
                            } else {
                                crate::utils::simple_debug::debug_info("DETAILS_PANEL", "Processing folder details");
                                
                                // Show current folder details
                                if let Some(current_path) = get_current_folder_path() {
                                    crate::utils::simple_debug::debug_trace("DETAILS_PANEL", &format!("Current folder: {}", current_path.display()));
                                    create_folder_details(content, &current_path);
                                } else {
                                    crate::utils::simple_debug::debug_warning("DETAILS_PANEL", "No current folder path found");
                                    create_no_selection_details(content);
                                }
                            }
                            
                            crate::utils::simple_debug::debug_info("DETAILS_PANEL", "Details panel update complete");
                        } else {
                            crate::utils::simple_debug::debug_error("DETAILS_PANEL", "Failed to downcast content box to Box");
                        }
                    } else {
                        crate::utils::simple_debug::debug_error("DETAILS_PANEL", "No content box found in viewport");
                    }
                } else {
                    crate::utils::simple_debug::debug_error("DETAILS_PANEL", "Failed to downcast viewport to Viewport");
                }
            } else {
                crate::utils::simple_debug::debug_error("DETAILS_PANEL", "No viewport found in scrolled window");
            }
        } else {
            crate::utils::simple_debug::debug_error("DETAILS_PANEL", "Failed to downcast to ScrolledWindow");
        }
    } else {
        crate::utils::simple_debug::debug_error("DETAILS_PANEL", "No scrolled window found in details panel");
    }
}

fn get_current_folder_path() -> Option<PathBuf> {
    if let Some(state_rc) = crate::core::navigation::get_global_state() {
        Some(state_rc.borrow().current_path().clone())
    } else {
        None
    }
}

fn create_file_details(content: &Box, file_info: &crate::utils::file_ops::FileInfo) {
    // Check if it's an image file for preview
    let thumbnail_manager = get_global_thumbnail_manager();
    let is_image = thumbnail_manager.is_image_file(&file_info.path);
    
    // File icon/thumbnail and name
    let header_box = Box::new(Orientation::Horizontal, 12);
    header_box.set_margin_bottom(16);

    if is_image {
        // Show thumbnail for images
        if let Some(thumbnail_path) = thumbnail_manager.get_thumbnail_or_placeholder(&file_info.path) {
            let picture = Picture::for_filename(&thumbnail_path);
            picture.add_css_class("details-thumbnail");
            picture.set_width_request(64);
            picture.set_height_request(64);
            picture.set_can_shrink(false);
            header_box.append(&picture);
        } else {
            // Fallback to emoji icon
            let icon_label = Label::new(Some(&get_file_icon(&file_info.file_type)));
            icon_label.add_css_class("details-file-icon");
            icon_label.set_width_request(48);
            icon_label.set_height_request(48);
            header_box.append(&icon_label);
        }
    } else {
        // File icon (using emoji for now, could be enhanced with real icons)
        let icon_label = Label::new(Some(&get_file_icon(&file_info.file_type)));
        icon_label.add_css_class("details-file-icon");
        icon_label.set_width_request(48);
        icon_label.set_height_request(48);
        header_box.append(&icon_label);
    }

    // File name and type
    let name_box = Box::new(Orientation::Vertical, 4);
    let name_label = Label::new(Some(&file_info.name));
    name_label.add_css_class("details-file-name");
    name_label.set_halign(gtk::Align::Start);
    name_label.set_hexpand(true);
    name_label.set_wrap(true);

    let type_label = Label::new(Some(&file_info.file_type));
    type_label.add_css_class("details-file-type");
    type_label.set_halign(gtk::Align::Start);

    name_box.append(&name_label);
    name_box.append(&type_label);
    header_box.append(&name_box);

    content.append(&header_box);

    // Separator
    let separator1 = Separator::new(Orientation::Horizontal);
    content.append(&separator1);

    // File properties
    let properties_box = Box::new(Orientation::Vertical, 8);
    properties_box.set_margin_top(8);

    // Size
    let size_formatted = format_size(file_info.size);
    create_property_row(&properties_box, "Size:", &size_formatted);
    
    // Location
    let location = file_info.path.parent().unwrap_or(&file_info.path).to_string_lossy();
    create_property_row(&properties_box, "Location:", &location);
    
    // Modified
    let modified_date = format_system_time(&file_info.modified);
    create_property_row(&properties_box, "Modified:", &modified_date);
    
    // Path
    create_property_row(&properties_box, "Path:", &file_info.path.to_string_lossy());

    content.append(&properties_box);
}

fn create_folder_details(content: &Box, folder_path: &PathBuf) {
    // Folder icon and name
    let header_box = Box::new(Orientation::Horizontal, 12);
    header_box.set_margin_bottom(16);

    // Folder icon
    let icon_label = Label::new(Some("📁"));
    icon_label.add_css_class("details-file-icon");
    icon_label.set_width_request(48);
    icon_label.set_height_request(48);
    header_box.append(&icon_label);

    // Folder name and type
    let name_box = Box::new(Orientation::Vertical, 4);
    let folder_name = folder_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown Folder");
    let name_label = Label::new(Some(folder_name));
    name_label.add_css_class("details-file-name");
    name_label.set_halign(gtk::Align::Start);
    name_label.set_hexpand(true);
    name_label.set_wrap(true);

    let type_label = Label::new(Some("Folder"));
    type_label.add_css_class("details-file-type");
    type_label.set_halign(gtk::Align::Start);

    name_box.append(&name_label);
    name_box.append(&type_label);
    header_box.append(&name_box);

    content.append(&header_box);

    // Separator
    let separator1 = Separator::new(Orientation::Horizontal);
    content.append(&separator1);

    // Folder properties
    let properties_box = Box::new(Orientation::Vertical, 8);
    properties_box.set_margin_top(8);

    // Count items in folder
    let item_count = count_folder_items(folder_path);
    let items_text = if item_count == 1 {
        "1 item".to_string()
    } else {
        format!("{} items", item_count)
    };
    create_property_row(&properties_box, "Items:", &items_text);
    
    // Location
    let location = folder_path.parent().unwrap_or(folder_path).to_string_lossy();
    create_property_row(&properties_box, "Location:", &location);
    
    // Path
    create_property_row(&properties_box, "Path:", &folder_path.to_string_lossy());

    content.append(&properties_box);
}

fn count_folder_items(folder_path: &PathBuf) -> usize {
    if let Ok(entries) = std::fs::read_dir(folder_path) {
        entries.count()
    } else {
        0
    }
}

fn create_property_row(parent: &Box, label: &str, value: &str) {
    let row_box = Box::new(Orientation::Vertical, 4);
    
    let label_widget = Label::new(Some(label));
    label_widget.add_css_class("details-property-label");
    label_widget.set_halign(gtk::Align::Start);
    
    let value_widget = Label::new(Some(value));
    value_widget.add_css_class("details-property-value");
    value_widget.set_halign(gtk::Align::Start);
    value_widget.set_wrap(true);
    value_widget.set_hexpand(true);
    
    row_box.append(&label_widget);
    row_box.append(&value_widget);
    parent.append(&row_box);
}

fn create_error_details(content: &Box, message: &str) {
    let error_label = Label::new(Some(message));
    error_label.add_css_class("details-error");
    error_label.set_halign(gtk::Align::Center);
    error_label.set_vexpand(true);
    content.append(&error_label);
}

fn create_no_selection_details(content: &Box) {
    let no_selection_label = Label::new(Some("No file selected"));
    no_selection_label.add_css_class("details-no-selection");
    no_selection_label.set_halign(gtk::Align::Center);
    no_selection_label.set_vexpand(true);
    content.append(&no_selection_label);
}

fn get_file_icon(file_type: &str) -> &'static str {
    match file_type {
        t if t.contains("Folder") => "📁",
        t if t.contains("Image") => "🖼️",
        t if t.contains("Audio") => "🎵",
        t if t.contains("Video") => "🎬",
        t if t.contains("Text") => "📄",
        t if t.contains("PDF") => "📕",
        t if t.contains("Document") => "📘",
        t if t.contains("Spreadsheet") => "📊",
        t if t.contains("Presentation") => "📽️",
        t if t.contains("Archive") => "📦",
        t if t.contains("Script") => "💻",
        _ => "📄",
    }
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

fn format_system_time(time: &std::time::SystemTime) -> String {
    use chrono::{Local, TimeZone};
    
    match time.duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => {
            let timestamp = duration.as_secs() as i64;
            match Local.timestamp_opt(timestamp, duration.subsec_nanos()) {
                chrono::LocalResult::Single(datetime) => {
                    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                }
                chrono::LocalResult::Ambiguous(datetime, _) => {
                    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                }
                chrono::LocalResult::None => {
                    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
                }
            }
        }
        Err(_) => "Unknown".to_string(),
    }
}

// Global details panel reference for updates
static mut GLOBAL_DETAILS_PANEL: Option<Box> = None;

pub fn set_global_details_panel(details_panel: Box) {
    unsafe {
        GLOBAL_DETAILS_PANEL = Some(details_panel);
    }
}

pub fn update_global_details_panel(file_path: Option<&PathBuf>) {
    unsafe {
        if let Some(details_panel) = &GLOBAL_DETAILS_PANEL {
            update_details_panel(details_panel, file_path);
        }
    }
}
