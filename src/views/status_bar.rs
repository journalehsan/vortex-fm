use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Button};
use crate::core::file_manager::{FileManagerState, ViewMode};
use crate::views::content_area::{switch_view_to_list, switch_view_to_grid};

pub fn create_status_bar(state: &FileManagerState) -> Box {
    let status_bar = Box::new(Orientation::Horizontal, 12);
    status_bar.add_css_class("toolbar");
    status_bar.add_css_class("status-bar");
    status_bar.set_margin_start(4);
    status_bar.set_margin_end(4);
    status_bar.set_margin_top(4);
    status_bar.set_margin_bottom(4);
    
    let status_label = Label::new(Some("Ready"));
    status_label.set_halign(gtk::Align::Start);
    status_label.set_hexpand(true);
    status_bar.append(&status_label);
    
    // Count actual items in directory
    let item_count = if let Ok(entries) = std::fs::read_dir(state.current_path()) {
        if state.config.show_hidden_files {
            entries.count()
        } else {
            entries.filter(|entry| {
                entry.as_ref()
                    .map(|e| e.file_name().to_str().map(|name| !name.starts_with('.')).unwrap_or(false))
                    .unwrap_or(false)
            })
            .count()
        }
    } else {
        0
    };
    
    let items_text = if item_count == 1 {
        "1 item".to_string()
    } else {
        format!("{} items", item_count)
    };
    
    let items_label = Label::new(Some(&items_text));
    status_bar.append(&items_label);
    
    // Add view mode buttons
    let view_box = Box::new(Orientation::Horizontal, 4);
    let list_view_btn = Button::from_icon_name("view-list-symbolic");
    let grid_view_btn = Button::from_icon_name("view-grid-symbolic");
    // Default active view follows config/state
    if state.current_view_mode == ViewMode::List {
        list_view_btn.add_css_class("suggested-action");
        list_view_btn.add_css_class("active-tab");
    } else {
        grid_view_btn.add_css_class("suggested-action");
        grid_view_btn.add_css_class("active-tab");
    }
    {
        let list_btn = list_view_btn.clone();
        let grid_btn = grid_view_btn.clone();
        list_view_btn.connect_clicked(move |_| {
            // Toggle active styling first (no state borrows)
            grid_btn.remove_css_class("suggested-action");
            grid_btn.remove_css_class("active-tab");
            list_btn.add_css_class("suggested-action");
            list_btn.add_css_class("active-tab");
            // Then switch view (borrow state once, use clone to drop early)
            if let Some(state_rc) = crate::core::navigation::get_global_state() {
                let state_ref = state_rc.borrow().clone();
                switch_view_to_list(&state_ref);
                drop(state_ref); // Explicitly drop to release borrow
                state_rc.borrow_mut().set_view_mode(ViewMode::List);
            }
        });
    }
    {
        let list_btn = list_view_btn.clone();
        let grid_btn = grid_view_btn.clone();
        grid_view_btn.connect_clicked(move |_| {
            // Toggle active styling first
            list_btn.remove_css_class("suggested-action");
            list_btn.remove_css_class("active-tab");
            grid_btn.add_css_class("suggested-action");
            grid_btn.add_css_class("active-tab");
            // Then switch view
            if let Some(state_rc) = crate::core::navigation::get_global_state() {
                let state_ref = state_rc.borrow().clone();
                switch_view_to_grid(&state_ref);
                drop(state_ref);
                state_rc.borrow_mut().set_view_mode(ViewMode::Grid);
            }
        });
    }
    
    view_box.append(&list_view_btn);
    view_box.append(&grid_view_btn);
    status_bar.append(&view_box);
    
    status_bar
}
