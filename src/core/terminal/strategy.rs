// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::Path;
use std::process::Command;
use tokio::process::Command as AsyncCommand;

use crate::common::terminal_types::{TerminalBackend, TerminalSession};

/// Trait for terminal backend strategies
pub trait TerminalStrategy: Send + Sync {
    /// Check if this terminal backend is available on the system
    fn is_available(&self) -> bool;
    
    /// Spawn a new terminal session
    fn spawn(&self, working_dir: &Path) -> Result<TerminalSession, String>;
    
    /// Send a command to the terminal session
    fn send_command(&self, session: &mut TerminalSession, command: &str) -> Result<(), String>;
    
    /// Check if the terminal session is currently busy
    fn is_busy(&self, session: &TerminalSession) -> bool;
    
    /// Get the backend type
    fn backend_type(&self) -> TerminalBackend;
}

/// Wezterm terminal strategy (Priority 1)
pub struct WeztermStrategy;

impl WeztermStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl TerminalStrategy for WeztermStrategy {
    fn is_available(&self) -> bool {
        Command::new("wezterm")
            .arg("--version")
            .output()
            .is_ok()
    }

    fn spawn(&self, working_dir: &Path) -> Result<TerminalSession, String> {
        let session_id = format!("wezterm_{}", std::process::id());
        
        // Try to spawn wezterm with socket support
        let output = Command::new("wezterm")
            .arg("cli")
            .arg("spawn")
            .arg("--cwd")
            .arg(working_dir)
            .output()
            .map_err(|e| format!("Failed to spawn wezterm: {}", e))?;

        if !output.status.success() {
            return Err("Wezterm spawn failed".to_string());
        }

        let session = TerminalSession::new(
            session_id,
            working_dir.to_path_buf(),
            TerminalBackend::Wezterm,
        );

        Ok(session)
    }

    fn send_command(&self, session: &mut TerminalSession, command: &str) -> Result<(), String> {
        // Send command via wezterm CLI
        let output = Command::new("wezterm")
            .arg("cli")
            .arg("send-text")
            .arg("--pane-id")
            .arg(&session.id)
            .arg(command)
            .output()
            .map_err(|e| format!("Failed to send command: {}", e))?;

        if !output.status.success() {
            return Err("Command send failed".to_string());
        }

        Ok(())
    }

    fn is_busy(&self, session: &TerminalSession) -> bool {
        session.is_busy
    }

    fn backend_type(&self) -> TerminalBackend {
        TerminalBackend::Wezterm
    }
}

/// Alacritty terminal strategy (Priority 2)
pub struct AlacrittyStrategy;

impl AlacrittyStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl TerminalStrategy for AlacrittyStrategy {
    fn is_available(&self) -> bool {
        Command::new("alacritty")
            .arg("--version")
            .output()
            .is_ok()
    }

    fn spawn(&self, working_dir: &Path) -> Result<TerminalSession, String> {
        let session_id = format!("alacritty_{}", std::process::id());
        
        // Spawn alacritty with working directory
        let child = Command::new("alacritty")
            .arg("--working-directory")
            .arg(working_dir)
            .spawn()
            .map_err(|e| format!("Failed to spawn alacritty: {}", e))?;

        let mut session = TerminalSession::new(
            session_id,
            working_dir.to_path_buf(),
            TerminalBackend::Alacritty,
        );
        session.process_id = Some(child.id());

        Ok(session)
    }

    fn send_command(&self, _session: &mut TerminalSession, _command: &str) -> Result<(), String> {
        // Alacritty doesn't have direct command sending, so we simulate it
        // In a real implementation, you'd need to use alacritty's socket or other IPC
        Ok(())
    }

    fn is_busy(&self, session: &TerminalSession) -> bool {
        session.is_busy
    }

    fn backend_type(&self) -> TerminalBackend {
        TerminalBackend::Alacritty
    }
}

/// Fallback terminal strategy using text input and tokio
pub struct FallbackStrategy {
    output_buffer: Vec<String>,
}

impl FallbackStrategy {
    pub fn new() -> Self {
        Self {
            output_buffer: Vec::new(),
        }
    }
}

impl TerminalStrategy for FallbackStrategy {
    fn is_available(&self) -> bool {
        true // Always available as fallback
    }

    fn spawn(&self, working_dir: &Path) -> Result<TerminalSession, String> {
        let session_id = format!("fallback_{}", std::process::id());
        
        let session = TerminalSession::new(
            session_id,
            working_dir.to_path_buf(),
            TerminalBackend::Fallback,
        );

        Ok(session)
    }

    fn send_command(&self, session: &mut TerminalSession, command: &str) -> Result<(), String> {
        // For fallback, we'll execute the command asynchronously
        let working_dir = session.working_directory.clone();
        let command = command.to_string();
        
        tokio::spawn(async move {
            let output = AsyncCommand::new("sh")
                .arg("-c")
                .arg(&command)
                .current_dir(&working_dir)
                .output()
                .await;

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    
                    if !stdout.is_empty() {
                        println!("STDOUT: {}", stdout);
                    }
                    if !stderr.is_empty() {
                        eprintln!("STDERR: {}", stderr);
                    }
                }
                Err(e) => {
                    eprintln!("Command execution failed: {}", e);
                }
            }
        });

        Ok(())
    }

    fn is_busy(&self, session: &TerminalSession) -> bool {
        session.is_busy
    }

    fn backend_type(&self) -> TerminalBackend {
        TerminalBackend::Fallback
    }
}

/// Terminal strategy factory
pub struct TerminalStrategyFactory;

impl TerminalStrategyFactory {
    /// Create the best available terminal strategy
    /// Currently only returns Fallback for stability
    pub fn create_best_strategy() -> Box<dyn TerminalStrategy> {
        // For now, always use fallback to ensure text-based UI works properly
        // TODO: In future, detect and embed Wezterm/Alacritty if available
        log::debug!("📺 Using Fallback terminal strategy");
        Box::new(FallbackStrategy::new())
    }

    /// Create a specific strategy
    pub fn create_strategy(backend: TerminalBackend) -> Box<dyn TerminalStrategy> {
        match backend {
            TerminalBackend::Wezterm => {
                log::warn!("⚠️ Wezterm embedding not yet implemented, using Fallback");
                Box::new(FallbackStrategy::new())
            }
            TerminalBackend::Alacritty => {
                log::warn!("⚠️ Alacritty embedding not yet implemented, using Fallback");
                Box::new(FallbackStrategy::new())
            }
            TerminalBackend::Fallback => Box::new(FallbackStrategy::new()),
        }
    }
}
