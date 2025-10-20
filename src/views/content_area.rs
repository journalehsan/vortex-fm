use gtk::prelude::*;
use gtk::{Box, Orientation, ScrolledWindow, Stack, Button};
use crate::core::file_manager::FileManagerState;
use crate::views::path_bar::create_path_bar;
use crate::views::status_bar::create_status_bar;
use crate::widgets::home_screen::create_home_screen;
use crate::widgets::file_view::{FileView, ListViewAdapter, GridViewAdapter};
use std::rc::Rc;
use std::cell::RefCell;

// Global refs for view switching and styling
static mut GLOBAL_STACK: Option<Rc<RefCell<Stack>>> = None;
static mut GLOBAL_HOME_BTN: Option<Button> = None;
static mut GLOBAL_BROWSER_BTN: Option<Button> = None;
static mut GLOBAL_HOME_CONTAINER: Option<ScrolledWindow> = None;
static mut GLOBAL_TAB_BAR: Option<Box> = None;
static mut GLOBAL_NAV_BUTTONS: Option<Box> = None;
static mut GLOBAL_FILE_VIEW: Option<Rc<RefCell<FileView>>> = None;
static mut GLOBAL_ACTIVE_VIEW: &'static str = "grid"; // track current view mode

pub fn set_global_stack(stack: Rc<RefCell<Stack>>) {
    unsafe {
        GLOBAL_STACK = Some(stack);
    }
}

pub fn set_global_tab_bar(tab_bar: Box) {
    unsafe {
        GLOBAL_TAB_BAR = Some(tab_bar);
    }
}

pub fn set_global_nav_buttons(nav_buttons: Box) {
    unsafe {
        GLOBAL_NAV_BUTTONS = Some(nav_buttons);
    }
}

pub fn set_global_file_view(fv: Rc<RefCell<FileView>>) {
    unsafe { GLOBAL_FILE_VIEW = Some(fv); }
}

pub fn switch_view_to_list(state: &FileManagerState) {
    unsafe {
        if let Some(fv) = &GLOBAL_FILE_VIEW {
            fv.borrow_mut().set_adapter(std::boxed::Box::new(ListViewAdapter::new()), state);
            fv.borrow_mut().refresh(state);
            GLOBAL_ACTIVE_VIEW = "list";
        }
    }
}

pub fn switch_view_to_grid(state: &FileManagerState) {
    unsafe {
        if let Some(fv) = &GLOBAL_FILE_VIEW {
            fv.borrow_mut().set_adapter(std::boxed::Box::new(GridViewAdapter::new()), state);
            fv.borrow_mut().refresh(state);
            GLOBAL_ACTIVE_VIEW = "grid";
        }
    }
}

pub fn refresh_active_view() {
    unsafe {
        if let Some(fv) = &GLOBAL_FILE_VIEW {
            if let Some(state_rc) = crate::core::navigation::get_global_state() {
                let state_ref = state_rc.borrow();
                fv.borrow_mut().refresh(&state_ref);
            }
        }
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
    
    // File view with adapter pattern inside a scrolled container
    let scrolled = ScrolledWindow::new();
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);
    scrolled.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);

    let file_view = Rc::new(RefCell::new(FileView::new()));
    set_global_file_view(file_view.clone());
    // Default to config/state view
    {
        let mut fv = file_view.borrow_mut();
        if state.current_view_mode == "grid" {
            fv.set_adapter(std::boxed::Box::new(GridViewAdapter::new()), state);
        } else {
            fv.set_adapter(std::boxed::Box::new(ListViewAdapter::new()), state);
        }
        scrolled.set_child(Some(fv.widget()));
    }
    file_browser.append(&scrolled);
    
    // Status bar
    let status_bar = create_status_bar(state);
    file_browser.append(&status_bar);
    
    stack.add_named(&file_browser, Some("browser"));
    
    // Add navigation buttons
    let nav_box = Box::new(Orientation::Horizontal, 8);
    nav_box.set_css_classes(&["nav-buttons"]);
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
        GLOBAL_NAV_BUTTONS = Some(nav_box.clone());
    }
    
    // Store references for later updates (keep status bar only for now)
    state.status_bar = Some(status_bar.clone());
    
    content
}

fn create_file_list(state: &FileManagerState) -> ScrolledWindow {
    let scrolled = ScrolledWindow::new();
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);
    // Allow GTK to manage scrollbars to avoid clipping/breaking layout
    scrolled.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    
    // Use the state's method to populate the file list
    state.update_file_list(&scrolled);
    
    scrolled
}

// Function to update responsive margins based on sidebar width
pub fn update_responsive_margins(sidebar_width: i32) {
    unsafe {
        let width_class = get_sidebar_width_class(sidebar_width);
        let all_width_classes = [
            "sidebar-width-small",
            "sidebar-width-medium",
            "sidebar-width-large",
            "sidebar-width-xlarge",
            "sidebar-width-xxlarge",
        ];

        // Update home screen container
        if let Some(home_container) = &GLOBAL_HOME_CONTAINER {
            for cls in &all_width_classes { home_container.remove_css_class(cls); }
            home_container.add_css_class(width_class);
        }

        // Update tab bar
        if let Some(tab_bar) = &GLOBAL_TAB_BAR {
            for cls in &all_width_classes { tab_bar.remove_css_class(cls); }
            tab_bar.add_css_class(width_class);
        }

        // Update navigation buttons
        if let Some(nav_buttons) = &GLOBAL_NAV_BUTTONS {
            for cls in &all_width_classes { nav_buttons.remove_css_class(cls); }
            nav_buttons.add_css_class(width_class);
        }
    }
}

// Helper function to determine CSS class based on sidebar width
fn get_sidebar_width_class(width: i32) -> &'static str {
    match width {
        0..=100 => "sidebar-width-small",
        101..=150 => "sidebar-width-medium",
        151..=200 => "sidebar-width-large",
        201..=250 => "sidebar-width-xlarge",
        _ => "sidebar-width-xxlarge",
    }
}
