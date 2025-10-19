use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Button, Separator, ScrolledWindow, ListBox, ListBoxRow};
use std::path::PathBuf;
use crate::core::bookmarks::BookmarksManager;
use crate::core::navigation::navigate_to_directory;

pub fn create_modern_sidebar(bookmarks_manager: &BookmarksManager) -> Box {
    let sidebar = Box::new(Orientation::Vertical, 0);
    sidebar.set_margin_start(8);
    sidebar.set_margin_end(8);
    sidebar.set_margin_top(8);
    sidebar.set_margin_bottom(8);
    sidebar.add_css_class("modern-sidebar");
    
    // Create scrollable area for sidebar content
    let scrolled = ScrolledWindow::new();
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);
    scrolled.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    
    let content_box = Box::new(Orientation::Vertical, 8);
    
    // Quick Access section
    let quick_access_section = create_sidebar_section("Quick Access", &bookmarks_manager.get_bookmarks_by_category("Quick Access"));
    content_box.append(&quick_access_section);
    
    // Add separator
    let separator1 = Separator::new(Orientation::Horizontal);
    content_box.append(&separator1);
    
    // This PC section
    let this_pc_section = create_sidebar_section("This PC", &bookmarks_manager.get_bookmarks_by_category("Quick Access"));
    content_box.append(&this_pc_section);
    
    // Add separator
    let separator2 = Separator::new(Orientation::Horizontal);
    content_box.append(&separator2);
    
    // Bookmarks section (if any custom bookmarks exist)
    let custom_bookmarks = bookmarks_manager.bookmarks.iter()
        .filter(|b| b.category != "Quick Access")
        .collect::<Vec<_>>();
    
    if !custom_bookmarks.is_empty() {
        let bookmarks_section = create_sidebar_section("Bookmarks", &custom_bookmarks);
        content_box.append(&bookmarks_section);
        
        let separator3 = Separator::new(Orientation::Horizontal);
        content_box.append(&separator3);
    }
    
    scrolled.set_child(Some(&content_box));
    sidebar.append(&scrolled);
    
    sidebar
}

fn create_sidebar_section(title: &str, bookmarks: &[&crate::core::bookmarks::Bookmark]) -> Box {
    let section = Box::new(Orientation::Vertical, 4);
    
    // Section title
    let title_label = Label::new(Some(title));
    title_label.add_css_class("sidebar-section-title");
    title_label.set_halign(gtk::Align::Start);
    title_label.set_margin_start(8);
    title_label.set_margin_end(8);
    title_label.set_margin_top(4);
    title_label.set_margin_bottom(4);
    section.append(&title_label);
    
    // Create list box for items
    let list_box = ListBox::new();
    list_box.add_css_class("sidebar-list");
    
    for bookmark in bookmarks {
        let item = create_sidebar_item(bookmark);
        list_box.append(&item);
    }
    
    section.append(&list_box);
    section
}

fn create_sidebar_item(bookmark: &crate::core::bookmarks::Bookmark) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.add_css_class("sidebar-item");
    
    let item_box = Box::new(Orientation::Horizontal, 8);
    item_box.set_margin_start(8);
    item_box.set_margin_end(8);
    item_box.set_margin_top(4);
    item_box.set_margin_bottom(4);
    
    // Icon
    let icon_label = Label::new(Some(&bookmark.icon));
    icon_label.add_css_class("sidebar-item-icon");
    icon_label.set_width_request(24);
    icon_label.set_halign(gtk::Align::Center);
    
    // Name
    let name_label = Label::new(Some(&bookmark.name));
    name_label.add_css_class("sidebar-item-name");
    name_label.set_halign(gtk::Align::Start);
    name_label.set_hexpand(true);
    
    item_box.append(&icon_label);
    item_box.append(&name_label);
    
    row.set_child(Some(&item_box));
    
    // Connect click handler
    let path_clone = bookmark.path.clone();
    row.connect_activate(move |_| {
        navigate_to_directory(path_clone.clone());
    });
    
    row
}
