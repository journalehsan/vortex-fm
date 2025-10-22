// Desktop environment theme detection and coordination

use cosmic::iced::Color;
use super::themes::{ThemeInfo, omarchy, kde, gnome, default, manager::{ThemeManager, ThemeStaged}, CosmicAccentPalette};
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
    
    // Check for Omarchy first (highest priority for theme customization)
    // If omarchy command exists, use Omarchy themes regardless of desktop
    if command_exists("omarchy-theme-current") {
        if xdg_desktop.contains("hyprland") {
            log::info!("Detected Hyprland with Omarchy themes available");
            return DesktopEnvironment::Omarchy;
        } else {
            log::info!("Detected Omarchy desktop environment");
            return DesktopEnvironment::Omarchy;
        }
    }
    
    // Check for Hyprland (without Omarchy)
    if xdg_desktop.contains("hyprland") {
        log::info!("Detected Hyprland desktop environment");
        return DesktopEnvironment::Hyprland;
    }
    
    // Check for Cosmic (fallback for theme customization)
    // cosmic-settings availability means we can use Cosmic theme system
    if xdg_desktop.contains("cosmic") || command_exists("cosmic-settings") {
        log::info!("Detected Cosmic desktop environment (cosmic-settings available)");
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
    
    // Check for common window manager environment variables
    let wm_session = std::env::var("XDG_SESSION_DESKTOP")
        .unwrap_or_default()
        .to_lowercase();
    
    // Common window managers that should hide window controls
    let is_wm_session = wm_session.contains("hyprland") || 
                       wm_session.contains("sway") || 
                       wm_session.contains("i3") || 
                       wm_session.contains("bspwm") || 
                       wm_session.contains("awesome") || 
                       wm_session.contains("dwm") || 
                       wm_session.contains("xmonad") ||
                       wm_session.contains("openbox") ||
                       wm_session.contains("fluxbox");
    
    // Treat as window manager if it's Hyprland, Omarchy, Unknown, or detected WM session
    let is_wm_desktop = matches!(desktop, DesktopEnvironment::Hyprland | DesktopEnvironment::Omarchy | DesktopEnvironment::Unknown);
    
    let result = is_wm_desktop || is_wm_session;
    
    if result {
        log::info!("🪟 Window manager detected - desktop: {:?}, session: {}", desktop, wm_session);
    }
    
    result
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

/// Global theme manager instance
use std::sync::Mutex;
use std::sync::OnceLock;

static THEME_MANAGER: OnceLock<Mutex<Option<ThemeManager>>> = OnceLock::new();

/// Initialize the global theme manager
fn init_theme_manager() -> &'static Mutex<Option<ThemeManager>> {
    THEME_MANAGER.get_or_init(|| Mutex::new(None))
}

