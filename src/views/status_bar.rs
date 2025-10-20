use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Button, Scale, Adjustment};
use crate::core::file_manager::{FileManagerState, ViewMode};
use crate::views::content_area::{switch_view_to_list, switch_view_to_grid};
use crate::utils::icon_manager::IconSize;

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
    
    // Add icon size slider for grid view
    let icon_size_label = Label::new(Some("Icon Size:"));
    icon_size_label.add_css_class("icon-size-label");
    status_bar.append(&icon_size_label);
    
    // Create size display label
    let size_display_label = Label::new(Some("32px"));
    size_display_label.add_css_class("icon-size-display");
    status_bar.append(&size_display_label);
    
    let adjustment = Adjustment::new(32.0, 16.0, 256.0, 1.0, 1.0, 0.0);
    let icon_size_scale = Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
    icon_size_scale.set_draw_value(false);
    icon_size_scale.set_width_request(120);
    icon_size_scale.add_css_class("icon-size-scale");
    
    // Set the initial value explicitly to ensure position matches
    icon_size_scale.set_value(32.0);
    
    // Connect icon size change
    let size_display_clone = size_display_label.clone();
    let scale_clone = icon_size_scale.clone();
    icon_size_scale.connect_value_changed(move |scale| {
        let raw_value = scale.value() as i32;
        
        // Snap to nearest standard icon size
        let standard_sizes = vec![16, 32, 48, 64, 96, 128, 256];
        let nearest_size = standard_sizes.iter()
            .min_by_key(|&&size| (size - raw_value).abs())
            .unwrap_or(&32);
        
        // Update the slider to the nearest standard size
        scale_clone.set_value(*nearest_size as f64);
        
        let icon_size = IconSize::from_pixels(*nearest_size);
        let size_text = format!("{}px", icon_size.to_pixels());
        size_display_clone.set_text(&size_text);
        
        // Update the global file view icon size
        if let Some(state_rc) = crate::core::navigation::get_global_state() {
            let state_ref = state_rc.borrow().clone();
            if let Some(fv) = crate::views::content_area::get_global_file_view() {
                fv.borrow_mut().set_icon_size(*nearest_size);
                // Refresh the view to apply new icon size
                fv.borrow_mut().update(&state_ref);
            }
        }
    });
    
    status_bar.append(&icon_size_scale);
    
    status_bar
}
