use gtk::prelude::*;
use gtk::{Dialog, Label, Box, Orientation, Button, Separator, Image, Window};
use std::path::PathBuf;
use std::fs;
use crate::utils::file_ops::get_file_info;

pub fn show_properties_dialog(path: &PathBuf) {
    let dialog = Dialog::new();
    dialog.set_title(Some("Properties"));
    dialog.set_default_size(400, 500);
    dialog.set_modal(true);
    
    // Get file information
    let file_info = match get_file_info(path) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error getting file info: {}", e);
            return;
        }
    };
    
    // Create content area
    let content = dialog.content_area();
    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    
    // File icon and name
    let header_box = Box::new(Orientation::Horizontal, 12);
    
    let icon_label = Label::new(Some(&get_file_icon(&file_info.file_type)));
    icon_label.add_css_class("file-icon-large");
    icon_label.set_css_classes(&["file-icon-large"]);
    
    let name_label = Label::new(Some(&file_info.name));
    name_label.add_css_class("file-name-large");
    name_label.set_halign(gtk::Align::Start);
    name_label.set_hexpand(true);
    
    header_box.append(&icon_label);
    header_box.append(&name_label);
    vbox.append(&header_box);
    
    // Separator
    let separator1 = Separator::new(Orientation::Horizontal);
    vbox.append(&separator1);
    
    // Properties section
    let properties_label = Label::new(Some("Properties"));
    properties_label.add_css_class("title-4");
    vbox.append(&properties_label);
    
    // File type
    let type_box = create_property_row("Type:", &file_info.file_type);
    vbox.append(&type_box);
    
    // Size
    let size_text = if file_info.file_type == "Directory" {
        "Directory".to_string()
    } else {
        format_size(file_info.size)
    };
    let size_box = create_property_row("Size:", &size_text);
    vbox.append(&size_box);
    
    // Path
    let path_text = path.to_string_lossy().to_string();
    let path_box = create_property_row("Location:", &path_text);
    vbox.append(&path_box);
    
    // Modified date
    let modified_text = format_system_time(file_info.modified);
    let modified_box = create_property_row("Modified:", &modified_text);
    vbox.append(&modified_box);
    
    // Permissions (if available)
    if let Ok(metadata) = fs::metadata(path) {
        let permissions = format_permissions(&metadata);
        let perms_box = create_property_row("Permissions:", &permissions);
        vbox.append(&perms_box);
    }
    
    // Separator
    let separator2 = Separator::new(Orientation::Horizontal);
    vbox.append(&separator2);
    
    // Buttons
    let button_box = Box::new(Orientation::Horizontal, 8);
    button_box.set_halign(gtk::Align::End);
    
    let close_btn = Button::with_label("Close");
    let dialog_clone = dialog.clone();
    close_btn.connect_clicked(move |_| {
        dialog_clone.close();
    });
    
    button_box.append(&close_btn);
    vbox.append(&button_box);
    
    content.append(&vbox);
    
    // Show dialog
    dialog.present();
}

fn create_property_row(label: &str, value: &str) -> Box {
    let row = Box::new(Orientation::Horizontal, 8);
    
    let label_widget = Label::new(Some(label));
    label_widget.set_width_request(100);
    label_widget.set_halign(gtk::Align::Start);
    label_widget.add_css_class("property-label");
    
    let value_widget = Label::new(Some(value));
    value_widget.set_halign(gtk::Align::Start);
    value_widget.set_hexpand(true);
    value_widget.set_wrap(true);
    value_widget.add_css_class("property-value");
    
    row.append(&label_widget);
    row.append(&value_widget);
    
    row
}

fn get_file_icon(file_type: &str) -> &'static str {
    match file_type {
        "Directory" => "📁",
        "Text File" => "📄",
        "Image File" => "🖼️",
        "Audio File" => "🎵",
        "Video File" => "🎬",
        "Archive File" => "📦",
        "Script File" => "💻",
        "PDF File" => "📕",
        "Document File" => "📘",
        "Spreadsheet File" => "📊",
        "Presentation File" => "📽️",
        _ => "📄",
    }
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

fn format_system_time(time: std::time::SystemTime) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let timestamp = duration.as_secs();
    
    // Convert to readable date format
    let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_default();
    
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn format_permissions(metadata: &fs::Metadata) -> String {
    use std::os::unix::fs::PermissionsExt;
    
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    
    let owner_read = if mode & 0o400 != 0 { "r" } else { "-" };
    let owner_write = if mode & 0o200 != 0 { "w" } else { "-" };
    let owner_execute = if mode & 0o100 != 0 { "x" } else { "-" };
    
    let group_read = if mode & 0o040 != 0 { "r" } else { "-" };
    let group_write = if mode & 0o020 != 0 { "w" } else { "-" };
    let group_execute = if mode & 0o010 != 0 { "x" } else { "-" };
    
    let other_read = if mode & 0o004 != 0 { "r" } else { "-" };
    let other_write = if mode & 0o002 != 0 { "w" } else { "-" };
    let other_execute = if mode & 0o001 != 0 { "x" } else { "-" };
    
    format!("{}{}{}{}{}{}{}{}{}", 
        owner_read, owner_write, owner_execute,
        group_read, group_write, group_execute,
        other_read, other_write, other_execute)
}
