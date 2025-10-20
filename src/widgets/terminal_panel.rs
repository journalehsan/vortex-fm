use gtk::prelude::*;
use gtk::glib;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use std::process::Command;

pub struct TerminalPanel {
    pub widget: gtk::Revealer,
    pub terminal: gtk::TextView,
    pub visible: bool,
    pub current_directory: RefCell<PathBuf>,
    pub is_busy: RefCell<bool>,
    pub command_history: RefCell<Vec<String>>,
    pub history_index: RefCell<usize>,
}

impl TerminalPanel {
    pub fn new() -> Self {
        // Create TextView as terminal
        let terminal = gtk::TextView::new();
        terminal.set_editable(true);
        terminal.set_cursor_visible(true);
        terminal.set_monospace(true);
        terminal.set_wrap_mode(gtk::WrapMode::Word);
        
        // Set a nice dark theme
        terminal.style_context().add_class("terminal-widget");
        
        // Create revealer for smooth show/hide
        let revealer = gtk::Revealer::new();
        revealer.set_child(Some(&terminal));
        revealer.set_reveal_child(false);
        revealer.set_transition_type(gtk::RevealerTransitionType::SlideUp);
        revealer.set_transition_duration(300);
        
        // Get current directory
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        
        // Initialize terminal with welcome message
        Self::append_to_terminal(&terminal, &format!("Vortex Terminal v1.0\nCurrent directory: {}\nType 'help' for available commands.\n\n$ ", current_dir.display()));
        
        // Set up key press handler for command execution
        let terminal_clone = terminal.clone();
        let controller = gtk::EventControllerKey::new();
        controller.connect_key_pressed(move |_, _key, keycode, _modifiers| {
            if keycode == 36 { // Return key
                Self::handle_key_press(&terminal_clone);
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        terminal.add_controller(controller);
        
        Self {
            widget: revealer,
            terminal,
            visible: false,
            current_directory: RefCell::new(current_dir),
            is_busy: RefCell::new(false),
            command_history: RefCell::new(Vec::new()),
            history_index: RefCell::new(0),
        }
    }
    
    fn append_to_terminal(terminal: &gtk::TextView, text: &str) {
        let buffer = terminal.buffer();
        let mut iter = buffer.end_iter();
        buffer.insert(&mut iter, text);
        
        // Scroll to bottom
        terminal.scroll_to_mark(&buffer.create_mark(None, &buffer.end_iter(), false), 0.0, false, 0.0, 0.0);
    }
    
    fn handle_key_press(terminal: &gtk::TextView) {
        // Get current line
        let buffer = terminal.buffer();
        let start = buffer.start_iter();
        let end = buffer.end_iter();
        let text = buffer.text(&start, &end, false).to_string();
        
        // Find the last command line (after last $)
        if let Some(last_dollar) = text.rfind("$ ") {
            let command = &text[last_dollar + 2..].trim();
            if !command.is_empty() {
                Self::execute_command(terminal, command);
            }
        }
    }
    
    fn execute_command(terminal: &gtk::TextView, command: &str) {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        
        // Handle built-in commands
        match command {
            "help" => {
                Self::append_to_terminal(terminal, "Available commands:\n");
                Self::append_to_terminal(terminal, "  help     - Show this help message\n");
                Self::append_to_terminal(terminal, "  clear    - Clear the terminal\n");
                Self::append_to_terminal(terminal, "  pwd      - Print current directory\n");
                Self::append_to_terminal(terminal, "  ls       - List directory contents\n");
                Self::append_to_terminal(terminal, "  cd <dir> - Change directory\n");
                Self::append_to_terminal(terminal, "  exit     - Close terminal\n");
                Self::append_to_terminal(terminal, "\n$ ");
            }
            "clear" => {
                let buffer = terminal.buffer();
                buffer.set_text("");
                Self::append_to_terminal(terminal, "$ ");
            }
            "pwd" => {
                Self::append_to_terminal(terminal, &format!("{}\n$ ", current_dir.display()));
            }
            "ls" => {
                match std::fs::read_dir(&current_dir) {
                    Ok(entries) => {
                        for entry in entries.flatten() {
                            let name = entry.file_name().to_string_lossy().to_string();
                            Self::append_to_terminal(terminal, &format!("{}\n", name));
                        }
                    }
                    Err(e) => {
                        Self::append_to_terminal(terminal, &format!("Error: {}\n", e));
                    }
                }
                Self::append_to_terminal(terminal, "$ ");
            }
            cmd if cmd.starts_with("cd ") => {
                let new_dir = &cmd[3..].trim();
                let target_path = if new_dir.is_empty() {
                    std::env::var("HOME").unwrap_or_else(|_| "/".to_string()).into()
                } else if new_dir.starts_with('/') {
                    PathBuf::from(new_dir)
                } else {
                    current_dir.join(new_dir)
                };
                
                if target_path.exists() && target_path.is_dir() {
                    if let Err(e) = std::env::set_current_dir(&target_path) {
                        Self::append_to_terminal(terminal, &format!("Error changing directory: {}\n$ ", e));
                    } else {
                        Self::append_to_terminal(terminal, &format!("Changed to: {}\n$ ", target_path.display()));
                    }
                } else {
                    Self::append_to_terminal(terminal, &format!("Directory not found: {}\n$ ", target_path.display()));
                }
            }
            "exit" => {
                Self::append_to_terminal(terminal, "Terminal closed.\n");
                // TODO: Implement actual terminal closing
            }
            _ => {
                // Try to execute as system command
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(&["/C", command])
                        .current_dir(&current_dir)
                        .output()
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .current_dir(&current_dir)
                        .output()
                };
                
                match output {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        
                        if !stdout.is_empty() {
                            Self::append_to_terminal(terminal, &stdout);
                        }
                        if !stderr.is_empty() {
                            Self::append_to_terminal(terminal, &stderr);
                        }
                    }
                    Err(e) => {
                        Self::append_to_terminal(terminal, &format!("Error executing command: {}\n", e));
                    }
                }
                Self::append_to_terminal(terminal, "$ ");
            }
        }
    }
    
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
        self.widget.set_reveal_child(self.visible);
        
        if self.visible {
            crate::utils::simple_debug::debug_info("TERMINAL", "Terminal panel shown - VTE integration active!");
            // Focus the terminal when shown
            self.terminal.grab_focus();
        } else {
            crate::utils::simple_debug::debug_info("TERMINAL", "Terminal panel hidden");
        }
    }
    