/// Apply theme colors to cosmic theme using ThemeBuilder
pub fn apply_theme_to_cosmic(theme: &ThemeInfo) -> cosmic::theme::Theme {
    log::info!("🎨 Applying theme '{}' to Cosmic theme system", theme.name);
    log::info!("🎨 Theme properties - is_light: {}, window_bg: {:?}, view_bg: {:?}, accent: {:?}, fg: {:?}", 
        theme.is_light, theme.window_background, theme.view_background, theme.accent_color, theme.foreground);
    
    // Map the accent color to Cosmic palette
    let mapped_accent = CosmicAccentPalette::map_accent_color(
        theme.accent_color,
        !theme.is_light,
    );
    log::info!("🎨 Mapped accent color from {:?} to Cosmic palette: {:?}", 
        theme.accent_color, mapped_accent);
    
    // Use ThemeBuilder when cosmic-settings is available (works on any desktop)
    if command_exists("cosmic-settings") {
        log::info!("🎨 Using ThemeBuilder (cosmic-settings available)");
        
        let theme_manager_mutex = init_theme_manager();
        let mut theme_manager_guard = theme_manager_mutex.lock().unwrap();
        
        // Initialize theme manager if needed
        if theme_manager_guard.is_none() {
            let desktop = detect_desktop_environment();
            *theme_manager_guard = Some(ThemeManager::new(desktop));
        }
        
        if let Some(theme_manager) = theme_manager_guard.as_mut() {
            // Apply external theme colors (which will use Cosmic palette mapping internally)
            if let Err(err) = theme_manager.apply_external_theme(theme) {
                log::warn!("❌ Failed to apply external theme: {}", err);
            }
            
            // Build and return the custom theme
            return theme_manager.cosmic_theme();
        }
    }
    
    // Fallback for non-Cosmic environments
    log::info!("🎨 Using fallback theme system for non-Cosmic desktop");
    
    // Convert our Color to cosmic::iced::Color (use mapped accent)
    let accent_color = cosmic::iced::Color::from_rgb(
        mapped_accent.r,
        mapped_accent.g, 
        mapped_accent.b
    );
    
    log::info!("🎨 Detected accent color: {:?}", accent_color);
    log::info!("🎨 Target accent color RGB: ({}, {}, {})", 
        (mapped_accent.r * 255.0) as u8,
        (mapped_accent.g * 255.0) as u8,
        (mapped_accent.b * 255.0) as u8
    );
    
    // Apply the correct light/dark theme based on detected desktop theme
    let mut cosmic_theme = if theme.is_light {
        log::info!("🎨 Using light theme as base (detected light theme)");
        cosmic::theme::system_light()
    } else {
        log::info!("🎨 Using dark theme as base (detected dark theme)");
        cosmic::theme::system_dark()
    };
    
    // Set the theme type to match the detected desktop theme
    cosmic_theme.theme_type.prefer_dark(Some(!theme.is_light));
    log::info!("🌙 Set theme to prefer_dark: {} (theme is_light: {})", !theme.is_light, theme.is_light);
    
    // Log the current theme accent color for comparison
    log::info!("🎨 Current theme accent color: {:?}", cosmic_theme.cosmic().palette.accent_blue);
    log::info!("🎨 Current theme accent RGB: ({}, {}, {})", 
        (cosmic_theme.cosmic().palette.accent_blue.color.red * 255.0) as u8,
        (cosmic_theme.cosmic().palette.accent_blue.color.green * 255.0) as u8,
        (cosmic_theme.cosmic().palette.accent_blue.color.blue * 255.0) as u8
    );
    
    // Log our target colors
    let window_bg_color = cosmic::iced::Color::from_rgb(
        theme.window_background.r,
        theme.window_background.g,
        theme.window_background.b
    );
    log::info!("🎨 Target window background color: {:?}", window_bg_color);
    
    let text_color = cosmic::iced::Color::from_rgb(
        theme.foreground.r,
        theme.foreground.g,
        theme.foreground.b
    );
    log::info!("🎨 Target text color: {:?}", text_color);
    
    log::info!("✅ Applied light/dark theme preference: {} (is_light: {})", 
        if theme.is_light { "LIGHT" } else { "DARK" }, theme.is_light);
    
    log::info!("✅ Applied theme '{}' to Cosmic theme system with Cosmic palette mapping", theme.name);
    cosmic_theme
}

/// Apply a theme with advanced color customization
/// This function provides a high-level interface for applying themes with custom colors
pub fn apply_advanced_theme(theme: &ThemeInfo) -> cosmic::theme::Theme {
    log::info!("🎨 Applying advanced theme '{}'", theme.name);
    
    // Map accent color to Cosmic palette for consistency
    let mapped_accent = CosmicAccentPalette::map_accent_color(
        theme.accent_color,
        !theme.is_light,
    );
    log::info!("🎨 Mapped accent for advanced theme from {:?} to {:?}", 
        theme.accent_color, mapped_accent);
    
    // Use ThemeBuilder when cosmic-settings is available (works on any desktop)
    if command_exists("cosmic-settings") {
        log::info!("🎨 Using ThemeBuilder for advanced color customization (cosmic-settings available)");
        let theme_manager_mutex = init_theme_manager();
        let mut theme_manager_guard = theme_manager_mutex.lock().unwrap();
        
        // Initialize theme manager if needed
        if theme_manager_guard.is_none() {
            let desktop = detect_desktop_environment();
            *theme_manager_guard = Some(ThemeManager::new(desktop));
        }
        
        if let Some(theme_manager) = theme_manager_guard.as_mut() {
            // Apply the external theme with full color customization
            if let Err(err) = theme_manager.apply_external_theme(theme) {
                log::warn!("❌ Failed to apply advanced theme: {}", err);
                // Fallback to basic theme application
                return apply_theme_to_cosmic(theme);
            }
            
            // Build the theme with all customizations
            let _ = theme_manager.build_theme(ThemeStaged::Current);
            
            return theme_manager.cosmic_theme();
        }
        
        // Fallback if theme manager initialization failed
        apply_theme_to_cosmic(theme)
    } else {
        log::info!("🎨 Using standard theme application (cosmic-settings not available)");
        apply_theme_to_cosmic(theme)
    }
}

