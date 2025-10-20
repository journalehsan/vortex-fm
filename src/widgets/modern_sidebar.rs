use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Button, Separator, ScrolledWindow, ListBox, ListBoxRow};
use gtk::{gdk, gio};
use gtk::{TreeView, TreeViewColumn, CellRendererText, TreeStore, DropTarget};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use crate::core::bookmarks::{BookmarksManager, Bookmark, get_global_bookmarks_manager};
use crate::core::config::VortexConfig;
use crate::core::navigation::navigate_to_directory;
use crate::views::content_area::switch_to_home_view;

pub fn create_modern_sidebar(bookmarks_manager: &BookmarksManager, config: &VortexConfig) -> Box {
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
    
    // This PC section (Tree view of /home and mounted devices)
    let this_pc_section = create_this_pc_section(config);
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
    
    // Register this list box if it's Quick Access
    if title == "Quick Access" {
        add_qa_list_box(&list_box);
    }
    
    for bookmark in bookmarks {
        let item = create_sidebar_item(bookmark);
        list_box.append(&item);
    }
    
    // Add drop target for Quick Access section
    if title == "Quick Access" {
        let drop_target = DropTarget::new(String::static_type(), gdk::DragAction::COPY);
        let list_box_weak = list_box.downgrade();
        drop_target.connect_drop(move |_target, value, _x, _y| {
            if let Ok(path_str) = value.get::<String>() {
                let path = PathBuf::from(&path_str);
                if path.is_dir() {
                    if let Some(list) = list_box_weak.upgrade() {
                        // Add to Quick Access
                        if let Some(manager_rc) = get_global_bookmarks_manager() {
                            let folder_name = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Folder")
                                .to_string();
                            let bookmark = Bookmark::new(
                                folder_name,
                                path.clone(),
                                "📁".to_string(),
                                "Quick Access".to_string(),
                            );
                            manager_rc.borrow_mut().add_bookmark(bookmark.clone());
                            let _ = manager_rc.borrow().save();
                            
                            // Directly add to the visible list
                            let item = create_sidebar_item(&bookmark);
                            list.append(&item);
                            println!("✅ Added folder to Quick Access: {}", path.display());
                        }
                    }
                }
            }
            true
        });
        list_box.add_controller(drop_target);
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

fn create_this_pc_section(config: &VortexConfig) -> Box {
    let section = Box::new(Orientation::Vertical, 4);

    // Section title
    let title_label = Label::new(Some("This PC"));
    title_label.add_css_class("sidebar-section-title");
    title_label.set_halign(gtk::Align::Start);
    title_label.set_margin_start(8);
    title_label.set_margin_end(8);
    title_label.set_margin_top(4);
    title_label.set_margin_bottom(4);
    section.append(&title_label);

    // Create TreeStore with three columns: icon, display name, path
    let store = TreeStore::new(&[String::static_type(), String::static_type(), String::static_type()]);

    // Add /home as root
    if let Ok(home_dir) = std::env::var("HOME") {
        let home_iter = store.append(None);
        store.set(&home_iter, &[(0, &"📁"), (1, &"Home"), (2, &home_dir)]);

        // Populate first-level children (user directories) filtered by show_hidden_files
        if let Ok(entries) = std::fs::read_dir(&home_dir) {
            for entry in entries.flatten() {
                if let Ok(mt) = entry.metadata() {
                    if mt.is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            // Skip hidden files if show_hidden_files is false
                            if !config.show_hidden_files && name.starts_with('.') {
                                continue;
                            }
                            let child_iter = store.append(Some(&home_iter));
                            let path = entry.path().to_string_lossy().to_string();
                            store.set(&child_iter, &[(0, &"📂"), (1, &name), (2, &path)]);
                        }
                    }
                }
            }
        }
    }

    // Mounted devices (from /proc/mounts) as other roots
    for mount in get_mounted_devices() {
        let iter = store.append(None);
        let display = mount.display_name.clone();
        let mp_str = mount.mount_point.to_string_lossy().to_string();
        store.set(&iter, &[(0, &"💾"), (1, &display), (2, &mp_str)]);
    }

    // Create TreeView
    let tree = TreeView::with_model(&store);
    tree.add_css_class("sidebar-tree");
    tree.set_headers_visible(false);
    tree.set_vexpand(false);
    tree.set_hexpand(true);

    // Column with icon and display name
    let col = TreeViewColumn::new();
    
    // Icon cell
    let icon_cell = CellRendererText::new();
    icon_cell.set_property("font", "16");
    col.pack_start(&icon_cell, false);
    col.add_attribute(&icon_cell, "text", 0);
    
    // Name cell
    let name_cell = CellRendererText::new();
    col.pack_start(&name_cell, true);
    col.add_attribute(&name_cell, "text", 1);
    
    tree.append_column(&col);

    // Connect single-click for navigation (matching content area behavior)
    let store_clone = store.clone();
    let tree_weak = tree.downgrade();
    let gesture = gtk::GestureClick::new();
    gesture.set_button(1); // Left mouse button
    gesture.connect_pressed(move |gesture, n_press, _x, _y| {
        if gesture.current_button() == 1 && n_press == 1 {
            // Single click - navigate
            if let Some(tv) = tree_weak.upgrade() {
                let selection = tv.selection();
                if let Some((_model, iter)) = selection.selected() {
                    if let Ok(path_value) = store_clone.get_value(&iter, 2).get::<String>() {
                        let pathbuf = PathBuf::from(path_value);
                        navigate_to_directory(pathbuf);
                    }
                }
            }
        }
    });
    tree.add_controller(gesture);

    section.append(&tree);
    section
}

