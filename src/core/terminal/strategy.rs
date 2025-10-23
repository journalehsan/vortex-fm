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
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        working_dir.hash(&mut hasher);
        let hash = hasher.finish() as u32;
        
        let session_id = format!("wezterm_{}_{}", std::process::id(), hash);
        
        log::info!("🖥️ Spawning Wezterm in {}", working_dir.display());
        
        // Spawn Wezterm with working directory using new-window command
        let child = Command::new("wezterm")
            .arg("cli")
            .arg("spawn")
            .arg("--cwd")
            .arg(working_dir)
            .spawn()
            .map_err(|e| {
                log::error!("❌ Failed to spawn wezterm: {}", e);
                format!("Failed to spawn wezterm: {}", e)
            })?;

        let mut session = TerminalSession::new(
            session_id,
            working_dir.to_path_buf(),
            TerminalBackend::Wezterm,
        );
        session.process_id = Some(child.id());

        log::info!("✅ Wezterm spawned with PID: {:?}", child.id());
        Ok(session)
    }

    fn send_command(&self, session: &mut TerminalSession, command: &str) -> Result<(), String> {
        // For external Wezterm, we can't directly send text through the CLI
        // Instead, just log that command would be sent
        log::debug!("📤 Would send to Wezterm: {}", command);
        
        // In a real implementation, you might:
        // 1. Use Wezterm's socket protocol
        // 2. Communicate via environment variables
        // 3. Use the mux server
        
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
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        working_dir.hash(&mut hasher);
        let hash = hasher.finish() as u32;
        
        let session_id = format!("alacritty_{}_{}", std::process::id(), hash);
        
        log::info!("🖥️ Spawning Alacritty in {}", working_dir.display());
        
        // Spawn alacritty with working directory
        let child = Command::new("alacritty")
            .arg("--working-directory")
            .arg(working_dir)
            .spawn()
            .map_err(|e| {
                log::error!("❌ Failed to spawn alacritty: {}", e);
                format!("Failed to spawn alacritty: {}", e)
            })?;

        let mut session = TerminalSession::new(
            session_id,
            working_dir.to_path_buf(),
            TerminalBackend::Alacritty,
        );
        session.process_id = Some(child.id());

        log::info!("✅ Alacritty spawned with PID: {:?}", child.id());
        Ok(session)
    }

    fn send_command(&self, _session: &mut TerminalSession, _command: &str) -> Result<(), String> {
        // Alacritty doesn't support IPC for command sending
        // External terminals are meant to be interactive, not programmatically controlled
        log::debug!("📤 Alacritty is external terminal - no IPC available");
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
    pub fn create_best_strategy() -> Box<dyn TerminalStrategy> {
        // Try Wezterm first (best features, multiplexing support)
        let wezterm = WeztermStrategy::new();
        if wezterm.is_available() {
            log::info!("📺 Using Wezterm terminal strategy");
            return Box::new(wezterm);
        }

        // Try Alacritty second (GPU accelerated, fast)
        let alacritty = AlacrittyStrategy::new();
        if alacritty.is_available() {
            log::info!("📺 Using Alacritty terminal strategy");
            return Box::new(alacritty);
        }

        // Fallback to text-based terminal (always available)
        log::info!("📺 Using Fallback terminal strategy (no Wezterm/Alacritty found)");
        Box::new(FallbackStrategy::new())
    }

    /// Create a specific strategy
    pub fn create_strategy(backend: TerminalBackend) -> Box<dyn TerminalStrategy> {
        match backend {
            TerminalBackend::Wezterm => {
                let strategy = WeztermStrategy::new();
                if strategy.is_available() {
                    Box::new(strategy)
                } else {
                    log::warn!("⚠️ Wezterm not found, using Fallback");
                    Box::new(FallbackStrategy::new())
                }
            }
            TerminalBackend::Alacritty => {
                let strategy = AlacrittyStrategy::new();
                if strategy.is_available() {
                    Box::new(strategy)
                } else {
                    log::warn!("⚠️ Alacritty not found, using Fallback");
                    Box::new(FallbackStrategy::new())
                }
            }
            TerminalBackend::Fallback => Box::new(FallbackStrategy::new()),
        }
    }
}