/// Get the current theme manager (for advanced customization)
/// Note: This returns a reference to the mutex, not the manager directly
pub fn get_theme_manager() -> Option<&'static Mutex<Option<ThemeManager>>> {
    log::info!("🔧 get_theme_manager called - using Cosmic theme system directly");
    
    // Always use theme manager when cosmic-settings is available
    // This ensures compatibility with all desktop environments where Cosmic apps work
    if command_exists("cosmic-settings") {
        log::info!("✅ Theme manager available (cosmic-settings detected)");
        Some(init_theme_manager())
    } else {
        log::warn!("❌ Theme manager not available (cosmic-settings not found)");
        None
    }
}

// Global state for custom theme name
static CUSTOM_THEME_NAME: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// Set the custom theme name globally
pub fn set_custom_theme_name(theme_name: String) {
    if let Ok(mut name) = CUSTOM_THEME_NAME.lock() {
        *name = Some(theme_name);
        log::info!("🎨 Custom theme name set globally");
        // Persist to disk so we can restore across restarts
        if let Err(err) = save_custom_theme_name_to_disk(name.as_ref().cloned()) {
            log::warn!("❌ Failed to persist custom theme name: {}", err);
        }
    }
}

/// Get the custom theme name globally
pub fn get_custom_theme_name() -> Option<String> {
    if let Ok(name) = CUSTOM_THEME_NAME.lock() {
        name.clone()
    } else {
        None
    }
}

/// Get custom theme info (similar to get_desktop_theme but for custom theme)
pub fn get_custom_theme() -> Option<ThemeInfo> {
    log::info!("🎨 get_custom_theme() called");
    if let Some(theme_name) = get_custom_theme_name() {
        log::info!("🎨 Getting custom theme: {}", theme_name);
        
        // Find the theme by name
        use crate::utils::themes::omarchy::OMARCHY_THEMES;
        if let Some(theme) = OMARCHY_THEMES.iter().find(|t| t.name == theme_name) {
            log::info!("✅ Found custom theme: {} (light: {})", theme.name, theme.is_light);
            
            // Convert OmarchyTheme to ThemeInfo
            let theme_info = ThemeInfo {
                name: theme.name.to_string(),
                is_light: theme.is_light,
                window_background: theme.window_background,
                view_background: theme.view_background,
                accent_color: theme.accent_color,
                foreground: theme.foreground,
            };
            
            return Some(theme_info);
        } else {
            log::warn!("❌ Custom theme not found: {}", theme_name);
            log::info!("📋 Available themes: {:?}", OMARCHY_THEMES.iter().map(|t| t.name).collect::<Vec<_>>());
        }
    } else {
        log::warn!("❌ No custom theme name set");
    }
    None
}

// --- Persistence helpers for custom theme name ---
fn config_dir() -> std::path::PathBuf {
    // Prefer XDG_CONFIG_HOME, fallback to ~/.config
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        return std::path::PathBuf::from(xdg).join("vortex-fm");
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("."));
    std::path::PathBuf::from(home).join(".config").join("vortex-fm")
}

pub fn load_custom_theme_name_from_disk() -> Option<String> {
    let path = config_dir().join("custom_theme.txt");
    match std::fs::read_to_string(&path) {
        Ok(s) => {
            let name = s.trim().to_string();
            if name.is_empty() {
                None
            } else {
                log::info!("📄 Loaded custom theme from disk: {}", name);
                Some(name)
            }
        }
        Err(err) => {
            log::info!("ℹ️ No persisted custom theme found at {:?}: {}", path, err);
            None
        }
    }
}

fn save_custom_theme_name_to_disk(name_opt: Option<String>) -> Result<(), String> {
    let dir = config_dir();
    if let Err(err) = std::fs::create_dir_all(&dir) {
        return Err(format!("create_dir_all {:?}: {}", dir, err));
    }
    let path = dir.join("custom_theme.txt");
    match name_opt {
        Some(name) => std::fs::write(&path, name).map_err(|e| e.to_string()),
        None => std::fs::remove_file(&path).map_err(|e| e.to_string()),
    }
}
