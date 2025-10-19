use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Orientation, Paned};
use std::rc::Rc;
use std::cell::RefCell;
use crate::core::file_manager::FileManagerState;
use crate::core::navigation::set_global_state;
use crate::views::sidebar::create_sidebar;
use crate::views::content_area::create_content_area;
use crate::utils::keyboard::setup_keyboard_shortcuts;

pub fn build_ui(app: &Application) {
    // Create main window with split panes
    let state = Rc::new(RefCell::new(FileManagerState::new()));
    set_global_state(state.clone());
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Vortex FM")
        .default_width(state.borrow().config.window_width)
        .default_height(state.borrow().config.window_height)
        .build();

    // Create the split pane layout (like Windows Explorer!)
    let main_paned = Paned::new(Orientation::Horizontal);
    
    // Left sidebar (20%)
    let sidebar = create_sidebar(&state.borrow());
    main_paned.set_start_child(Some(&sidebar));
    
    // Main content area (80%)
    let content_area = create_content_area(&mut state.borrow_mut());
    main_paned.set_end_child(Some(&content_area));
    
    main_paned.set_position(state.borrow().config.sidebar_width);

    window.set_child(Some(&main_paned));
    
    // Add keyboard shortcuts
    setup_keyboard_shortcuts(&window);
    
    window.present();
}
