use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Separator, Picture};
use std::path::PathBuf;
use crate::utils::file_ops::get_file_info;
use crate::utils::thumbnails::get_global_thumbnail_manager;

pub fn create_details_panel() -> Box {
    let details_panel = Box::new(Orientation::Vertical, 0);
    details_panel.add_css_class("details-panel");
    // Let details panel size to its content
    details_panel.set_height_request(-1);
    details_panel.set_margin_start(0);
    details_panel.set_margin_end(0);
    details_panel.set_margin_top(0);
    details_panel.set_margin_bottom(0);
    details_panel.set_valign(gtk::Align::Center);

    // Content container (horizontal layout: photo | details | info)
    let content_box = Box::new(Orientation::Horizontal, 0);
    content_box.set_margin_start(0);
    content_box.set_margin_end(0);
    content_box.set_margin_top(0);
    content_box.set_margin_bottom(0);
    content_box.set_valign(gtk::Align::Center);
    content_box.set_halign(gtk::Align::Center);
    // Let content box size naturally
    content_box.set_height_request(-1);
    
    // Default "No selection" message
    let no_selection_label = Label::new(Some("No file selected"));
    no_selection_label.add_css_class("details-no-selection");
    no_selection_label.set_halign(gtk::Align::Center);
    no_selection_label.set_valign(gtk::Align::Center);
    content_box.append(&no_selection_label);
    
    details_panel.append(&content_box);

    details_panel
}

