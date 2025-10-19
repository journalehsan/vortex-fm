use gtk::prelude::*;
use gtk::{Button, Box, Orientation, Label, GestureClick};
use std::path::PathBuf;
use crate::core::config::VortexConfig;
use crate::core::navigation::navigate_to_directory;
use crate::widgets::context_menu::{create_folder_context_menu, create_file_context_menu};

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
    
    // Set up right-click gesture for context menu
    let gesture = GestureClick::new();
    gesture.set_button(3); // Right mouse button
    let button_clone = button.clone();
    let path_for_context = path.clone();
    gesture.connect_pressed(move |_gesture, _n_press, _x, _y| {
        let context_menu = if path_for_context.is_dir() {
            create_folder_context_menu(path_for_context.clone())
        } else {
            create_file_context_menu(path_for_context.clone())
        };
        context_menu.set_parent(&button_clone);
        context_menu.popup();
    });
    button.add_controller(gesture);
    
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
