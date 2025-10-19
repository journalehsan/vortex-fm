use gtk::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

pub struct TerminalPanel {
    pub widget: gtk::ScrolledWindow,
    pub visible: bool,
    pub current_directory: RefCell<PathBuf>,
}

impl TerminalPanel {
    pub fn new() -> Self {
        // Create a placeholder terminal widget for now
        // TODO: Replace with actual VTE terminal when API issues are resolved
        let scrolled_window = gtk::ScrolledWindow::new();
        scrolled_window.set_vexpand(true);
        scrolled_window.set_hscrollbar_policy(gtk::PolicyType::Never);
        
        // Create a simple text view as placeholder
        let text_view = gtk::TextView::new();
        text_view.set_editable(false);
        text_view.set_cursor_visible(false);
        text_view.set_wrap_mode(gtk::WrapMode::Word);
        
        // Add some placeholder text
        let buffer = text_view.buffer();
        buffer.set_text("🚀 Vortex Terminal Panel\n\nVTE integration coming soon!\nThis will be a fully functional terminal\nthat syncs with the current directory.\n\nPress F4 to toggle this panel.\n\nFeatures coming:\n• Real terminal emulation\n• Directory synchronization\n• Multiple terminal tabs\n• Custom themes");
        
        scrolled_window.set_child(Some(&text_view));
        
        // Get current directory
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        
        Self {
            widget: scrolled_window,
            visible: false,
            current_directory: RefCell::new(current_dir),
        }
    }
    
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
    
    pub fn set_current_directory(&self, path: &PathBuf) {
        // Update the current directory
        *self.current_directory.borrow_mut() = path.clone();
        crate::utils::simple_debug::debug_info("TERMINAL", &format!("Synced terminal to: {}", path.display()));
        
        // TODO: When VTE is properly integrated, send cd command to terminal
        // For now, just log the directory change
    }
    
    pub fn get_current_directory(&self) -> PathBuf {
        self.current_directory.borrow().clone()
    }
}

pub fn create_terminal_panel() -> (TerminalPanel, gtk::Revealer) {
    let terminal_panel = TerminalPanel::new();
    
    // Create revealer for smooth animations
    let terminal_revealer = gtk::Revealer::new();
    terminal_revealer.set_child(Some(&terminal_panel.widget));
    terminal_revealer.set_reveal_child(false); // Hidden by default
    terminal_revealer.set_transition_type(gtk::RevealerTransitionType::SlideUp);
    terminal_revealer.set_transition_duration(300);
    
    // Make it completely invisible when hidden
    terminal_panel.widget.set_height_request(200);
    terminal_revealer.set_height_request(0); // No height when hidden
    terminal_revealer.set_visible(false); // Completely hidden by default
    
    // Add some styling
    terminal_panel.widget.add_css_class("terminal-panel");
    
    (terminal_panel, terminal_revealer)
}

// Global terminal panel reference for updates
static mut GLOBAL_TERMINAL_PANEL: Option<Rc<RefCell<TerminalPanel>>> = None;
static mut GLOBAL_TERMINAL_REVEALER: Option<gtk::Revealer> = None;

pub fn set_global_terminal_panel(terminal_panel: Rc<RefCell<TerminalPanel>>) {
    unsafe {
        GLOBAL_TERMINAL_PANEL = Some(terminal_panel);
    }
}

pub fn set_global_terminal_revealer(terminal_revealer: gtk::Revealer) {
    unsafe {
        GLOBAL_TERMINAL_REVEALER = Some(terminal_revealer);
    }
}

pub fn get_global_terminal_panel() -> Option<Rc<RefCell<TerminalPanel>>> {
    unsafe {
        GLOBAL_TERMINAL_PANEL.as_ref().map(|panel| panel.clone())
    }
}

pub fn toggle_terminal_panel() {
    unsafe {
        if let Some(revealer) = &GLOBAL_TERMINAL_REVEALER {
            let currently_visible = revealer.reveals_child();
            
            if !currently_visible {
                // Showing - make visible and set height
                revealer.set_visible(true);
                revealer.set_height_request(200);
                revealer.set_reveal_child(true);
                crate::utils::simple_debug::debug_info("TERMINAL", "Terminal panel shown - VTE integration coming soon!");
            } else {
                // Hiding - completely hide and remove from layout
                revealer.set_reveal_child(false);
                revealer.set_visible(false);
                revealer.set_height_request(0);
                crate::utils::simple_debug::debug_info("TERMINAL", "Terminal panel hidden");
            }
        }
    }
}

pub fn sync_terminal_directory(path: &PathBuf) {
    if let Some(terminal_rc) = get_global_terminal_panel() {
        terminal_rc.borrow().set_current_directory(path);
    }
}
