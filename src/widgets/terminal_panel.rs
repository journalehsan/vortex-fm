use gtk::prelude::*;
use gtk::glib;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use std::process::Command;

pub struct TerminalPanel {
    pub widget: gtk::Revealer,
    pub output_view: gtk::TextView,
    pub input_entry: gtk::Entry,
    pub prompt_label: gtk::Label,
    pub visible: bool,
    pub current_directory: RefCell<PathBuf>,
    pub is_busy: RefCell<bool>,
    pub command_history: RefCell<Vec<String>>,
    pub history_index: RefCell<usize>,
}

impl TerminalPanel {
    pub fn new() -> Self {
        // Create output TextView (read-only)
        let output_view = gtk::TextView::new();
        output_view.set_editable(false);
        output_view.set_cursor_visible(false);
        output_view.set_monospace(true);
        output_view.set_wrap_mode(gtk::WrapMode::Word);
        output_view.style_context().add_class("terminal-output");
        
        // Wrap output view in ScrolledWindow for scrolling
        let scrolled_output = gtk::ScrolledWindow::new();
        scrolled_output.set_child(Some(&output_view));
        scrolled_output.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        scrolled_output.set_hexpand(true);
        scrolled_output.set_vexpand(true);
        scrolled_output.set_min_content_height(200);
        scrolled_output.set_max_content_height(500);
        scrolled_output.style_context().add_class("terminal-scrolled");
        
        // Get current directory
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        
        // Create input Entry
        let input_entry = gtk::Entry::new();
        input_entry.set_placeholder_text(Some("Enter command..."));
        input_entry.style_context().add_class("terminal-input");
        
        // Create prompt label with user@hostname:pwd$
        let prompt_label = gtk::Label::new(Some(""));
        prompt_label.style_context().add_class("terminal-prompt");
        Self::update_prompt_label(&prompt_label, &current_dir);
        
        // Create input container with prompt and entry
        let input_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        input_container.append(&prompt_label);
        input_container.append(&input_entry);
        input_container.style_context().add_class("terminal-input-container");
        
        // Create main container with input on top, output below
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        main_box.append(&input_container);
        main_box.append(&scrolled_output);
        main_box.style_context().add_class("terminal-panel");
        
        // Create revealer for smooth show/hide
        let revealer = gtk::Revealer::new();
        revealer.set_child(Some(&main_box));
        revealer.set_reveal_child(false);
        revealer.set_transition_type(gtk::RevealerTransitionType::SlideUp);
        revealer.set_transition_duration(300);
        
        // Initialize terminal with welcome message
        Self::append_output(&output_view, &format!("Vortex Terminal v1.0\nCurrent directory: {}\nType 'help' for available commands.\n\n", current_dir.display()));
        Self::update_prompt(&input_entry, &current_dir);
        
        Self {
            widget: revealer,
            output_view,
            input_entry,
            prompt_label,
            visible: false,
            current_directory: RefCell::new(current_dir),
            is_busy: RefCell::new(false),
            command_history: RefCell::new(Vec::new()),
            history_index: RefCell::new(0),
        }
    }
    
    fn append_output(output_view: &gtk::TextView, text: &str) {
        let buffer = output_view.buffer();
        let mut iter = buffer.end_iter();
        buffer.insert(&mut iter, text);
        
        // Scroll to bottom
        let mark = buffer.create_mark(None, &buffer.end_iter(), false);
        output_view.scroll_to_mark(&mark, 0.0, false, 0.0, 0.0);
    }
    
    fn update_prompt_label(prompt_label: &gtk::Label, current_dir: &PathBuf) {
        let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
        let hostname = std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("HOST"))
            .unwrap_or_else(|_| "localhost".to_string());
        
        let dir_name = current_dir.file_name()
            .unwrap_or(std::ffi::OsStr::new("~"))
            .to_string_lossy();
        
