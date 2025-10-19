use gtk::prelude::*;
use gtk::{Box, Orientation, Button, Label, Notebook};
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use crate::core::tab_manager::{TabManager, Tab};
use crate::core::navigation::navigate_to_directory;

pub fn create_tab_bar(tab_manager: Rc<RefCell<TabManager>>) -> Box {
    let tab_bar = Box::new(Orientation::Horizontal, 0);
    tab_bar.add_css_class("tab-bar");
    tab_bar.set_margin_start(4);
    tab_bar.set_margin_end(4);
    tab_bar.set_margin_top(4);
    tab_bar.set_margin_bottom(4);
    
    // New tab button
    let new_tab_btn = Button::with_label("+");
    new_tab_btn.add_css_class("tab-new-button");
    new_tab_btn.set_tooltip_text(Some("New Tab (Ctrl+T)"));
    new_tab_btn.set_width_request(32);
    new_tab_btn.set_height_request(32);
    
    let tab_manager_clone = tab_manager.clone();
    let tab_bar_clone = tab_bar.clone();
    new_tab_btn.connect_clicked(move |_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        let home_path = PathBuf::from(&home);
        tab_manager_clone.borrow_mut().add_tab(home_path);
        
        // Update the tab bar UI
        update_tab_bar(&tab_bar_clone, tab_manager_clone.borrow().clone());
    });
    
    tab_bar.append(&new_tab_btn);
    
    // Tab container
    let tabs_container = Box::new(Orientation::Horizontal, 0);
    tabs_container.add_css_class("tabs-container");
    tabs_container.set_hexpand(true);
    tab_bar.append(&tabs_container);
    
    // Store reference to tabs container for updates
    {
        let mut manager = tab_manager.borrow_mut();
        manager.notebook = Some(Notebook::new());
    }
    
    tab_bar
}

pub fn create_tab_widget(tab: &Tab, tab_manager: Rc<RefCell<TabManager>>) -> Box {
    let tab_widget = Box::new(Orientation::Horizontal, 4);
    tab_widget.add_css_class("tab-widget");
    
    if tab.is_active {
        tab_widget.add_css_class("active");
    }
    
    // Tab icon (folder icon)
    let icon_label = Label::new(Some("📁"));
    icon_label.add_css_class("tab-icon");
    
    // Tab title
    let title_label = Label::new(Some(&tab.title));
    title_label.add_css_class("tab-title");
    title_label.set_halign(gtk::Align::Start);
    title_label.set_hexpand(true);
    
    // Close button
    let close_btn = Button::with_label("×");
    close_btn.add_css_class("tab-close-button");
    close_btn.set_tooltip_text(Some("Close Tab"));
    close_btn.set_width_request(20);
    close_btn.set_height_request(20);
    
    let tab_id = tab.id;
    let tab_manager_clone = tab_manager.clone();
    close_btn.connect_clicked(move |_| {
        tab_manager_clone.borrow_mut().close_tab(tab_id);
        // TODO: Update tab bar UI - this would need the tab_bar reference
    });
    
    tab_widget.append(&icon_label);
    tab_widget.append(&title_label);
    tab_widget.append(&close_btn);
    
    // Connect tab click handler using gesture
    let tab_manager_clone = tab_manager.clone();
    let tab_id = tab.id;
    
    let gesture = gtk::GestureClick::new();
    gesture.connect_pressed(move |_, n_press, _x, _y| {
        if n_press == 1 {
            // Single click - switch to tab
            crate::utils::simple_debug::debug_info("TAB_BAR", &format!("Switching to tab {}", tab_id));
            crate::core::navigation::switch_to_tab(tab_id);
            // Update tab bar UI
            crate::widgets::tab_bar::update_global_tab_bar();
        } else if n_press == 2 {
            // Double click - also switch to tab (for consistency)
            crate::utils::simple_debug::debug_info("TAB_BAR", &format!("Double-click switching to tab {}", tab_id));
            crate::core::navigation::switch_to_tab(tab_id);
            // Update tab bar UI
            crate::widgets::tab_bar::update_global_tab_bar();
        }
    });
    tab_widget.add_controller(gesture);
    
    tab_widget
}

pub fn update_tab_bar(tab_bar: &Box, tab_manager: TabManager) {
    // Clear existing tabs
    if let Some(tabs_container) = tab_bar.last_child() {
        if let Some(container) = tabs_container.downcast_ref::<Box>() {
            // Remove all children except the new tab button
            while let Some(child) = container.first_child() {
                if child.css_classes().contains(&gtk::glib::GString::from("tab-widget")) {
                    container.remove(&child);
                } else {
                    break;
                }
            }
        }
    }
    
    // Add current tabs
    if let Some(tabs_container) = tab_bar.last_child() {
        if let Some(container) = tabs_container.downcast_ref::<Box>() {
            for tab in &tab_manager.tabs {
                let tab_widget = create_tab_widget(tab, Rc::new(RefCell::new(tab_manager.clone())));
                container.append(&tab_widget);
            }
        }
    }
}

// Global tab bar reference for updates
static mut GLOBAL_TAB_BAR: Option<Box> = None;

pub fn set_global_tab_bar(tab_bar: Box) {
    unsafe {
        GLOBAL_TAB_BAR = Some(tab_bar);
    }
}

pub fn update_global_tab_bar() {
    unsafe {
        if let Some(tab_bar) = &GLOBAL_TAB_BAR {
            if let Some(tab_manager_rc) = crate::core::navigation::get_global_tab_manager() {
                update_tab_bar(tab_bar, tab_manager_rc.borrow().clone());
            }
        }
    }
}
