use gtk::prelude::*;
use gtk::{Box, Orientation, ScrolledWindow, Stack, Button};
use crate::core::file_manager::FileManagerState;
use crate::views::path_bar::create_path_bar;
use crate::views::status_bar::create_status_bar;
use crate::widgets::home_screen::create_home_screen;
use std::rc::Rc;
use std::cell::RefCell;

// Global refs for view switching and styling
static mut GLOBAL_STACK: Option<Rc<RefCell<Stack>>> = None;
static mut GLOBAL_HOME_BTN: Option<Button> = None;
static mut GLOBAL_BROWSER_BTN: Option<Button> = None;
static mut GLOBAL_HOME_CONTAINER: Option<ScrolledWindow> = None;

pub fn set_global_stack(stack: Rc<RefCell<Stack>>) {
    unsafe {
        GLOBAL_STACK = Some(stack);
    }
}

pub fn switch_to_browser_view() {
    unsafe {
        if let Some(stack) = &GLOBAL_STACK { stack.borrow().set_visible_child_name("browser"); }
        if let Some(home_btn) = &GLOBAL_HOME_BTN { home_btn.set_css_classes(&["flat"]); }
        if let Some(browser_btn) = &GLOBAL_BROWSER_BTN { browser_btn.set_css_classes(&["suggested-action", "active-tab"]); }
        if let Some(home_container) = &GLOBAL_HOME_CONTAINER { home_container.set_css_classes(&["home-screen", "home-inactive"]); }
    }
}

pub fn switch_to_home_view() {
    unsafe {
        if let Some(stack) = &GLOBAL_STACK { stack.borrow().set_visible_child_name("home"); }
        if let Some(home_btn) = &GLOBAL_HOME_BTN { home_btn.set_css_classes(&["suggested-action", "active-tab"]); }
        if let Some(browser_btn) = &GLOBAL_BROWSER_BTN { browser_btn.set_css_classes(&["flat"]); }
        if let Some(home_container) = &GLOBAL_HOME_CONTAINER { home_container.set_css_classes(&["home-screen"]); }
    }
}

pub fn create_content_area(state: &mut FileManagerState) -> Box {
    let content = Box::new(Orientation::Vertical, 0);
    
    // Create a stack to switch between home screen and file browser
    let stack = Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(300);
    
    // Store global reference
    let stack_rc = Rc::new(RefCell::new(stack));
    set_global_stack(stack_rc.clone());
    let stack = stack_rc.borrow().clone();
    
    // Home screen (wrapped in scrolled window)
    let home_screen = create_home_screen();
    let home_scrolled = ScrolledWindow::new();
    home_scrolled.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    home_scrolled.set_hexpand(true);
    home_scrolled.set_vexpand(true);
    home_scrolled.set_child(Some(&home_screen));
    home_scrolled.set_css_classes(&["home-screen"]);
    stack.add_named(&home_scrolled, Some("home"));
    
    // File browser view
    let file_browser = Box::new(Orientation::Vertical, 0);
    
    // Path bar (like Windows Explorer)
    let path_bar = create_path_bar(state);
    file_browser.append(&path_bar);
    
    // File list area
    let file_list = create_file_list(state);
    file_browser.append(&file_list);
    
    // Status bar
    let status_bar = create_status_bar(state);
    file_browser.append(&status_bar);
    
    stack.add_named(&file_browser, Some("browser"));
    
    // Add navigation buttons
    let nav_box = Box::new(Orientation::Horizontal, 8);
    nav_box.set_margin_start(8);
    nav_box.set_margin_end(8);
    nav_box.set_margin_top(8);
    nav_box.set_margin_bottom(8);
    
    let home_btn = Button::new();
    home_btn.set_label("🏠 Home");
    home_btn.set_css_classes(&["suggested-action", "active-tab"]);
    home_btn.connect_clicked({
        move |_| { switch_to_home_view(); }
    });
    
    let browser_btn = Button::new();
    browser_btn.set_label("📁 Browser");
    browser_btn.set_css_classes(&["flat"]);
    browser_btn.connect_clicked({
        move |_| { switch_to_browser_view(); }
    });
    
    nav_box.append(&home_btn);
    nav_box.append(&browser_btn);
    
    content.append(&nav_box);
    content.append(&stack);
    
    // Start with home screen visible
    stack.set_visible_child_name("home");

    // Store global ui refs for styling toggles
    unsafe {
        GLOBAL_HOME_BTN = Some(home_btn.clone());
        GLOBAL_BROWSER_BTN = Some(browser_btn.clone());
        GLOBAL_HOME_CONTAINER = Some(home_scrolled.clone());
    }
    
    // Store references for later updates
    state.file_list_widget = Some(file_list.clone());
    state.status_bar = Some(status_bar.clone());
    
    content
}

fn create_file_list(state: &FileManagerState) -> ScrolledWindow {
    let scrolled = ScrolledWindow::new();
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);
    
    // Use the state's method to populate the file list
    state.update_file_list(&scrolled);
    
    scrolled
}
