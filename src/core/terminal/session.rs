// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;
use std::process::Child;
use tokio::process::Command as AsyncCommand;

use crate::common::terminal_types::{TerminalBackend, TerminalSession, TerminalOutputLine};

/// Terminal session manager
pub struct TerminalSessionManager {
    session: Option<TerminalSession>,
    process: Option<Child>,
    output_lines: Vec<TerminalOutputLine>,
    command_history: Vec<String>,
}

impl TerminalSessionManager {
    pub fn new() -> Self {
        Self {
            session: None,
            process: None,
            output_lines: Vec::new(),
            command_history: Vec::new(),
        }
    }

    /// Start a new terminal session
    pub fn start_session(&mut self, working_dir: PathBuf, backend: TerminalBackend) -> Result<(), String> {
        let session_id = format!("session_{}", std::process::id());
        let session = TerminalSession::new(session_id, working_dir, backend);
        
        self.session = Some(session);
        Ok(())
    }

    /// Execute a command in the terminal session
    pub async fn execute_command(&mut self, command: &str) -> Result<(), String> {
        let (backend, working_dir) = if let Some(session) = &self.session {
            if session.is_busy {
                return Err("Terminal is busy".to_string());
            }
            (session.backend.clone(), session.working_directory.clone())
        } else {
            return Err("No active session".to_string());
        };

        // Add to command history
        self.command_history.push(command.to_string());

        // Mark session as busy
        if let Some(session) = &mut self.session {
            session.set_busy(true);
        }

        // Execute command based on backend
        match backend {
            TerminalBackend::Fallback => {
                self.execute_fallback_command(command, &working_dir).await?;
            }
            _ => {
                // For other backends, we'd need to implement proper IPC
                // For now, just simulate execution
                self.add_output_line(format!("$ {}", command), false);
                self.add_output_line("Command executed (simulated)".to_string(), false);
            }
        }

        // Mark session as not busy
        if let Some(session) = &mut self.session {
            session.set_busy(false);
        }

        Ok(())
    }

    /// Execute command using fallback method (tokio)
    async fn execute_fallback_command(&mut self, command: &str, working_dir: &PathBuf) -> Result<(), String> {
        self.add_output_line(format!("$ {}", command), false);

        let output = AsyncCommand::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(working_dir)
            .output()
            .await
            .map_err(|e| format!("Command execution failed: {}", e))?;

        // Add stdout
        if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                self.add_output_line(line.to_string(), false);
            }
        }

        // Add stderr
        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stderr.lines() {
                self.add_output_line(line.to_string(), true);
            }
        }

        // Add exit status
        let exit_code = output.status.code().unwrap_or(-1);
        if exit_code != 0 {
            self.add_output_line(format!("[Exit code: {}]", exit_code), true);
        }

        Ok(())
    }

    /// Add output line to the buffer
    fn add_output_line(&mut self, content: String, is_error: bool) {
        let line = TerminalOutputLine::new(content, is_error);
        self.output_lines.push(line);
        
        // Keep only last 1000 lines to prevent memory issues
        if self.output_lines.len() > 1000 {
            self.output_lines.remove(0);
        }
    }

    /// Get current output lines
    pub fn get_output_lines(&self) -> &[TerminalOutputLine] {
        &self.output_lines
    }

    /// Get command history
    pub fn get_command_history(&self) -> &[String] {
        &self.command_history
    }

    /// Check if session is busy
    pub fn is_busy(&self) -> bool {
        self.session.as_ref().map_or(false, |s| s.is_busy)
    }

    /// Update working directory
    pub fn update_directory(&mut self, new_dir: PathBuf) -> Result<(), String> {
        if let Some(session) = &mut self.session {
            if session.is_busy {
                return Err("Cannot change directory while terminal is busy".to_string());
            }
            session.update_directory(new_dir);
        }
        Ok(())
    }

    /// Get current working directory
    pub fn current_directory(&self) -> Option<&PathBuf> {
        self.session.as_ref().map(|s| &s.working_directory)
    }

    /// End the current session
    pub fn end_session(&mut self) {
        if let Some(process) = &mut self.process {
            let _ = process.kill();
        }
        self.session = None;
        self.process = None;
    }

    /// Get session info
    pub fn get_session(&self) -> Option<&TerminalSession> {
        self.session.as_ref()
    }
}
