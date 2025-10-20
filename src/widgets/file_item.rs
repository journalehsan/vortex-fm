use gtk::prelude::*;
use gtk::{Button, Box, Orientation, Label, GestureClick, Picture, DragSource, gdk};
use std::path::PathBuf;
use crate::core::config::VortexConfig;
use crate::core::navigation::navigate_to_directory;
use crate::core::selection::{select_file, clear_selection, get_global_selection_manager};
use crate::core::bookmarks::{Bookmark, get_global_bookmarks_manager};
use crate::widgets::context_menu::{create_folder_context_menu, create_file_context_menu};
use crate::utils::thumbnails::get_global_thumbnail_manager;

pub fn create_file_item(icon: &str, name: &str, _file_type: &str, path: PathBuf, config: &VortexConfig) -> Button {
    let item_box = Box::new(Orientation::Vertical, 4);
    item_box.set_width_request(80);
    item_box.set_height_request(80);
    item_box.add_css_class("file-item");
    
    // Check if this file is currently selected
    if let Some(selection_manager_rc) = get_global_selection_manager() {
        if selection_manager_rc.borrow().is_selected(&path) {
            item_box.add_css_class("selected");
        }
    }
    
    // Icon or thumbnail
    let thumbnail_manager = get_global_thumbnail_manager();
    let is_image = thumbnail_manager.is_image_file(&path);
    
    if is_image {
        // Show thumbnail for images
        if let Some(thumbnail_path) = thumbnail_manager.get_thumbnail_or_placeholder(&path) {
            let picture = Picture::for_filename(&thumbnail_path);
            picture.add_css_class("file-thumbnail");
            picture.set_width_request(32);
            picture.set_height_request(32);
            picture.set_can_shrink(false);
            item_box.append(&picture);
        } else {
            // Fallback to emoji icon
            let icon_label = Label::new(Some(icon));
            icon_label.add_css_class("file-icon");
            icon_label.set_halign(gtk::Align::Center);
            item_box.append(&icon_label);
        }
    } else {
        // File icon (using emoji for now, could be enhanced with real icons)
        let icon_label = Label::new(Some(icon));
        icon_label.add_css_class("file-icon");
        icon_label.set_halign(gtk::Align::Center);
        item_box.append(&icon_label);
    }
    
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
    
    // Connect click handler with double-click detection
    let _name_clone = name.to_string();
    let path_clone = path.clone();
    let single_click_mode = config.single_click_to_open;
    
    // Use GestureClick to handle both single and double clicks
    let gesture = gtk::GestureClick::new();
    gesture.set_button(1); // Left mouse button
    
    gesture.connect_pressed(move |gesture, n_press, _x, _y| {
        if gesture.current_button() == 1 {
            if path_clone.is_dir() {
                // For folders: always open on single click (regardless of config)
                if n_press == 1 {
                    println!("📁 Opening directory: {}", path_clone.display());
                    navigate_to_directory(path_clone.clone());
                }
            } else {
                // For files: behavior depends on single_click_to_open config
                if single_click_mode {
                    // Single-click mode: single click opens the file
                    if n_press == 1 {
                        println!("📄 Opening file (single-click mode): {}", path_clone.display());
                        // TODO: Open file with default application
                    }
                } else {
                    // Double-click mode: single click selects, double click opens
                    if n_press == 1 {
                        // Single click - select the file
                        select_file(path_clone.clone());
                        println!("📄 Selected file: {}", path_clone.display());
                    } else if n_press == 2 {
                        // Double click - open the file
                        println!("📄 Opening file (double-click mode): {}", path_clone.display());
                        // TODO: Open file with default application
                    }
                }
            }
        }
    });
    
    button.add_controller(gesture);
    
    // Add drag source for folders (for drag-and-drop to Quick Access)
    if path.is_dir() {
        let drag_source = DragSource::new();
        drag_source.set_actions(gdk::DragAction::COPY);
        
        let path_clone_drag = path.clone();
        drag_source.connect_prepare(move |_source, _x, _y| {
            let content = gdk::ContentProvider::for_value(&path_clone_drag.to_string_lossy().to_string().to_value());
            Some(content)
        });
        
        button.add_controller(drag_source);
    }
    
    // Return the button
    button
}
