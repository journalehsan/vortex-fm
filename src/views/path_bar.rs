use gtk::prelude::*;
use gtk::{Box, Orientation, Button, Separator, Label, SearchEntry};
use crate::core::file_manager::FileManagerState;

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
    
    // Disable buttons initially
    back_btn.set_sensitive(false);
    forward_btn.set_sensitive(false);
    up_btn.set_sensitive(false);
    
    path_bar.append(&back_btn);
    path_bar.append(&forward_btn);
    path_bar.append(&up_btn);
    path_bar.append(&refresh_btn);
    
    // Separator
    let separator = Separator::new(Orientation::Vertical);
    path_bar.append(&separator);
    
    // Path display
    let current_path_str = state.current_path.to_string_lossy().to_string();
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
    path_bar.append(&search_entry);
    
    path_bar
}
