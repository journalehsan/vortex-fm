use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Button, Separator, ScrolledWindow, ListBox, ListBoxRow};
use gtk::{gdk, gio};
use std::path::PathBuf;
use crate::core::bookmarks::{BookmarksManager, get_global_bookmarks_manager};
use crate::core::navigation::navigate_to_directory;
use crate::views::content_area::switch_to_home_view;

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
    // Prepend a custom "Welcome" item to Quick Access
    if let Some(list_widget) = quick_access_section.last_child() {
        if let Some(list_box) = list_widget.downcast_ref::<ListBox>() {
            let welcome_row = create_welcome_item();
            list_box.insert(&welcome_row, 0);
        }
    }
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

fn create_welcome_item() -> ListBoxRow {
    let row = ListBoxRow::new();
    row.add_css_class("sidebar-item");

    let item_box = Box::new(Orientation::Horizontal, 8);
    item_box.set_margin_start(8);
    item_box.set_margin_end(8);
    item_box.set_margin_top(4);
    item_box.set_margin_bottom(4);

    let icon_label = Label::new(Some("👋"));
    icon_label.add_css_class("sidebar-item-icon");
    icon_label.set_width_request(24);
    icon_label.set_halign(gtk::Align::Center);

    let name_label = Label::new(Some("Welcome"));
    name_label.add_css_class("sidebar-item-name");
    name_label.set_halign(gtk::Align::Start);
    name_label.set_hexpand(true);

    item_box.append(&icon_label);
    item_box.append(&name_label);

    row.set_child(Some(&item_box));

    let gesture = gtk::GestureClick::new();
    gesture.connect_pressed(move |_, _n_press, _x, _y| {
        switch_to_home_view();
    });
    row.add_controller(gesture);

    row
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

    // Pin/unpin button (visible for Quick Access items except Welcome)
    let show_pin = bookmark.category == "Quick Access" && bookmark.name != "Welcome";
    let pin_btn = Button::from_icon_name("pin-symbolic");
    pin_btn.add_css_class("flat");
    pin_btn.set_tooltip_text(Some("Unpin from Quick Access"));
    pin_btn.set_visible(show_pin);
    let path_for_pin = bookmark.path.clone();
    let row_weak = row.downgrade();
    pin_btn.connect_clicked(move |_| {
        if let Some(manager_rc) = get_global_bookmarks_manager() {
            manager_rc.borrow_mut().remove_bookmark(&path_for_pin);
            let _ = manager_rc.borrow().save();
        }
        // Remove the row from UI immediately
        if let Some(obj) = row_weak.upgrade() {
            if let Ok(row) = obj.downcast::<ListBoxRow>() {
                if let Some(parent) = row.parent() {
                    if let Ok(list_box) = parent.downcast::<ListBox>() {
                        list_box.remove(&row);
                    }
                }
            }
        }
    });
    item_box.append(&pin_btn);
    
    row.set_child(Some(&item_box));
    
    // Connect click handler using gesture
    let path_clone = bookmark.path.clone();
    let gesture = gtk::GestureClick::new();
    gesture.connect_pressed(move |_, _n_press, _x, _y| {
        navigate_to_directory(path_clone.clone());
    });
    row.add_controller(gesture);

    // Context menu (secondary click): remove from Quick Access (except "Welcome")
    if bookmark.category == "Quick Access" {
        let name_for_menu = bookmark.name.clone();
        let path_for_menu = bookmark.path.clone();
        let secondary = gtk::GestureClick::new();
        secondary.set_button(3);
        secondary.connect_pressed(move |g, _n, x, y| {
            if name_for_menu == "Welcome" { return; }
            let model = gio::Menu::new();
            model.append(Some("Remove"), Some("qa.remove"));
            let pop = gtk::PopoverMenu::from_model(Some(&model));
            let ag = gio::SimpleActionGroup::new();
            let path_for_menu_clone = path_for_menu.clone();
            let remove_action = gio::SimpleAction::new("remove", None);
            remove_action.connect_activate(move |_, _| {
                if let Some(manager_rc) = get_global_bookmarks_manager() {
                    manager_rc.borrow_mut().remove_bookmark(&path_for_menu_clone);
                    let _ = manager_rc.borrow().save();
                }
            });
            ag.add_action(&remove_action);
            // Attach actions to the popover
            pop.insert_action_group("qa", Some(&ag));
            // Show at click position
            let widget = g.widget();
            pop.set_parent(&widget);
            let rect = gdk::Rectangle::new(x as i32, y as i32, 1, 1);
            pop.set_pointing_to(Some(&rect));
            pop.set_has_arrow(false);
            pop.popup();
        });
        row.add_controller(secondary);
    }
    
    row
}
