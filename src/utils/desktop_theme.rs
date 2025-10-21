// Desktop environment theme detection and coordination

use cosmic::iced::Color;
use super::themes::{ThemeInfo, DesktopTheme, omarchy, kde, gnome, default};
use crate::utils::command_utils::SafeCommand;

/// Desktop environment types
#[derive(Debug, Clone, PartialEq)]
pub enum DesktopEnvironment {
    Omarchy,
    Hyprland,
    Cosmic,
    Kde,
    Gnome,
    Unknown,
}

/// Detect the current desktop environment
pub fn detect_desktop_environment() -> DesktopEnvironment {
    let xdg_desktop = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase();
    
    log::info!("XDG_CURRENT_DESKTOP: {}", xdg_desktop);
    
    // Check for Hyprland first, but if omarchy command exists, use Omarchy themes
    if xdg_desktop.contains("hyprland") {
        if command_exists("omarchy-theme-current") {
            log::info!("Detected Hyprland with Omarchy themes available");
            return DesktopEnvironment::Omarchy;
        } else {
            log::info!("Detected Hyprland desktop environment");
            return DesktopEnvironment::Hyprland;
        }
    }
    
    // Check for Omarchy (for other desktop environments)
    if command_exists("omarchy-theme-current") {
        log::info!("Detected Omarchy desktop environment");
        return DesktopEnvironment::Omarchy;
    }
    
    // Check for Cosmic
    if xdg_desktop.contains("cosmic") {
        log::info!("Detected Cosmic desktop environment");
        return DesktopEnvironment::Cosmic;
    }
    
    // Check for KDE
    if xdg_desktop.contains("kde") {
        log::info!("Detected KDE desktop environment");
        return DesktopEnvironment::Kde;
    }
    
    // Check for GNOME
    if xdg_desktop.contains("gnome") || xdg_desktop.contains("unity") {
        log::info!("Detected GNOME desktop environment");
        return DesktopEnvironment::Gnome;
    }
    
    log::info!("Unknown desktop environment, defaulting to Unknown");
    DesktopEnvironment::Unknown
}

/// Check if a command exists in the system
fn command_exists(command: &str) -> bool {
    SafeCommand::new("which")
        .args([command])
        .status()
        .unwrap_or(false)
}

/// Detect if running under a window manager (not a full desktop environment)
pub fn is_window_manager() -> bool {
    let desktop = detect_desktop_environment();
    // Treat as window manager if it's Hyprland, Omarchy, or Unknown desktop environment
    matches!(desktop, DesktopEnvironment::Hyprland | DesktopEnvironment::Omarchy | DesktopEnvironment::Unknown)
}

/// Get theme information for the current desktop environment
pub fn get_desktop_theme() -> ThemeInfo {
    let desktop = detect_desktop_environment();
    
    match desktop {
        DesktopEnvironment::Omarchy => {
            log::info!("🎭 Detecting Omarchy theme...");
            if let Some(theme) = omarchy::detect_omarchy_theme() {
                log::info!("✅ Successfully detected Omarchy theme: '{}' (light: {})", theme.name, theme.is_light);
                log::info!("🎨 Theme colors - window_bg: {:?}, view_bg: {:?}, accent: {:?}, fg: {:?}", 
                    theme.window_background, theme.view_background, theme.accent_color, theme.foreground);
                return theme;
            }
            log::warn!("❌ Failed to detect Omarchy theme, using fallback");
        }
        DesktopEnvironment::Hyprland => {
            // Try Omarchy first for Hyprland
            log::info!("🪟 Detecting theme for Hyprland (trying Omarchy first)...");
            if let Some(theme) = omarchy::detect_omarchy_theme() {
                log::info!("✅ Successfully detected Omarchy theme for Hyprland: '{}' (light: {})", theme.name, theme.is_light);
                log::info!("🎨 Theme colors - window_bg: {:?}, view_bg: {:?}, accent: {:?}, fg: {:?}", 
                    theme.window_background, theme.view_background, theme.accent_color, theme.foreground);
                return theme;
            }
            log::warn!("❌ No Omarchy theme detected for Hyprland, using fallback");
        }
        DesktopEnvironment::Cosmic => {
            // Cosmic has its own theming, use system preference
            log::info!("Using Cosmic system theme");
            return get_system_theme();
        }
        DesktopEnvironment::Kde => {
            if let Some(theme) = kde::detect_kde_theme() {
                log::info!("Detected KDE theme: {}", theme.name);
                return theme;
            }
            log::warn!("Failed to detect KDE theme, using fallback");
        }
        DesktopEnvironment::Gnome => {
            if let Some(theme) = gnome::detect_gnome_theme() {
                log::info!("Detected GNOME theme: {}", theme.name);
                return theme;
            }
            log::warn!("Failed to detect GNOME theme, using fallback");
        }
        DesktopEnvironment::Unknown => {
            log::warn!("Unknown desktop environment, using fallback");
        }
    }
    
    // Fallback to default theme
    default::get_default_theme()
}