        let prompt = format!("{}@{}:{} $ ", username, hostname, dir_name);
        prompt_label.set_text(&prompt);
    }
    
    fn update_prompt(input_entry: &gtk::Entry, current_dir: &PathBuf) {
        // Keep the entry placeholder simple since we now have a separate prompt label
        input_entry.set_placeholder_text(Some("Enter command..."));
    }
    
    pub fn connect_events(&self) {
        let output_view = self.output_view.clone();
        let input_entry = self.input_entry.clone();
        let prompt_label = self.prompt_label.clone();
        let current_dir = self.current_directory.clone();
        let command_history = self.command_history.clone();
        let history_index = self.history_index.clone();
        
        // Handle Enter key to execute command
        let input_entry_clone = input_entry.clone();
        input_entry.connect_activate(move |entry| {
            let command = entry.text().to_string();
            if !command.trim().is_empty() {
                Self::execute_command(&output_view, &input_entry_clone, &prompt_label, &current_dir, &command_history, &history_index, &command);
                entry.set_text("");
            }
        });
        
        // Handle Up/Down arrows for command history
        let input_entry_for_history = self.input_entry.clone();
        let command_history = self.command_history.clone();
        let history_index = self.history_index.clone();
        
        let controller = gtk::EventControllerKey::new();
        controller.connect_key_pressed(move |_, _key, keycode, _modifiers| {
            match keycode {
                111 => { // Up arrow
                    Self::navigate_history(&input_entry_for_history, &command_history, &history_index, 1);
                    glib::Propagation::Stop
                }
                116 => { // Down arrow
                    Self::navigate_history(&input_entry_for_history, &command_history, &history_index, -1);
                    glib::Propagation::Stop
                }
                _ => glib::Propagation::Proceed,
            }
        });
        self.input_entry.add_controller(controller);
    }
    
    fn navigate_history(input_entry: &gtk::Entry, command_history: &RefCell<Vec<String>>, history_index: &RefCell<usize>, direction: i32) {
        let history = command_history.borrow();
        let mut index = history_index.borrow_mut();
        
        if history.is_empty() {
            return;
        }
        
        match direction {
            1 => { // Up arrow - older command
                if *index > 0 {
                    *index -= 1;
                }
            }
            -1 => { // Down arrow - newer command
                if *index < history.len() - 1 {
                    *index += 1;
                } else {
                    *index = history.len();
                    input_entry.set_text("");
                    return;
                }
            }
            _ => return,
        }
        
        if *index < history.len() {
            input_entry.set_text(&history[*index]);
        }
    }
    
    fn execute_command(
        output_view: &gtk::TextView, 
        input_entry: &gtk::Entry, 
        prompt_label: &gtk::Label,
        current_dir: &RefCell<PathBuf>, 
        command_history: &RefCell<Vec<String>>, 
        history_index: &RefCell<usize>, 
        command: &str
    ) {
        // Add command to history
        {
            let mut history = command_history.borrow_mut();
            history.push(command.to_string());
            *history_index.borrow_mut() = history.len();
        }
        
        // Show command in output
        Self::append_output(output_view, &format!("$ {}\n", command));
        
        let current_dir_path = current_dir.borrow().clone();
        
        // Handle built-in commands
        match command {
            "help" => {
                Self::append_output(output_view, "Available commands:\n");
                Self::append_output(output_view, "  help     - Show this help message\n");
                Self::append_output(output_view, "  clear    - Clear the terminal\n");
                Self::append_output(output_view, "  pwd      - Print current directory\n");
                Self::append_output(output_view, "  ls       - List directory contents\n");
                Self::append_output(output_view, "  cd <dir> - Change directory\n");
                Self::append_output(output_view, "  exit     - Close terminal\n");
                Self::append_output(output_view, "\n");
            }
            "clear" => {
                let buffer = output_view.buffer();
                buffer.set_text("");
                Self::append_output(output_view, "Vortex Terminal v1.0\n");
                Self::append_output(output_view, &format!("Current directory: {}\n", current_dir_path.display()));
                Self::append_output(output_view, "Type 'help' for available commands.\n\n");
            }
            "pwd" => {
                Self::append_output(output_view, &format!("{}\n", current_dir_path.display()));
            }
            "ls" => {
                match std::fs::read_dir(&current_dir_path) {
                    Ok(entries) => {
                        for entry in entries.flatten() {
                            let name = entry.file_name().to_string_lossy().to_string();
                            Self::append_output(output_view, &format!("{}\n", name));
                        }
                    }
                    Err(e) => {
                        Self::append_output(output_view, &format!("Error: {}\n", e));
                    }
                }
            }
            cmd if cmd.starts_with("cd ") => {
                let new_dir = &cmd[3..].trim();
                let target_path = if new_dir.is_empty() {
                    std::env::var("HOME").unwrap_or_else(|_| "/".to_string()).into()
                } else if new_dir.starts_with('/') {
                    PathBuf::from(new_dir)
                } else {
                    current_dir_path.join(new_dir)
                };
                
                if target_path.exists() && target_path.is_dir() {
                    if let Err(e) = std::env::set_current_dir(&target_path) {
                        Self::append_output(output_view, &format!("Error changing directory: {}\n", e));
                    } else {
                        *current_dir.borrow_mut() = target_path.clone();
                        Self::append_output(output_view, &format!("Changed to: {}\n", target_path.display()));
                        Self::update_prompt_label(prompt_label, &target_path);
                        Self::update_prompt(input_entry, &target_path);
                    }
                } else {
                    Self::append_output(output_view, &format!("Directory not found: {}\n", target_path.display()));
                }
            }
            "exit" => {
                Self::append_output(output_view, "Terminal closed.\n");
                // TODO: Implement actual terminal closing
            }
            _ => {
                // Try to execute as system command
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(&["/C", command])
                        .current_dir(&current_dir_path)
                        .output()
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .current_dir(&current_dir_path)
                        .output()
                };
                
                match output {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        
                        if !stdout.is_empty() {
                            Self::append_output(output_view, &stdout);
                        }
                        if !stderr.is_empty() {
                            Self::append_output(output_view, &stderr);
                        }
                    }
                    Err(e) => {
                        Self::append_output(output_view, &format!("Error executing command: {}\n", e));
                    }
                }
            }
        }
    }
    
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
        self.widget.set_reveal_child(self.visible);
        
        if self.visible {
            crate::utils::simple_debug::debug_info("TERMINAL", "Terminal panel shown - Input/Output terminal active!");
            // Focus the input entry when shown
            self.input_entry.grab_focus();
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
        
        // Update prompt label and entry if visible
        if self.visible {
            Self::update_prompt_label(&self.prompt_label, path);
            Self::update_prompt(&self.input_entry, path);
            Self::append_output(&self.output_view, &format!("Directory changed to: {}\n", path.display()));
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
            Self::execute_command(
                &self.output_view, 
                &self.input_entry, 
                &self.prompt_label,
                &self.current_directory, 
                &self.command_history, 
                &self.history_index, 
                command
            );
            crate::utils::simple_debug::debug_info("TERMINAL", &format!("Executed command: {}", command));
        }
    }
    
    pub fn terminal_widget(&self) -> &gtk::TextView {
        &self.output_view
    }
}

pub fn create_terminal_panel() -> (TerminalPanel, gtk::Revealer) {
    let terminal_panel = TerminalPanel::new();
    
    // Connect events for the terminal
    terminal_panel.connect_events();
    
    // Get the revealer from the terminal panel
    let terminal_revealer = terminal_panel.widget.clone();
    
    // Set up the revealer
    terminal_revealer.set_reveal_child(true); // Visible when in bottom panel
    terminal_revealer.set_transition_type(gtk::RevealerTransitionType::SlideUp);
    terminal_revealer.set_transition_duration(300);
    
    // Set height for the terminal (scrolled window will handle the actual height)
    terminal_panel.output_view.set_height_request(300);
    
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
                
                // Focus the input entry when shown
                if let Some(terminal_panel) = &GLOBAL_TERMINAL_PANEL {
                    let panel = terminal_panel.borrow();
                    panel.input_entry.grab_focus();
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

pub fn focus_terminal_input() {
    if let Some(terminal_rc) = get_global_terminal_panel() {
        let terminal = terminal_rc.borrow();
        terminal.input_entry.grab_focus();
        crate::utils::simple_debug::debug_info("TERMINAL", "Terminal input focused");
    }
}
