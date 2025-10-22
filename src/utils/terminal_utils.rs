// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::process::Command;
use std::path::Path;
use crate::common::terminal_types::TerminalBackend;

/// Detect which terminal emulators are installed on the system
pub fn detect_installed_terminals() -> Vec<TerminalBackend> {
    let mut available = Vec::new();

    // Check for Wezterm
    if Command::new("wezterm")
        .arg("--version")
        .output()
        .is_ok()
    {
        available.push(TerminalBackend::Wezterm);
    }

    // Check for Alacritty
    if Command::new("alacritty")
        .arg("--version")
        .output()
        .is_ok()
    {
        available.push(TerminalBackend::Alacritty);
    }

    // Fallback is always available
    available.push(TerminalBackend::Fallback);

    available
}

/// Get the default shell for the system
pub fn get_default_shell() -> String {
    std::env::var("SHELL")
        .unwrap_or_else(|_| "/bin/bash".to_string())
}

/// Validate if a command is safe to execute
pub fn validate_command(cmd: &str) -> bool {
    // Basic validation - check for dangerous commands
    let dangerous_commands = [
        "rm -rf /",
        "sudo rm -rf",
        "dd if=",
        "mkfs",
        "fdisk",
        "parted",
    ];

    let cmd_lower = cmd.to_lowercase();
    !dangerous_commands.iter().any(|dangerous| cmd_lower.contains(dangerous))
}

/// Get terminal configuration directory
pub fn get_terminal_config_dir() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    std::path::PathBuf::from(home).join(".config").join("vortex-fm")
}

/// Create terminal configuration directory if it doesn't exist
pub fn ensure_terminal_config_dir() -> Result<std::path::PathBuf, String> {
    let config_dir = get_terminal_config_dir();
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    Ok(config_dir)
}

/// Check if a path is a valid directory
pub fn is_valid_directory(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// Get a safe working directory (fallback to home if invalid)
pub fn get_safe_working_directory(path: &Path) -> std::path::PathBuf {
    if is_valid_directory(path) {
        path.to_path_buf()
    } else {
        std::env::var("HOME")
            .map(|home| std::path::PathBuf::from(home))
            .unwrap_or_else(|_| std::path::PathBuf::from("/tmp"))
    }
}

/// Format terminal output for display
pub fn format_terminal_output(output: &str, max_width: usize) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let mut formatted = String::new();

    for line in lines {
        if line.len() > max_width {
            // Truncate long lines
            let truncated = &line[..max_width - 3];
            formatted.push_str(&format!("{}...\n", truncated));
        } else {
            formatted.push_str(&format!("{}\n", line));
        }
    }

    formatted
}

/// Get terminal theme based on system theme
pub fn get_system_terminal_theme() -> String {
    // Try to detect system theme preference
    if let Ok(theme) = std::env::var("GTK_THEME") {
        if theme.contains("dark") {
            return "dark".to_string();
        }
    }

    // Default to light theme
    "light".to_string()
}

/// Check if terminal supports colors
pub fn supports_colors() -> bool {
    std::env::var("TERM")
        .map(|term| !term.is_empty())
        .unwrap_or(false)
}

/// Get recommended font size based on system
pub fn get_recommended_font_size() -> u16 {
    // Try to get system font size preference
    if let Ok(dpi) = std::env::var("GDK_DPI_SCALE") {
        if let Ok(scale) = dpi.parse::<f32>() {
            return (14.0 * scale) as u16;
        }
    }

    // Default font size
    14
}
