use gtk::prelude::*;
use gtk::{Box, Orientation, Button, Separator, Label, SearchEntry};
use crate::core::file_manager::FileManagerState;
use crate::core::navigation::get_global_state;

pub fn create_path_bar(state: &mut FileManagerState) -> Box {
    let path_bar = Box::new(Orientation::Horizontal, 8);
    path_bar.add_css_class("toolbar");
    path_bar.set_margin_start(8);
    path_bar.set_margin_end(8);
    path_bar.set_margin_top(8);
    path_bar.set_margin_bottom(8);
    
    // Navigation buttons
    let back_btn = Button::from_icon_name("go-previous-symbolic");
    let forward_btn = Button::from_icon_name("go-next-symbolic");
    let up_btn = Button::from_icon_name("go-up-symbolic");
    let refresh_btn = Button::from_icon_name("view-refresh-symbolic");
    
    // Set initial button states
    back_btn.set_sensitive(state.can_go_back());
    forward_btn.set_sensitive(state.can_go_forward());
    up_btn.set_sensitive(state.can_go_up());
    
    // Connect navigation button handlers
    back_btn.connect_clicked(move |_| {
        if let Some(state_rc) = get_global_state() {
            state_rc.borrow_mut().go_back();
        }
    });
    
    forward_btn.connect_clicked(move |_| {
        if let Some(state_rc) = get_global_state() {
            state_rc.borrow_mut().go_forward();
        }
    });
    
    up_btn.connect_clicked(move |_| {
        if let Some(state_rc) = get_global_state() {
            state_rc.borrow_mut().go_up();
        }
    });
    
    refresh_btn.connect_clicked(move |_| {
        if let Some(state_rc) = get_global_state() {
            state_rc.borrow().refresh_ui();
        }
    });
    
    path_bar.append(&back_btn);
    path_bar.append(&forward_btn);
    path_bar.append(&up_btn);
    path_bar.append(&refresh_btn);
    
    // Store button references in state
    state.back_button = Some(back_btn.clone());
    state.forward_button = Some(forward_btn.clone());
    state.up_button = Some(up_btn.clone());
    
    // Separator
    let separator = Separator::new(Orientation::Vertical);
    path_bar.append(&separator);
    
    // Path display
    let current_path_str = state.current_path().to_string_lossy().to_string();
    let path_label = Label::new(Some(&current_path_str));
    path_label.set_halign(gtk::Align::Start);
    path_label.set_hexpand(true);
    path_label.add_css_class("path-label");
    path_bar.append(&path_label);
    
    // Store reference for later updates
    state.path_label = Some(path_label.clone());
    
    // Search box
    let search_entry = SearchEntry::new();
    search_entry.set_placeholder_text(Some("Search files..."));
    search_entry.set_width_request(200);
    
    // Connect search functionality
    search_entry.connect_search_changed(move |entry| {
        if let Some(state_rc) = get_global_state() {
            let filter_text = entry.text().to_string();
            state_rc.borrow_mut().set_filter(filter_text);
            state_rc.borrow().refresh_ui();
        }
    });
    
    path_bar.append(&search_entry);
    
    path_bar
}

