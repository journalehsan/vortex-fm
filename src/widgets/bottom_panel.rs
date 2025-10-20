use gtk::prelude::*;
use gtk::{Box, Orientation, Button, Stack, Label, ProgressBar, Separator};
use std::rc::Rc;
use std::cell::RefCell;

pub struct BottomPanel {
    pub container: Box,
    pub stack: Stack,
    pub info_bar: Box,
}

impl BottomPanel {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        container.add_css_class("bottom-panel");
        
        // Top: Info bar showing [File Details] [Location Info] [Storage]
        let info_bar = create_info_bar();
        container.append(&info_bar);
        
        // Separator
        let separator = Separator::new(Orientation::Horizontal);
        container.append(&separator);
        
        // Tab buttons for Details/Terminal
        let tabs_box = Box::new(Orientation::Horizontal, 0);
        tabs_box.add_css_class("bottom-panel-tabs");
        
        let details_tab = Button::with_label("📋 Details");
        details_tab.add_css_class("bottom-tab-button");
        details_tab.add_css_class("active-tab");
        
        let terminal_tab = Button::with_label("🖥️ Terminal");
        terminal_tab.add_css_class("bottom-tab-button");
        
        tabs_box.append(&details_tab);
        tabs_box.append(&terminal_tab);
        
        // Spacer
        let spacer = Box::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        tabs_box.append(&spacer);
        
        container.append(&tabs_box);
        
        // Stack for Details and Terminal
        let stack = Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
        stack.set_transition_duration(200);
        stack.set_vexpand(true);
        stack.set_hexpand(true);
        
        // Placeholders will be replaced with actual widgets via add_details_panel and add_terminal_panel
        let details_placeholder = Box::new(Orientation::Vertical, 0);
        details_placeholder.add_css_class("details-placeholder");
        
        let terminal_placeholder = Box::new(Orientation::Vertical, 0);
        terminal_placeholder.add_css_class("terminal-placeholder");
        
        stack.add_named(&details_placeholder, Some("details"));
        stack.add_named(&terminal_placeholder, Some("terminal"));
        stack.set_visible_child_name("details");
        
        // Connect tab buttons to stack
        let stack_clone = stack.clone();
        details_tab.connect_clicked(move |btn| {
            stack_clone.set_visible_child_name("details");
            btn.add_css_class("active-tab");
            // Remove active from terminal tab
            if let Some(parent) = btn.parent() {
                if let Some(parent_box) = parent.downcast_ref::<Box>() {
                    let mut child = parent_box.first_child();
                    while let Some(c) = child {
                        if let Some(btn_child) = c.downcast_ref::<Button>() {
                            if btn_child != btn {
                                btn_child.remove_css_class("active-tab");
                            }
                        }
                        child = c.next_sibling();
                    }
                }
            }
        });
        
        let stack_clone = stack.clone();
        terminal_tab.connect_clicked(move |btn| {
            stack_clone.set_visible_child_name("terminal");
            btn.add_css_class("active-tab");
            // Remove active from details tab
            if let Some(parent) = btn.parent() {
                if let Some(parent_box) = parent.downcast_ref::<Box>() {
                    let mut child = parent_box.first_child();
                    while let Some(c) = child {
                        if let Some(btn_child) = c.downcast_ref::<Button>() {
                            if btn_child != btn {
                                btn_child.remove_css_class("active-tab");
                            }
                        }
                        child = c.next_sibling();
                    }
                }
            }
        });
        
        container.append(&stack);
        
        BottomPanel {
            container,
            stack,
            info_bar,
        }
    }
    
    pub fn set_details_panel(&self, details_widget: &Box) {
        // Remove the placeholder and add the real details panel
        if let Some(placeholder) = self.stack.child_by_name("details") {
            self.stack.remove(&placeholder);
        }
        self.stack.add_named(details_widget, Some("details"));
    }
    
    pub fn set_terminal_panel(&self, terminal_widget: &gtk::ScrolledWindow) {
        // Remove the placeholder and add the real terminal panel
        if let Some(placeholder) = self.stack.child_by_name("terminal") {
            self.stack.remove(&placeholder);
        }
        self.stack.add_named(terminal_widget, Some("terminal"));
    }
}

