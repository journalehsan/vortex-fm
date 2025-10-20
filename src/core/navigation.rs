use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use crate::core::file_manager::FileManagerState;
use crate::core::tab_manager::TabManager;

// Global state for navigation
static mut GLOBAL_STATE: Option<Rc<RefCell<FileManagerState>>> = None;
static mut GLOBAL_TAB_MANAGER: Option<Rc<RefCell<TabManager>>> = None;

pub fn set_global_state(state: Rc<RefCell<FileManagerState>>) {
    unsafe {
        GLOBAL_STATE = Some(state);
    }
}

pub fn get_global_state() -> Option<Rc<RefCell<FileManagerState>>> {
    unsafe {
        GLOBAL_STATE.as_ref().map(|state| state.clone())
    }
}

pub fn set_global_tab_manager(tab_manager: Rc<RefCell<TabManager>>) {
    unsafe {
        GLOBAL_TAB_MANAGER = Some(tab_manager);
    }
}

pub fn get_global_tab_manager() -> Option<Rc<RefCell<TabManager>>> {
    unsafe {
        GLOBAL_TAB_MANAGER.as_ref().map(|tab_manager| tab_manager.clone())
    }
}

pub fn navigate_to_directory(path: PathBuf) {
    crate::utils::simple_debug::debug_info("NAVIGATION", &format!("navigate_to_directory called with path: {}", path.display()));
    if let Some(tab_manager_rc) = get_global_tab_manager() {
        crate::utils::simple_debug::debug_info("NAVIGATION", "Navigating active tab");
        // Navigate the active tab
        tab_manager_rc.borrow_mut().navigate_active_tab_to(path.clone());
        
        crate::utils::simple_debug::debug_info("NAVIGATION", "Updating active tab title");
        // Update the tab title after navigation
        tab_manager_rc.borrow_mut().update_active_tab_title();
        
        // Get the updated navigation history from the active tab
        let navigation_history = {
            let tab_manager = tab_manager_rc.borrow();
            if let Some(active_tab) = tab_manager.get_active_tab() {
                Some(active_tab.navigation_history.clone())
            } else {
                None
            }
        };
        
        crate::utils::simple_debug::debug_info("NAVIGATION", "Updating global state");
        // Update the global state to match the active tab
        if let Some(nav_history) = navigation_history {
            if let Some(state_rc) = get_global_state() {
                let mut state = state_rc.borrow_mut();
                state.navigation_history = nav_history;
                crate::utils::simple_debug::debug_info("NAVIGATION", "Calling state.refresh_ui()");
                state.refresh_ui();
                crate::utils::simple_debug::debug_info("NAVIGATION", "state.refresh_ui() completed");
            } else {
                crate::utils::simple_debug::debug_info("NAVIGATION", "ERROR: Could not get global state!");
            }
        } else {
            crate::utils::simple_debug::debug_info("NAVIGATION", "ERROR: No navigation history available!");
        }
        
        crate::utils::simple_debug::debug_info("NAVIGATION", "Clearing selection");
        // Clear selection when navigating to a new directory
        crate::core::selection::clear_selection();
        
        // Update tab bar UI to reflect title changes
        crate::widgets::tab_bar::update_global_tab_bar();
        
        // Sync terminal directory
        crate::widgets::terminal_panel::sync_terminal_directory(&path);
    }
}

pub fn switch_to_tab(tab_id: usize) {
    crate::utils::simple_debug::debug_info("NAVIGATION", &format!("Switching to tab {}", tab_id));
    
    if let Some(tab_manager_rc) = get_global_tab_manager() {
        // Activate the tab and get its navigation history
        let navigation_history = {
            let mut tab_manager = tab_manager_rc.borrow_mut();
            tab_manager.activate_tab(tab_id);
            
            // Get the navigation history from the newly activated tab
            if let Some(active_tab) = tab_manager.get_active_tab() {
                Some(active_tab.navigation_history.clone())
            } else {
                None
            }
        };
        
        // Update the global state with the new tab's navigation history
        if let Some(nav_history) = navigation_history {
            if let Some(state_rc) = get_global_state() {
                let mut state = state_rc.borrow_mut();
                state.navigation_history = nav_history;
                state.refresh_ui();
                crate::utils::simple_debug::debug_info("NAVIGATION", &format!("Switched to tab {} with path: {}", tab_id, state.current_path().display()));
            }
        }
        
        // Clear selection when switching tabs
        crate::core::selection::clear_selection();
        
        // Update tab bar UI to reflect title changes
        crate::widgets::tab_bar::update_global_tab_bar();
        
        // Sync terminal directory
        if let Some(state_rc) = get_global_state() {
            crate::widgets::terminal_panel::sync_terminal_directory(state_rc.borrow().current_path());
        }
    }
}

pub fn open_in_new_tab(path: PathBuf) {
    if let Some(tab_manager_rc) = get_global_tab_manager() {
        // Create new tab
        let new_tab_id = tab_manager_rc.borrow_mut().add_tab(path.clone());
        
        // Switch to the new tab
        switch_to_tab(new_tab_id);
        
        // Update tab bar UI
        crate::widgets::tab_bar::update_global_tab_bar();
    }
}
