use gtk::prelude::*;
use gtk::{Button, Box, Orientation, Label};
use std::path::PathBuf;
use crate::core::config::VortexConfig;
use crate::core::navigation::navigate_to_directory;

pub fn create_file_item(icon: &str, name: &str, _file_type: &str, path: PathBuf, config: &VortexConfig) -> Button {
    let item_box = Box::new(Orientation::Vertical, 4);
    item_box.set_width_request(80);
    item_box.set_height_request(80);
    item_box.add_css_class("file-item");
    
    // File icon
    let icon_label = Label::new(Some(icon));
    icon_label.add_css_class("file-icon");
    icon_label.set_halign(gtk::Align::Center);
    item_box.append(&icon_label);
    
    // File name
    let name_label = Label::new(Some(name));
    name_label.add_css_class("file-name");
    name_label.set_halign(gtk::Align::Center);
    name_label.set_wrap(true);
    name_label.set_max_width_chars(10);
    item_box.append(&name_label);
    
    // Make it clickable
    let button = Button::new();
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
