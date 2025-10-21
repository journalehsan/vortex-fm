use gtk::prelude::*;
use gtk::{Button, Box, Orientation, Label, GestureClick, Picture, DragSource, gdk, Image, CheckButton};
use std::path::PathBuf;
use crate::core::config::VortexConfig;
use crate::core::navigation::navigate_to_directory;
use crate::core::selection::{select_file, clear_selection, get_global_selection_manager};
use crate::core::bookmarks::{Bookmark, get_global_bookmarks_manager};
use crate::widgets::context_menu::{create_folder_context_menu, create_file_context_menu};
use crate::utils::thumbnails::get_global_thumbnail_manager;
use crate::utils::icon_manager::get_global_icon_manager;

pub fn create_file_item(icon: &str, name: &str, _file_type: &str, path: PathBuf, config: &VortexConfig) -> Button {
    create_file_item_with_size(icon, name, _file_type, path, config, 64)
}

pub fn create_file_item_with_size(icon: &str, name: &str, _file_type: &str, path: PathBuf, config: &VortexConfig, icon_size: i32) -> Button {
    // Calculate dynamic tile dimensions based on icon size
    let base_width = 120;
    let base_height = 100;
    
    // Scale tile size based on icon size (16px = 0.5x, 256px = 2x)
    let scale_factor = (icon_size as f32 / 64.0).max(0.5).min(2.0);
    let item_width = (base_width as f32 * scale_factor) as i32;
    let item_height = (base_height as f32 * scale_factor) as i32;
    
    let item_box = Box::new(Orientation::Vertical, 4);
    item_box.set_width_request(item_width);
    item_box.set_height_request(item_height);
    item_box.set_size_request(item_width, item_height); // Force the size
    item_box.add_css_class("file-item");
    item_box.add_css_class("grid-item");
    
    // Check if this file is currently selected
    if let Some(selection_manager_rc) = get_global_selection_manager() {
        if selection_manager_rc.borrow().is_selected(&path) {
            item_box.add_css_class("selected");
        }
    }
    
    // Icon container with proper centering
    let icon_container = Box::new(Orientation::Vertical, 0);
    icon_container.set_halign(gtk::Align::Center);
    icon_container.set_valign(gtk::Align::Center);
    icon_container.set_hexpand(true);
    icon_container.set_vexpand(true);
    
    // Use system icons instead of emojis
    let icon_widget = {
        let manager = get_global_icon_manager().lock().unwrap();
        manager.create_icon_widget(&path, icon_size)
    };
    icon_widget.set_halign(gtk::Align::Center);
    icon_widget.set_valign(gtk::Align::Center);
    icon_container.append(&icon_widget);
    
    item_box.append(&icon_container);
    
    // File name with specific character limits based on icon size
    let max_chars = match icon_size {
        16..=24 => 8i32,   // Small icons: 8 characters max
        25..=32 => 10i32,  // Medium icons: 10 characters max
        33..=48 => 12i32,  // Large icons: 12 characters max
        49..=64 => 14i32,  // Extra large icons: 14 characters max
        _ => 10i32,        // Default: 10 characters
    };
    
    // Debug logging for filename truncation
    let name_length = name.chars().count();
    let will_truncate = name_length > max_chars as usize;
    let has_spaces = name.contains(' ');
    crate::utils::simple_debug::debug_info("FILE_ITEM", &format!(
        "Filename: '{}', Length: {} chars, Max: {} chars, Icon size: {}px, Will truncate: {}, Has spaces: {}",
        name, name_length, max_chars, icon_size, will_truncate, has_spaces
    ));
    
        // Handle long filenames without spaces by adding word breaks
        let display_name = if name.len() > max_chars as usize && !name.contains(' ') {
            // For filenames without spaces, insert soft hyphens every few characters
            let mut result = String::new();
            let chars: Vec<char> = name.chars().collect();
            let break_interval = (max_chars as usize / 2).max(3); // Break every 3-7 chars depending on max_chars
            
            for (i, ch) in chars.iter().enumerate() {
                if i > 0 && i % break_interval == 0 {
                    result.push('\u{200B}'); // Zero-width space for word break
                }
                result.push(*ch);
            }
            
            // Debug logging for word breaks
            crate::utils::simple_debug::debug_info("FILE_ITEM", &format!(
                "Added word breaks to '{}' -> '{}' (break interval: {})",
                name, result.replace('\u{200B}', "|"), break_interval
            ));
            
            result
        } else {
            name.to_string()
        };
        
        let name_label = Label::new(Some(&display_name));
        name_label.add_css_class("file-name");
        name_label.set_halign(gtk::Align::Center);
        name_label.set_wrap(true);
        name_label.set_wrap_mode(gtk::pango::WrapMode::Word);
        name_label.set_max_width_chars(max_chars);
        name_label.set_ellipsize(gtk::pango::EllipsizeMode::End); // Ellipsis at the end
        name_label.set_lines(2);
        name_label.set_vexpand(false); // Don't expand vertically
        name_label.set_hexpand(false); // Don't expand horizontally
    
    // Add tooltip with full filename
    name_label.set_tooltip_text(Some(name));
    
    item_box.append(&name_label);
    
    // Make it clickable
    let button = Button::new();
    button.set_child(Some(&item_box));
    button.set_size_request(item_width, item_height); // Force button size too
    button.add_css_class("file-button");
    
    // Add tooltip to the entire button with full filename
    button.set_tooltip_text(Some(name));
    
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
    
    // Connect click handler with multi-selection support
    let path_clone = path.clone();
    let single_click_mode = config.single_click_to_open;
    
    // Use GestureClick to handle both single and double clicks
    let gesture = gtk::GestureClick::new();
    gesture.set_button(1); // Left mouse button
    
    gesture.connect_pressed(move |gesture, n_press, _x, _y| {
        if gesture.current_button() == 1 {
            let modifiers = gesture.current_event_state();
            let ctrl_pressed = modifiers.contains(gdk::ModifierType::CONTROL_MASK);
            let shift_pressed = modifiers.contains(gdk::ModifierType::SHIFT_MASK);
            
            if path_clone.is_dir() {
                // For folders: single click opens if config allows
                if n_press == 1 && single_click_mode && !ctrl_pressed && !shift_pressed {
                    println!("📁 Opening directory: {}", path_clone.display());
                    navigate_to_directory(path_clone.clone());
                }
                // If double-click mode, open on double-click
                else if n_press == 2 && !single_click_mode && !ctrl_pressed && !shift_pressed {
                    println!("📁 Opening directory (double-click): {}", path_clone.display());
                    navigate_to_directory(path_clone.clone());
                }
                // Handle selection
                if n_press == 1 {
                    if ctrl_pressed {
                        // Ctrl+click: toggle selection
                        if let Some(selection_manager_rc) = get_global_selection_manager() {
                            let mut selection_manager = selection_manager_rc.borrow_mut();
                            if selection_manager.is_selected(&path_clone) {
                                selection_manager.deselect_file(&path_clone);
                            } else {
                                selection_manager.select_file(path_clone.clone());
                            }
                        }
                    } else if shift_pressed {
                        // Shift+click: range selection (implement range selection logic)
                        if let Some(selection_manager_rc) = get_global_selection_manager() {
                            // For now, just select this item
                            selection_manager_rc.borrow_mut().select_file(path_clone.clone());
                        }
                    } else {
                        // Regular click: select this item
                        select_file(path_clone.clone());
                        println!("📁 Selected folder: {}", path_clone.display());
                    }
                }
            } else {
                // For files: handle selection and opening
                if n_press == 1 {
                    if ctrl_pressed {
                        // Ctrl+click: toggle selection
                        if let Some(selection_manager_rc) = get_global_selection_manager() {
                            let mut selection_manager = selection_manager_rc.borrow_mut();
                            if selection_manager.is_selected(&path_clone) {
                                selection_manager.deselect_file(&path_clone);
                            } else {
                                selection_manager.select_file(path_clone.clone());
                            }
                        }
                    } else if shift_pressed {
                        // Shift+click: range selection
                        if let Some(selection_manager_rc) = get_global_selection_manager() {
                            selection_manager_rc.borrow_mut().select_file(path_clone.clone());
                        }
                    } else {
                        // Regular click: select the file
                        select_file(path_clone.clone());
                        println!("📄 Selected file: {}", path_clone.display());
                    }
                } else if n_press == 2 {
                    // Double click - open the file
                    println!("📄 Opening file (double-click): {}", path_clone.display());
                    if let Err(e) = crate::utils::file_ops::open_with_system(&path_clone) {
                        eprintln!("Error opening file: {}", e);
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