pub fn update_details_panel(details_panel: &Box, file_path: Option<&PathBuf>) {
    crate::utils::simple_debug::debug_debug("DETAILS_PANEL", &format!("Updating details panel with: {:?}", 
        file_path.map(|p| p.display().to_string())));
    
    // Find the content box (first child - horizontal box)
    if let Some(content) = details_panel.first_child() {
        if let Some(content_box) = content.downcast_ref::<Box>() {
            crate::utils::simple_debug::debug_trace("DETAILS_PANEL", "Content box found");
            
            // Clear existing content
            while let Some(child) = content_box.first_child() {
                content_box.remove(&child);
            }
            
            crate::utils::simple_debug::debug_trace("DETAILS_PANEL", "Content cleared successfully");

            if let Some(path) = file_path {
                crate::utils::simple_debug::debug_info("DETAILS_PANEL", &format!("Processing file: {}", path.display()));
                
                // Show file details
                if let Ok(file_info) = get_file_info(path) {
                    crate::utils::simple_debug::debug_trace("DETAILS_PANEL", "File info retrieved successfully");
                    create_file_details(content_box, &file_info);
                } else {
                    crate::utils::simple_debug::debug_error("DETAILS_PANEL", &format!("Failed to get file info for: {}", path.display()));
                    create_error_details(content_box, "Could not read file information");
                }
            } else {
                crate::utils::simple_debug::debug_info("DETAILS_PANEL", "Processing folder details");
                
                // Show current folder details
                if let Some(current_path) = get_current_folder_path() {
                    crate::utils::simple_debug::debug_trace("DETAILS_PANEL", &format!("Current folder: {}", current_path.display()));
                    create_folder_details(content_box, &current_path);
                } else {
                    crate::utils::simple_debug::debug_warning("DETAILS_PANEL", "No current folder path found");
                    create_no_selection_details(content_box);
                }
            }
            
            crate::utils::simple_debug::debug_info("DETAILS_PANEL", "Details panel update complete");
        }
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
    let thumbnail_manager = get_global_thumbnail_manager();
    let is_image = thumbnail_manager.is_image_file(&file_info.path);
    
    // LEFT: Thumbnail/Icon (70px - compact)
    let left_box = Box::new(Orientation::Vertical, 0);
    left_box.set_width_request(64);
    left_box.set_halign(gtk::Align::Center);
    left_box.set_valign(gtk::Align::Center);
    
    if is_image {
        if let Some(thumbnail_path) = thumbnail_manager.get_thumbnail_or_placeholder(&file_info.path) {
            let picture = Picture::for_filename(&thumbnail_path);
            picture.add_css_class("details-thumbnail-compact");
            picture.set_width_request(52);
            picture.set_height_request(52);
            picture.set_can_shrink(false);
            picture.set_valign(gtk::Align::Center);
            left_box.append(&picture);
        } else {
            let icon_label = Label::new(Some(&get_file_icon(&file_info.file_type)));
            icon_label.add_css_class("details-file-icon-compact");
            icon_label.set_halign(gtk::Align::Center);
            icon_label.set_valign(gtk::Align::Center);
            left_box.append(&icon_label);
        }
    } else {
        let icon_label = Label::new(Some(&get_file_icon(&file_info.file_type)));
        icon_label.add_css_class("details-file-icon-compact");
        icon_label.set_halign(gtk::Align::Center);
        icon_label.set_valign(gtk::Align::Center);
        left_box.append(&icon_label);
    }
    content.append(&left_box);
    
    // DIVIDER
    let divider = Separator::new(Orientation::Vertical);
    divider.set_valign(gtk::Align::Center);
    divider.set_height_request(44); // slightly smaller for better proportion
    content.append(&divider);
    
    // MIDDLE: Details (3 rows - tighter spacing)
    let middle_box = Box::new(Orientation::Vertical, 4);
    middle_box.set_hexpand(true);
    middle_box.set_valign(gtk::Align::Center);
    
    // Row 1: Name
    let name_row = create_compact_row("Name:", &file_info.name);
    middle_box.append(&name_row);
    
    // Row 2: Size
    let size_formatted = format_size(file_info.size);
    let size_row = create_compact_row("Size:", &size_formatted);
    middle_box.append(&size_row);
    
    // Row 3: Type
    let type_row = create_compact_row("Type:", &file_info.file_type);
    middle_box.append(&type_row);
    
    content.append(&middle_box);
    
    // DIVIDER
    let divider2 = Separator::new(Orientation::Vertical);
    divider2.set_valign(gtk::Align::Center);
    divider2.set_height_request(44); // slightly smaller for better proportion
    content.append(&divider2);
    
    // RIGHT: Additional Info (tighter spacing)
    let right_box = Box::new(Orientation::Vertical, 4);
    right_box.set_halign(gtk::Align::Start);
    right_box.set_valign(gtk::Align::Center);
    
    let modified_date = format_system_time(&file_info.modified);
    let modified_row = create_compact_row("Modified:", &modified_date);
    right_box.append(&modified_row);
    
    let location = file_info.path.parent().unwrap_or(&file_info.path).to_string_lossy();
    let truncated_location = if location.len() > 40 {
        format!("{}...", location.chars().take(37).collect::<String>())
    } else {
        location.to_string()
    };
    let location_row = create_compact_row("Location:", &truncated_location);
    right_box.append(&location_row);
    
    content.append(&right_box);
}

fn create_folder_details(content: &Box, folder_path: &PathBuf) {
    // LEFT: Folder Icon (70px - compact)
    let left_box = Box::new(Orientation::Vertical, 0);
    left_box.set_width_request(64);
    left_box.set_halign(gtk::Align::Center);
    left_box.set_valign(gtk::Align::Center);
    
    let icon_label = Label::new(Some("📁"));
    icon_label.add_css_class("details-file-icon-compact");
    icon_label.set_halign(gtk::Align::Center);
    icon_label.set_valign(gtk::Align::Center);
    left_box.append(&icon_label);
    content.append(&left_box);
    
    // DIVIDER
    let divider = Separator::new(Orientation::Vertical);
    divider.set_valign(gtk::Align::Center);
    divider.set_height_request(44);
    content.append(&divider);
    
    // MIDDLE: Folder Details (3 rows - tighter spacing)
    let middle_box = Box::new(Orientation::Vertical, 4);
    middle_box.set_hexpand(true);
    middle_box.set_valign(gtk::Align::Center);
    
    let folder_name = folder_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown Folder");
    
    // Row 1: Name
    let name_row = create_compact_row("Name:", folder_name);
    middle_box.append(&name_row);
    
    // Row 2: Item count
    let item_count = count_folder_items(folder_path);
    let items_text = format!("{} items", item_count);
    let items_row = create_compact_row("Items:", &items_text);
    middle_box.append(&items_row);
    
    // Row 3: Type
    let type_row = create_compact_row("Type:", "Folder");
    middle_box.append(&type_row);
    
    content.append(&middle_box);
    
    // DIVIDER
    let divider2 = Separator::new(Orientation::Vertical);
    divider2.set_valign(gtk::Align::Center);
    divider2.set_height_request(44);
    content.append(&divider2);
    
    // RIGHT: Additional Info (tighter spacing)
    let right_box = Box::new(Orientation::Vertical, 4);
    right_box.set_halign(gtk::Align::Start);
    right_box.set_valign(gtk::Align::Center);
    
    let location = folder_path.parent().unwrap_or(folder_path).to_string_lossy();
    let truncated_location = if location.len() > 40 {
        format!("{}...", location.chars().take(37).collect::<String>())
    } else {
        location.to_string()
    };
    let location_row = create_compact_row("Location:", &truncated_location);
    right_box.append(&location_row);
    
    // Get path
    let path_str = folder_path.to_string_lossy();
    let truncated_path = if path_str.len() > 40 {
        format!("{}...", path_str.chars().take(37).collect::<String>())
    } else {
        path_str.to_string()
    };
    let path_row = create_compact_row("Path:", &truncated_path);
    right_box.append(&path_row);
    
    content.append(&right_box);
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
    // Right-align labels for a neat column
    label_widget.set_halign(gtk::Align::End);
    
    let value_widget = Label::new(Some(value));
    value_widget.add_css_class("details-property-value");
    value_widget.set_halign(gtk::Align::Start);
    value_widget.set_wrap(true);
    value_widget.set_hexpand(true);
    
    row_box.append(&label_widget);
    row_box.append(&value_widget);
    parent.append(&row_box);
}

fn create_compact_row(label: &str, value: &str) -> Box {
    let row = Box::new(Orientation::Horizontal, 4);
    row.set_valign(gtk::Align::Center);
    row.set_halign(gtk::Align::Start);
    
    let label_widget = Label::new(Some(label));
    label_widget.add_css_class("details-label-compact");
    label_widget.set_halign(gtk::Align::Start);
    label_widget.set_valign(gtk::Align::Center);
    label_widget.set_width_request(70);
    
    let value_widget = Label::new(Some(value));
    value_widget.add_css_class("details-value-compact");
    value_widget.set_halign(gtk::Align::Start);
    value_widget.set_valign(gtk::Align::Center);
    value_widget.set_hexpand(true);
    value_widget.set_ellipsize(gtk::pango::EllipsizeMode::End);
    
    row.append(&label_widget);
    row.append(&value_widget);
    row
}

fn create_error_details(content: &Box, message: &str) {
    let error_label = Label::new(Some(message));
    error_label.add_css_class("details-error");
    error_label.set_halign(gtk::Align::Center);
    error_label.set_valign(gtk::Align::Center);
    content.append(&error_label);
}

fn create_no_selection_details(content: &Box) {
    let no_selection_label = Label::new(Some("No file selected"));
    no_selection_label.add_css_class("details-no-selection");
    no_selection_label.set_halign(gtk::Align::Center);
    no_selection_label.set_valign(gtk::Align::Center);
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
                    datetime.format("%Y-%m-%d").to_string()
                }
                chrono::LocalResult::Ambiguous(datetime, _) => {
                    datetime.format("%Y-%m-%d").to_string()
                }
                chrono::LocalResult::None => {
                    Local::now().format("%Y-%m-%d").to_string()
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
