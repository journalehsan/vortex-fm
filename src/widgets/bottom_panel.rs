use gtk::prelude::*;
use gtk::{Box, Orientation, Button, Stack, Label, Separator};
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
        // Don't expand vertically - maintain compact height
        container.set_vexpand(false);
        
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
        // Don't expand vertically - let details height define the size
        stack.set_vexpand(false);
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
            // Clear any fixed height so details can size naturally
            if let Some(parent) = stack_clone.parent() {
                if let Some(container) = parent.downcast_ref::<Box>() {
                    container.set_height_request(-1);
                    stack_clone.set_vexpand(false);
                }
            }
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
        let container_clone = container.clone();
        terminal_tab.connect_clicked(move |btn| {
            stack_clone.set_visible_child_name("terminal");
            // Set height for terminal (tabs + terminal ~300px)
            container_clone.set_height_request(300);
            // Don't expand - let height_request control the size
            stack_clone.set_vexpand(false);
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
            
            // Focus the terminal input when terminal tab is clicked
            crate::widgets::terminal_panel::focus_terminal_input();
        });
        
        container.append(&stack);
        
        // Let initial height be determined by details content
        container.set_height_request(-1);
        
        let info_bar = container.clone();
        
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
    
    pub fn set_terminal_panel(&self, terminal_widget: &gtk::Revealer) {
        // Remove the placeholder and add the real terminal panel
        if let Some(placeholder) = self.stack.child_by_name("terminal") {
            self.stack.remove(&placeholder);
        }
        self.stack.add_named(terminal_widget, Some("terminal"));
    }
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