/// Get system theme preference (dark/light)
fn get_system_theme() -> ThemeInfo {
    // Try to detect system dark mode preference
    let is_dark = detect_system_dark_mode();
    
    if is_dark {
        ThemeInfo::new(
            "System Dark".to_string(),
            false,
            Color::from_rgb(0.18, 0.18, 0.18), // Dark background
            Color::from_rgb(0.24, 0.24, 0.24), // Dark container
            Color::from_rgb(0.24, 0.60, 0.89), // Blue accent
            Color::from_rgb(0.88, 0.88, 0.88), // Light text
        )
    } else {
        ThemeInfo::new(
            "System Light".to_string(),
            true,
            Color::from_rgb(0.98, 0.98, 0.98), // Light background
            Color::from_rgb(1.0, 1.0, 1.0),   // White container
            Color::from_rgb(0.0, 0.46, 0.85), // Blue accent
            Color::from_rgb(0.13, 0.13, 0.13), // Dark text
        )
    }
}

/// Detect system dark mode preference
fn detect_system_dark_mode() -> bool {
    // Try gsettings for GNOME/COSMIC
    if let Ok(output) = SafeCommand::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "color-scheme"])
        .output_text()
    {
        if output.contains("dark") {
            return true;
        }
    }
    
    // Try KDE
    if let Ok(output) = SafeCommand::new("kreadconfig5")
        .args(["--file", "kdeglobals", "--group", "General", "--key", "ColorScheme"])
        .output_text()
    {
        if output.to_lowercase().contains("dark") {
            return true;
        }
    }
    
    // Check environment variables
    if let Ok(theme) = std::env::var("GTK_THEME") {
        if theme.to_lowercase().contains("dark") {
            return true;
        }
    }
    
    // Default to light mode
    false
}

/// Apply theme colors to cosmic theme
pub fn apply_theme_to_cosmic(theme: &ThemeInfo) -> cosmic::theme::Theme {
    log::info!("🎨 Applying theme '{}' to Cosmic theme system", theme.name);
    log::info!("🎨 Theme properties - is_light: {}, window_bg: {:?}, view_bg: {:?}, accent: {:?}, fg: {:?}", 
        theme.is_light, theme.window_background, theme.view_background, theme.accent_color, theme.foreground);
    
    let mut cosmic_theme = cosmic::theme::system_preference();
    
    // Set theme type based on light/dark
    let prefer_dark = !theme.is_light();
    cosmic_theme.theme_type.prefer_dark(Some(prefer_dark));
    
    log::info!("🌙 Set Cosmic theme to prefer_dark: {}", prefer_dark);
    
    // For now, just return the system preference theme
    // Custom color application would require more complex cosmic theme manipulation
    log::info!("✅ Applied theme '{}' to Cosmic theme system", theme.name);
    cosmic_theme
}
