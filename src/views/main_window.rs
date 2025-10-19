use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Orientation, Paned, Box};
use std::rc::Rc;
use std::cell::RefCell;
use crate::core::file_manager::FileManagerState;
use crate::core::navigation::set_global_state;
use crate::core::tab_manager::TabManager;
use crate::core::bookmarks::BookmarksManager;
use crate::widgets::modern_sidebar::create_modern_sidebar;
use crate::widgets::tab_bar::create_tab_bar;
use crate::views::content_area::create_content_area;
use crate::utils::keyboard::setup_keyboard_shortcuts;

pub fn build_ui(app: &Application) {
    // Create main window with split panes
    let state = Rc::new(RefCell::new(FileManagerState::new()));
    set_global_state(state.clone());
    
    // Create tab manager and bookmarks manager
    let tab_manager = Rc::new(RefCell::new(TabManager::new()));
    let bookmarks_manager = BookmarksManager::load();
    
    // Add initial tab
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
    let home_path = std::path::PathBuf::from(&home);
    tab_manager.borrow_mut().add_tab(home_path);
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Vortex FM")
        .default_width(state.borrow().config.window_width)
        .default_height(state.borrow().config.window_height)
        .build();

    // Create main vertical layout
    let main_box = Box::new(Orientation::Vertical, 0);
    
    // Tab bar at the top
    let tab_bar = create_tab_bar(tab_manager.clone());
    main_box.append(&tab_bar);
    
    // Create the split pane layout (like Windows Explorer!)
    let main_paned = Paned::new(Orientation::Horizontal);
    
    // Left sidebar (modern design)
    let sidebar = create_modern_sidebar(&bookmarks_manager);
    main_paned.set_start_child(Some(&sidebar));
    
    // Main content area (80%)
    let content_area = create_content_area(&mut state.borrow_mut());
    main_paned.set_end_child(Some(&content_area));
    
    main_paned.set_position(state.borrow().config.sidebar_width);
    
    main_box.append(&main_paned);

    window.set_child(Some(&main_box));
    
    // Add keyboard shortcuts
    setup_keyboard_shortcuts(&window);
    
    window.present();
}
