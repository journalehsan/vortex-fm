use gtk::prelude::*;
use gtk::{ApplicationWindow, gio, gdk};

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
    
    // Add key controller for global shortcuts
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(|_, key, keycode, state| {
        use gdk::Key;
        let ctrl = state.contains(gdk::ModifierType::CONTROL_MASK);
        let shift = state.contains(gdk::ModifierType::SHIFT_MASK);

        // F4: toggle terminal
        if key == Key::F4 {
            crate::utils::simple_debug::debug_info("KEYBOARD", "F4 key pressed - toggling terminal");
            crate::widgets::terminal_panel::toggle_terminal_panel();
            return gtk::glib::Propagation::Stop;
        }

        // Ctrl+W: close current tab (defer to idle to avoid borrow conflicts)
        if ctrl && (key == Key::w || key == Key::W) {
            gtk::glib::idle_add_once(move || {
                if let Some(tab_manager_rc) = crate::core::navigation::get_global_tab_manager() {
                    // Snapshot active id in a read borrow scope
                    let active_id_opt = { tab_manager_rc.borrow().active_tab_id };
                    if let Some(active_id) = active_id_opt {
                        // Close in a write borrow scope
                        let _closed = tab_manager_rc.borrow_mut().close_tab(active_id);
                        // Read next path in a separate read borrow scope
                        let next_path_opt = { tab_manager_rc.borrow().get_active_path() };
                        crate::widgets::tab_bar::update_global_tab_bar();
                        if let Some(path) = next_path_opt {
                            crate::core::navigation::navigate_to_directory(path);
                        }
                    }
                }
            });
            return gtk::glib::Propagation::Stop;
        }

        // Ctrl+Tab: next tab; Ctrl+Shift+Tab: previous tab
        if ctrl && (key == Key::Tab) {
            if let Some(tab_manager_rc) = crate::core::navigation::get_global_tab_manager() {
                let tabs = tab_manager_rc.borrow().tabs.clone();
                if tabs.is_empty() { return gtk::glib::Propagation::Proceed; }
                if let Some(current_id) = tab_manager_rc.borrow().active_tab_id {
                    let mut idx = tabs.iter().position(|t| t.id == current_id).unwrap_or(0) as isize;
                    if shift { idx -= 1; } else { idx += 1; }
                    let len = tabs.len() as isize;
                    let idx = (idx.rem_euclid(len)) as usize;
                    let next_id = tabs[idx].id;
                    crate::core::navigation::switch_to_tab(next_id);
                    crate::widgets::tab_bar::update_global_tab_bar();
                    return gtk::glib::Propagation::Stop;
                }
            }
        }

        gtk::glib::Propagation::Proceed
    });
    window.add_controller(key_controller);
}