struct MountPoint {
    mount_point: PathBuf,
    display_name: String,
}

fn get_mounted_devices() -> Vec<MountPoint> {
    let mut mounts = Vec::new();
    if let Ok(f) = File::open("/proc/mounts") {
        let reader = BufReader::new(f);
        let mut seen = std::collections::HashSet::new();
        for line in reader.lines().flatten() {
            // /proc/mounts format: device mountpoint fstype options dump pass
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let mp = parts[1];
                // We only want mountpoints under /media, /run/media, /mnt, or root (/) and skip pseudo filesystems
                if mp.starts_with("/media") || mp.starts_with("/run/media") || mp.starts_with("/mnt") || mp == "/" {
                    if !seen.contains(mp) {
                        seen.insert(mp.to_string());
                        let display = if mp == "/" { "/".to_string() } else { mp.rsplit('/').next().unwrap_or(mp).to_string() };
                        mounts.push(MountPoint { mount_point: PathBuf::from(mp), display_name: display });
                    }
                }
            }
        }
    }
    mounts
}

// Global sidebar widget for refresh/update - store references to all Quick Access list boxes
static mut GLOBAL_SIDEBAR: Option<std::rc::Rc<std::cell::RefCell<Box>>> = None;
static mut GLOBAL_SIDEBAR_CONFIG: Option<VortexConfig> = None;
static mut GLOBAL_QA_LIST_BOXES: Option<std::rc::Rc<std::cell::RefCell<Vec<gtk::glib::WeakRef<ListBox>>>>> = None;

pub fn set_global_sidebar(sidebar: Box, config: VortexConfig) {
    unsafe {
        GLOBAL_SIDEBAR = Some(std::rc::Rc::new(std::cell::RefCell::new(sidebar)));
        GLOBAL_SIDEBAR_CONFIG = Some(config);
        GLOBAL_QA_LIST_BOXES = Some(std::rc::Rc::new(std::cell::RefCell::new(Vec::new())));
    }
}

pub fn add_qa_list_box(list_box: &ListBox) {
    unsafe {
        if let Some(qa_list_rc) = GLOBAL_QA_LIST_BOXES.as_ref() {
            qa_list_rc.borrow_mut().push(list_box.downgrade());
        }
    }
}

pub fn add_bookmark_to_qa_ui(bookmark: &Bookmark) {
    unsafe {
        if let Some(qa_list_rc) = GLOBAL_QA_LIST_BOXES.as_ref() {
            let mut list_boxes = qa_list_rc.borrow_mut();
            list_boxes.retain(|weak_ref| {
                if let Some(list) = weak_ref.upgrade() {
                    // Directly add item to the list
                    let item = create_sidebar_item(bookmark);
                    list.append(&item);
                    true
                } else {
                    false
                }
            });
        }
    }
}

pub fn refresh_sidebar() {
    if let Some(manager_rc) = get_global_bookmarks_manager() {
        let bookmarks_manager = manager_rc.borrow();
        unsafe {
            if let Some(qa_list_rc) = GLOBAL_QA_LIST_BOXES.as_ref() {
                let mut list_boxes = qa_list_rc.borrow_mut();
                list_boxes.retain(|weak_ref| {
                    if let Some(list) = weak_ref.upgrade() {
                        // Clear the list
                        while let Some(child) = list.first_child() {
                            list.remove(&child);
                        }
                        
                        // Re-add Welcome item
                        let welcome_row = create_welcome_item();
                        list.insert(&welcome_row, 0);
                        
                        // Re-add all Quick Access bookmarks
                        for bookmark in bookmarks_manager.get_bookmarks_by_category("Quick Access") {
                            let item = create_sidebar_item(bookmark);
                            list.append(&item);
                        }
                        true
                    } else {
                        false
                    }
                });
            }
        }
    }
}
