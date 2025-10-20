use gtk::prelude::*;
use gtk::{PopoverMenu, gio};
use std::path::PathBuf;
use crate::core::navigation::navigate_to_directory;
use crate::core::bookmarks::{Bookmark, get_global_bookmarks_manager};
use crate::utils::file_ops::open_with_system;
use crate::widgets::properties_dialog::show_properties_dialog;
use crate::widgets::file_operations_dialog::{show_rename_dialog, show_delete_confirmation, show_copy_dialog, show_move_dialog};

pub fn create_folder_context_menu(path: PathBuf) -> PopoverMenu {
    let menu = gio::Menu::new();
    
    // Open action
    let open_action = gio::SimpleAction::new("open", None);
    let path_clone = path.clone();
    open_action.connect_activate(move |_, _| {
        println!("📁 Opening folder: {}", path_clone.display());
        navigate_to_directory(path_clone.clone());
    });
    
    // Open in new tab action
    let new_tab_action = gio::SimpleAction::new("open-new-tab", None);
    let path_clone = path.clone();
    new_tab_action.connect_activate(move |_, _| {
        println!("📁 Opening folder in new tab: {}", path_clone.display());
        crate::core::navigation::open_in_new_tab(path_clone.clone());
    });
    
    // Add to Quick Access action
    let quick_access_action = gio::SimpleAction::new("add-quick-access", None);
    let path_clone = path.clone();
    quick_access_action.connect_activate(move |_, _| {
        println!("⭐ Adding folder to Quick Access: {}", path_clone.display());
        if let Some(manager_rc) = get_global_bookmarks_manager() {
            println!("   ✓ Got global bookmarks manager");
            let folder_name = path_clone.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Folder")
                .to_string();
            println!("   ✓ Folder name: {}", folder_name);
            let bookmark = Bookmark::new(
                folder_name.clone(),
                path_clone.clone(),
                "📁".to_string(),
                "Quick Access".to_string(),
            );
            println!("   ✓ Bookmark created: {} -> {}", bookmark.name, bookmark.path.display());
            manager_rc.borrow_mut().add_bookmark(bookmark.clone());
            println!("   ✓ Bookmark added to manager");
            let save_result = manager_rc.borrow().save();
            println!("   ✓ Bookmark saved: {:?}", save_result);
            // Add directly to the UI
            println!("   → Calling add_bookmark_to_qa_ui...");
            crate::widgets::modern_sidebar::add_bookmark_to_qa_ui(&bookmark);
            println!("   ✓ add_bookmark_to_qa_ui completed");
        } else {
            println!("   ✗ Failed to get global bookmarks manager!");
        }
    });
    
    // Properties action
    let properties_action = gio::SimpleAction::new("properties", None);
    let path_clone = path.clone();
    properties_action.connect_activate(move |_, _| {
        println!("📁 Showing properties for: {}", path_clone.display());
        show_properties_dialog(&path_clone);
    });
    
    // Add menu items
    menu.append(Some("Open"), Some("folder.open"));
    menu.append(Some("Open in New Tab"), Some("folder.open-new-tab"));
    menu.append(Some("Add to Quick Access"), Some("folder.add-quick-access"));
    menu.append(Some("Properties"), Some("folder.properties"));
    
    let popover = PopoverMenu::from_model(Some(&menu));
    popover.set_menu_model(Some(&menu));
    
    // Create action group for this popover
    let action_group = gio::SimpleActionGroup::new();
    action_group.add_action(&open_action);
    action_group.add_action(&new_tab_action);
    action_group.add_action(&quick_access_action);
    action_group.add_action(&properties_action);
    
    popover.insert_action_group("folder", Some(&action_group));
    
    popover
}

pub fn create_file_context_menu(path: PathBuf) -> PopoverMenu {
    let menu = gio::Menu::new();
    
    // Open with system action
    let open_action = gio::SimpleAction::new("open", None);
    let path_clone = path.clone();
    open_action.connect_activate(move |_, _| {
        println!("📄 Opening file: {}", path_clone.display());
        if let Err(e) = open_with_system(&path_clone) {
            eprintln!("Error opening file: {}", e);
        }
    });
    
    // Copy action
    let copy_action = gio::SimpleAction::new("copy", None);
    let path_clone = path.clone();
    copy_action.connect_activate(move |_, _| {
        println!("📋 Copying file: {}", path_clone.display());
        show_copy_dialog(&path_clone);
    });
    
    // Cut action
    let cut_action = gio::SimpleAction::new("cut", None);
    let path_clone = path.clone();
    cut_action.connect_activate(move |_, _| {
        println!("✂️ Cutting file: {}", path_clone.display());
        show_move_dialog(&path_clone);
    });
    
    // Rename action
    let rename_action = gio::SimpleAction::new("rename", None);
    let path_clone = path.clone();
    rename_action.connect_activate(move |_, _| {
        println!("✏️ Renaming file: {}", path_clone.display());
        show_rename_dialog(&path_clone);
    });
    
    // Delete action
    let delete_action = gio::SimpleAction::new("delete", None);
    let path_clone = path.clone();
    delete_action.connect_activate(move |_, _| {
        println!("🗑️ Deleting file: {}", path_clone.display());
        show_delete_confirmation(&path_clone);
    });
    
    // Properties action
    let properties_action = gio::SimpleAction::new("properties", None);
    let path_clone = path.clone();
    properties_action.connect_activate(move |_, _| {
        println!("📄 Showing properties for: {}", path_clone.display());
        show_properties_dialog(&path_clone);
    });
    
    // Add menu items
    menu.append(Some("Open"), Some("file.open"));
    menu.append(Some("Copy"), Some("file.copy"));
    menu.append(Some("Cut"), Some("file.cut"));
    menu.append(Some("Rename"), Some("file.rename"));
    menu.append(Some("Delete"), Some("file.delete"));
    menu.append(Some("Properties"), Some("file.properties"));
    
    let popover = PopoverMenu::from_model(Some(&menu));
    popover.set_menu_model(Some(&menu));
    
    // Create action group for this popover
    let action_group = gio::SimpleActionGroup::new();
    action_group.add_action(&open_action);
    action_group.add_action(&copy_action);
    action_group.add_action(&cut_action);
    action_group.add_action(&rename_action);
    action_group.add_action(&delete_action);
    action_group.add_action(&properties_action);
    
    popover.insert_action_group("file", Some(&action_group));
    
    popover
}

// Properties dialog is now implemented in properties_dialog.rs

// File operations dialogs are now implemented in file_operations_dialog.rs