    pub fn set_current_directory(&self, path: &PathBuf) {
        if *self.is_busy.borrow() {
            crate::utils::simple_debug::debug_info("TERMINAL", "Terminal busy - skipping directory sync");
            return;
        }
        
        // Update the current directory
        *self.current_directory.borrow_mut() = path.clone();
        
        // Update terminal display if visible
        if self.visible {
            Self::append_to_terminal(&self.terminal, &format!("Directory changed to: {}\n$ ", path.display()));
            crate::utils::simple_debug::debug_info("TERMINAL", &format!("Synced terminal to: {}", path.display()));
        }
    }
    
    pub fn get_current_directory(&self) -> PathBuf {
        self.current_directory.borrow().clone()
    }
    
    pub fn is_busy(&self) -> bool {
        *self.is_busy.borrow()
    }
    
    pub fn set_busy(&self, busy: bool) {
        *self.is_busy.borrow_mut() = busy;
    }
    
    pub fn run_command(&self, command: &str) {
        if self.visible {
            Self::execute_command(&self.terminal, command);
            crate::utils::simple_debug::debug_info("TERMINAL", &format!("Executed command: {}", command));
        }
    }
    
    pub fn terminal_widget(&self) -> &gtk::TextView {
        &self.terminal
    }
}

pub fn create_terminal_panel() -> (TerminalPanel, gtk::Revealer) {
    let terminal_panel = TerminalPanel::new();
    
    // Get the revealer from the terminal panel
    let terminal_revealer = terminal_panel.widget.clone();
    
    // Set up the revealer
    terminal_revealer.set_reveal_child(false); // Hidden by default
    terminal_revealer.set_transition_type(gtk::RevealerTransitionType::SlideUp);
    terminal_revealer.set_transition_duration(300);
    
    // Set height for the terminal
    terminal_panel.terminal.set_height_request(200);
    
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
                // Showing - make visible
                revealer.set_reveal_child(true);
                crate::utils::simple_debug::debug_info("TERMINAL", "Terminal panel shown - VTE integration active!");
                
                // Focus the terminal when shown
                if let Some(terminal_panel) = &GLOBAL_TERMINAL_PANEL {
                    let panel = terminal_panel.borrow();
                    panel.terminal.grab_focus();
                }
            } else {
                // Hiding
                revealer.set_reveal_child(false);
                crate::utils::simple_debug::debug_info("TERMINAL", "Terminal panel hidden");
            }
        }
    }
}

pub fn sync_terminal_directory(path: &PathBuf) {
    if let Some(terminal_rc) = get_global_terminal_panel() {
        let terminal = terminal_rc.borrow();
        if !terminal.is_busy() {
            terminal.set_current_directory(path);
        } else {
            crate::utils::simple_debug::debug_info("TERMINAL", "Terminal busy - directory sync deferred");
        }
    }
}
