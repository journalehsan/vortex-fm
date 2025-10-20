use gtk::prelude::*;
use gtk::{Box, Orientation, Button, Separator, Label, SearchEntry, ScrolledWindow};
use crate::core::file_manager::FileManagerState;
use crate::core::navigation::{get_global_state, navigate_to_directory};
use std::path::PathBuf;

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
    
    // Breadcrumb path display
    let breadcrumb_container = create_breadcrumb_path(state.current_path());
    breadcrumb_container.set_hexpand(true);
    set_global_breadcrumb_container(breadcrumb_container.clone());
    path_bar.append(&breadcrumb_container);
    
    // Store reference for later updates
    state.path_label = Some(create_path_label_for_state(state.current_path()));
    
    // Compact search: icon that expands to an entry with a close button
    let search_icon_btn = Button::from_icon_name("system-search-symbolic");
    let search_revealer = gtk::Revealer::new();
    search_revealer.set_transition_type(gtk::RevealerTransitionType::SlideLeft);
    search_revealer.set_reveal_child(false);

    let search_box = Box::new(Orientation::Horizontal, 4);
    let search_entry = SearchEntry::new();
    search_entry.set_placeholder_text(Some("Search files..."));
    search_entry.set_width_request(220);
    let search_close_btn = Button::from_icon_name("window-close-symbolic");

    // Connect search functionality
    search_entry.connect_search_changed(move |entry| {
        if let Some(state_rc) = get_global_state() {
            let filter_text = entry.text().to_string();
            state_rc.borrow_mut().set_filter(filter_text);
            state_rc.borrow().refresh_ui();
        }
    });

    // Toggle open: reveal and focus
    {
        let revealer = search_revealer.clone();
        let entry = search_entry.clone();
        search_icon_btn.connect_clicked(move |_| {
            revealer.set_reveal_child(true);
            entry.grab_focus();
        });
    }

    // Close: hide, clear text/filter, and refresh
    search_close_btn.connect_clicked({
        let revealer = search_revealer.clone();
        let entry = search_entry.clone();
        move |_| {
            entry.set_text("");
            if let Some(state_rc) = get_global_state() {
                state_rc.borrow_mut().clear_filter();
                state_rc.borrow().refresh_ui();
            }
            revealer.set_reveal_child(false);
        }
    });

    search_box.append(&search_entry);
    search_box.append(&search_close_btn);
    search_revealer.set_child(Some(&search_box));

    path_bar.append(&search_icon_btn);
    path_bar.append(&search_revealer);
    
    path_bar
}

fn create_breadcrumb_path(current_path: &PathBuf) -> ScrolledWindow {
    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Never);
    scrolled.set_hexpand(true);
    scrolled.add_css_class("breadcrumb-container");
    
    let mut breadcrumb_box = Box::new(Orientation::Horizontal, 0);
    breadcrumb_box.add_css_class("breadcrumb-path");
    
    // Get path components
    let path_str = current_path.to_string_lossy().to_string();
    let components = path_str.split('/').filter(|s| !s.is_empty()).collect::<Vec<&str>>();
    
    if components.is_empty() {
        // Root directory
        let root_btn = create_breadcrumb_button("/", "/");
        breadcrumb_box.append(&root_btn);
    } else {
        // Smart ellipsis handling for long paths
        let max_visible_segments = 6; // Show max 6 segments before ellipsis
        
        if components.len() <= max_visible_segments {
            // Short path - show all segments
            build_full_breadcrumb(&components, &mut breadcrumb_box);
        } else {
            // Long path - show first few, ellipsis, last few
            build_ellipsis_breadcrumb(&components, max_visible_segments, &mut breadcrumb_box);
        }
    }
    
    scrolled.set_child(Some(&breadcrumb_box));
    scrolled
}

fn create_breadcrumb_button(text: &str, path: &str) -> Button {
    let btn = Button::new();
    btn.set_label(text);
    btn.add_css_class("breadcrumb-button");
    btn.set_halign(gtk::Align::Start);
    
    // Connect click handler
    let path_buf = PathBuf::from(path);
    btn.connect_clicked(move |_| {
        navigate_to_directory(path_buf.clone());
    });
    
    btn
}

fn create_breadcrumb_separator() -> Label {
    let separator = Label::new(Some("›"));
    separator.add_css_class("breadcrumb-separator");
    separator.set_halign(gtk::Align::Center);
    separator
}

