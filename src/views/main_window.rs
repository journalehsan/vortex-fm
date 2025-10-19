use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Orientation, Paned, Box};
use std::rc::Rc;
use std::cell::RefCell;
use crate::core::file_manager::FileManagerState;
use crate::core::navigation::{set_global_state, set_global_tab_manager};
use crate::core::tab_manager::TabManager;
use crate::core::bookmarks::BookmarksManager;
use crate::core::selection::{SelectionManager, set_global_selection_manager};
use crate::widgets::modern_sidebar::create_modern_sidebar;
use crate::widgets::tab_bar::create_tab_bar;
use crate::widgets::details_panel::create_details_panel;
use crate::widgets::terminal_panel::{create_terminal_panel, set_global_terminal_panel, set_global_terminal_revealer};
use crate::views::content_area::create_content_area;
use crate::utils::keyboard::setup_keyboard_shortcuts;

pub fn build_ui(app: &Application) {
    // Create main window with split panes
    let state = Rc::new(RefCell::new(FileManagerState::new()));
    set_global_state(state.clone());
    
    // Create tab manager, bookmarks manager, and selection manager
    let tab_manager = Rc::new(RefCell::new(TabManager::new()));
    set_global_tab_manager(tab_manager.clone());
    let bookmarks_manager = BookmarksManager::load();
    
    let selection_manager = Rc::new(RefCell::new(SelectionManager::new()));
    set_global_selection_manager(selection_manager.clone());
    
    // Add initial tab
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
    let home_path = std::path::PathBuf::from(&home);
    let initial_tab_id = tab_manager.borrow_mut().add_tab(home_path);
    
    // Sync the global state with the initial tab
    if let Some(active_tab) = tab_manager.borrow().get_active_tab() {
        state.borrow_mut().navigation_history = active_tab.navigation_history.clone();
    }
    
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
    crate::widgets::tab_bar::set_global_tab_bar(tab_bar.clone());
    main_box.append(&tab_bar);
    
    // Create the main horizontal split pane
    let main_paned = Paned::new(Orientation::Horizontal);
    
    // Left sidebar (modern design)
    let sidebar = create_modern_sidebar(&bookmarks_manager);
    main_paned.set_start_child(Some(&sidebar));
    
    // Create the content + details split pane
    let content_details_paned = Paned::new(Orientation::Horizontal);
    
    // Main content area
    let content_area = create_content_area(&mut state.borrow_mut());
    content_details_paned.set_start_child(Some(&content_area));
    
    // Right details panel
    let details_panel = create_details_panel();
    crate::widgets::details_panel::set_global_details_panel(details_panel.clone());
    content_details_paned.set_end_child(Some(&details_panel));
    content_details_paned.set_position(600); // Set initial position
    
    main_paned.set_end_child(Some(&content_details_paned));
    main_paned.set_position(state.borrow().config.sidebar_width);
    
    main_box.append(&main_paned);
    
    // Create terminal panel
    let (terminal_panel, terminal_revealer) = create_terminal_panel();
    set_global_terminal_panel(Rc::new(RefCell::new(terminal_panel)));
    set_global_terminal_revealer(terminal_revealer.clone());
    main_box.append(&terminal_revealer);

    window.set_child(Some(&main_box));
    
    // Add keyboard shortcuts
    setup_keyboard_shortcuts(&window);
    
    window.present();
}
