// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

/// Position of the terminal panel in the UI
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalPosition {
    Bottom,
    Right,
}

impl Default for TerminalPosition {
    fn default() -> Self {
        Self::Bottom
    }
}

/// Available terminal backends in order of preference
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalBackend {
    Wezterm,
    Alacritty,
    Fallback,
}

/// Messages for terminal widget communication
#[derive(Clone, Debug)]
pub enum TerminalMessage {
    ToggleVisibility,
    TogglePosition,
    SyncDirectory,
    ExecuteCommand(String),
    CommandInput(String),
    CommandSubmit,
    OutputReceived(String),
    SessionStarted,
    SessionEnded,
    Error(String),
}

/// Terminal session state
#[derive(Clone, Debug)]
pub struct TerminalSession {
    pub id: String,
    pub working_directory: PathBuf,
    pub is_busy: bool,
    pub backend: TerminalBackend,
    pub process_id: Option<u32>,
}

impl TerminalSession {
    pub fn new(id: String, working_directory: PathBuf, backend: TerminalBackend) -> Self {
        Self {
            id,
            working_directory,
            is_busy: false,
            backend,
            process_id: None,
        }
    }

    pub fn set_busy(&mut self, busy: bool) {
        self.is_busy = busy;
    }

    pub fn update_directory(&mut self, new_dir: PathBuf) {
        if !self.is_busy {
            self.working_directory = new_dir;
        }
    }
}

/// Terminal configuration
#[derive(Clone, Debug)]
pub struct TerminalConfig {
    pub position: TerminalPosition,
    pub auto_sync: bool,
    pub font_size: u16,
    pub theme: String,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            position: TerminalPosition::default(),
            auto_sync: false,
            font_size: 14,
            theme: "default".to_string(),
        }
    }
}

/// Terminal output line for fallback terminal
#[derive(Clone, Debug)]
pub struct TerminalOutputLine {
    pub content: String,
    pub is_error: bool,
    pub timestamp: std::time::SystemTime,
}

impl TerminalOutputLine {
    pub fn new(content: String, is_error: bool) -> Self {
        Self {
            content,
            is_error,
            timestamp: std::time::SystemTime::now(),
        }
    }
}