fn build_full_breadcrumb(components: &[&str], breadcrumb_box: &mut Box) {
    let mut current_path_str = String::new();
    
    for (i, component) in components.iter().enumerate() {
        if !current_path_str.is_empty() {
            current_path_str.push('/');
        }
        current_path_str.push_str(component);
        
        // Create breadcrumb button
        let btn = create_breadcrumb_button(component, &current_path_str);
        breadcrumb_box.append(&btn);
        
        // Add separator (except for last item)
        if i < components.len() - 1 {
            let separator = create_breadcrumb_separator();
            breadcrumb_box.append(&separator);
        }
    }
}

fn build_ellipsis_breadcrumb(components: &[&str], _max_visible: usize, breadcrumb_box: &mut Box) {
    let total = components.len();
    let show_first = 2; // Always show first 2 segments
    let show_last = 2;  // Always show last 2 segments
    
    // Build first segments
    let mut current_path_str = String::new();
    for i in 0..show_first {
        if !current_path_str.is_empty() {
            current_path_str.push('/');
        }
        current_path_str.push_str(components[i]);
        
        let btn = create_breadcrumb_button(components[i], &current_path_str);
        breadcrumb_box.append(&btn);
        
        let separator = create_breadcrumb_separator();
        breadcrumb_box.append(&separator);
    }
    
    // Add ellipsis button (shows middle segments on click)
    let ellipsis_btn = create_ellipsis_button(components, show_first, total - show_last);
    breadcrumb_box.append(&ellipsis_btn);
    
    let separator = create_breadcrumb_separator();
    breadcrumb_box.append(&separator);
    
    // Build last segments - reconstruct the full path correctly
    let mut last_segments_path = String::new();
    for i in 0..(total - show_last) {
        if !last_segments_path.is_empty() {
            last_segments_path.push('/');
        }
        last_segments_path.push_str(components[i]);
    }
    
    for i in (total - show_last)..total {
        if !last_segments_path.is_empty() {
            last_segments_path.push('/');
        }
        last_segments_path.push_str(components[i]);
        
        let btn = create_breadcrumb_button(components[i], &last_segments_path);
        breadcrumb_box.append(&btn);
        
        if i < total - 1 {
            let separator = create_breadcrumb_separator();
            breadcrumb_box.append(&separator);
        }
    }
}

fn create_ellipsis_button(_components: &[&str], _start: usize, _end: usize) -> Button {
    let btn = Button::new();
    btn.set_label("⋯");
    btn.add_css_class("breadcrumb-ellipsis");
    btn.set_halign(gtk::Align::Center);
    
    // For now, ellipsis is just visual - in future could show popover
    btn.connect_clicked(move |_| {
        println!("Ellipsis clicked - showing hidden path segments");
    });
    
    btn
}

fn create_path_label_for_state(current_path: &PathBuf) -> Label {
    // This is kept for compatibility with existing state management
    let current_path_str = current_path.to_string_lossy().to_string();
    let path_label = Label::new(Some(&current_path_str));
    path_label.add_css_class("path-label");
    path_label
}

// Global reference to the breadcrumb container for updates
static mut GLOBAL_BREADCRUMB_CONTAINER: Option<ScrolledWindow> = None;

pub fn set_global_breadcrumb_container(container: ScrolledWindow) {
    unsafe {
        GLOBAL_BREADCRUMB_CONTAINER = Some(container);
    }
}

pub fn update_breadcrumb_path(current_path: &PathBuf) {
    unsafe {
        if let Some(container) = &GLOBAL_BREADCRUMB_CONTAINER {
            // Clear existing content
            if let Some(child) = container.child() {
                container.set_child(None::<&gtk::Widget>);
            }
            
            // Create new breadcrumb
            let mut breadcrumb_box = Box::new(Orientation::Horizontal, 0);
            breadcrumb_box.add_css_class("breadcrumb-path");
            
            // Get path components
            let path_str = current_path.to_string_lossy().to_string();
            let components = path_str.split('/').filter(|s| !s.is_empty()).collect::<Vec<&str>>();
            
            if components.is_empty() {
                // Root directory
                let root_btn = create_breadcrumb_button("/", "/");
                breadcrumb_box.append(&root_btn);
            } else {
                // Smart ellipsis handling for long paths
                let max_visible_segments = 6; // Show max 6 segments before ellipsis
                
                if components.len() <= max_visible_segments {
                    // Short path - show all segments
                    build_full_breadcrumb(&components, &mut breadcrumb_box);
                } else {
                    // Long path - show first few, ellipsis, last few
                    build_ellipsis_breadcrumb(&components, max_visible_segments, &mut breadcrumb_box);
                }
            }
            
            container.set_child(Some(&breadcrumb_box));
        }
    }
}