fn create_info_bar() -> Box {
    let info_bar = Box::new(Orientation::Horizontal, 16);
    info_bar.add_css_class("info-bar");
    info_bar.set_margin_start(12);
    info_bar.set_margin_end(12);
    info_bar.set_margin_top(8);
    info_bar.set_margin_bottom(8);
    
    // Section 1: File Details
    let file_details_box = create_info_section("File Details", vec![
        ("Name:", "No file selected"),
        ("Size:", "—"),
        ("Type:", "—"),
        ("Modified:", "—"),
    ]);
    info_bar.append(&file_details_box);
    
    // Separator
    let sep1 = Separator::new(Orientation::Vertical);
    info_bar.append(&sep1);
    
    // Section 2: Location Info
    let location_info_box = create_info_section("Location Info", vec![
        ("Location:", "—"),
        ("Items:", "—"),
        ("Selected:", "—"),
    ]);
    info_bar.append(&location_info_box);
    
    // Separator
    let sep2 = Separator::new(Orientation::Vertical);
    info_bar.append(&sep2);
    
    // Section 3: Storage
    let storage_box = create_info_section("Storage", vec![
        ("Usage:", "—"),
        ("Free:", "—"),
    ]);
    info_bar.append(&storage_box);
    
    info_bar
}

fn create_info_section(title: &str, items: Vec<(&str, &str)>) -> Box {
    let section = Box::new(Orientation::Vertical, 4);
    section.add_css_class("info-section");
    section.set_hexpand(true);
    
    // Section title
    let title_label = Label::new(Some(title));
    title_label.add_css_class("info-section-title");
    title_label.set_halign(gtk::Align::Start);
    section.append(&title_label);
    
    // Items
    for (label, value) in items {
        let item_box = Box::new(Orientation::Horizontal, 8);
        
        let label_widget = Label::new(Some(label));
        label_widget.add_css_class("info-item-label");
        label_widget.set_halign(gtk::Align::Start);
        label_widget.set_width_request(80);
        
        let value_widget = Label::new(Some(value));
        value_widget.add_css_class("info-item-value");
        value_widget.set_halign(gtk::Align::Start);
        value_widget.set_hexpand(true);
        
        item_box.append(&label_widget);
        item_box.append(&value_widget);
        section.append(&item_box);
    }
    
    section
}

// Global bottom panel reference
static mut GLOBAL_BOTTOM_PANEL: Option<Box> = None;
static mut GLOBAL_BOTTOM_STACK: Option<Stack> = None;

pub fn set_global_bottom_panel(bottom_panel: Box) {
    unsafe {
        GLOBAL_BOTTOM_PANEL = Some(bottom_panel);
    }
}

pub fn set_global_bottom_stack(stack: Stack) {
    unsafe {
        GLOBAL_BOTTOM_STACK = Some(stack);
    }
}

pub fn toggle_bottom_panel() {
    unsafe {
        if let Some(panel) = &GLOBAL_BOTTOM_PANEL {
            let is_visible = panel.is_visible();
            panel.set_visible(!is_visible);
            
            if is_visible {
                crate::utils::simple_debug::debug_info("BOTTOM_PANEL", "Bottom panel hidden");
            } else {
                crate::utils::simple_debug::debug_info("BOTTOM_PANEL", "Bottom panel shown");
            }
        }
    }
}

pub fn switch_bottom_panel_to_details() {
    unsafe {
        if let Some(stack) = &GLOBAL_BOTTOM_STACK {
            stack.set_visible_child_name("details");
        }
    }
}

pub fn switch_bottom_panel_to_terminal() {
    unsafe {
        if let Some(stack) = &GLOBAL_BOTTOM_STACK {
            stack.set_visible_child_name("terminal");
        }
    }
}
