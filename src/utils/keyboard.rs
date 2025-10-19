use gtk::prelude::*;
use gtk::{ApplicationWindow, gio, gdk};
use glib::Propagation;

pub fn setup_keyboard_shortcuts(window: &ApplicationWindow) {
    // Create action group
    let action_group = gio::SimpleActionGroup::new();
    
    // Copy action (Ctrl+C)
    let copy_action = gio::SimpleAction::new("copy", None);
    copy_action.connect_activate(|_, _| {
        println!("📋 Copy action triggered (Ctrl+C)");
        // TODO: Implement copy functionality
    });
    action_group.add_action(&copy_action);
    
    // Cut action (Ctrl+X)
    let cut_action = gio::SimpleAction::new("cut", None);
    cut_action.connect_activate(|_, _| {
        println!("✂️ Cut action triggered (Ctrl+X)");
        // TODO: Implement cut functionality
    });
    action_group.add_action(&cut_action);
    
    // Paste action (Ctrl+V)
    let paste_action = gio::SimpleAction::new("paste", None);
    paste_action.connect_activate(|_, _| {
        println!("📋 Paste action triggered (Ctrl+V)");
        // TODO: Implement paste functionality
    });
    action_group.add_action(&paste_action);
    
    // Delete action (Delete key)
    let delete_action = gio::SimpleAction::new("delete", None);
    delete_action.connect_activate(|_, _| {
        println!("🗑️ Delete action triggered (Delete)");
        // TODO: Implement delete functionality
    });
    action_group.add_action(&delete_action);
    
    // Rename action (F2)
    let rename_action = gio::SimpleAction::new("rename", None);
    rename_action.connect_activate(|_, _| {
        println!("✏️ Rename action triggered (F2)");
        // TODO: Implement rename functionality
    });
    action_group.add_action(&rename_action);
    
    // Refresh action (F5)
    let refresh_action = gio::SimpleAction::new("refresh", None);
    refresh_action.connect_activate(|_, _| {
        println!("🔄 Refresh action triggered (F5)");
        // TODO: Implement refresh functionality
    });
    action_group.add_action(&refresh_action);
    
    // Add action group to window
    window.insert_action_group("file", Some(&action_group));
    
    // Create application actions
    let app = window.application().unwrap();
    
    // New folder action (Ctrl+Shift+N)
    let new_folder_action = gio::SimpleAction::new("new-folder", None);
    new_folder_action.connect_activate(|_, _| {
        println!("📁 New folder action triggered (Ctrl+Shift+N)");
        // TODO: Implement new folder functionality
    });
    app.add_action(&new_folder_action);
    
    // Terminal toggle action (F4)
    let terminal_toggle_action = gio::SimpleAction::new("toggle-terminal", None);
    terminal_toggle_action.connect_activate(|_, _| {
        crate::utils::simple_debug::debug_info("KEYBOARD", "Terminal toggle action triggered (F4)");
        crate::widgets::terminal_panel::toggle_terminal_panel(); // Call the toggle function
    });
    app.add_action(&terminal_toggle_action);
    
    // Add key controller for F4
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(|_, key, _, _| {
        if *key == gdk::Key::F4 {
            crate::utils::simple_debug::debug_info("KEYBOARD", "F4 key pressed - toggling terminal");
            crate::widgets::terminal_panel::toggle_terminal_panel();
            Propagation::Stop
        } else {
            Propagation::Continue
        }
    });
    window.add_controller(key_controller);
}
