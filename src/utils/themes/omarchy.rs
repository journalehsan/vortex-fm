// Omarchy theme definitions and detection

use cosmic::iced::Color;
use super::{DesktopTheme, ThemeInfo, cosmic_palette::CosmicAccentPalette};
use crate::utils::command_utils::SafeCommand;

/// Omarchy theme definitions
pub struct OmarchyTheme {
    pub name: &'static str,
    pub is_light: bool,
    pub window_background: Color,
    pub view_background: Color,
    pub accent_color: Color,
    pub foreground: Color,
}

impl DesktopTheme for OmarchyTheme {
    fn name(&self) -> &str {
        self.name
    }
    
    fn is_light(&self) -> bool {
        self.is_light
    }
    
    fn window_background(&self) -> Color {
        self.window_background
    }
    
    fn view_background(&self) -> Color {
        self.view_background
    }
    
    fn accent_color(&self) -> Color {
        // Map the accent color to the nearest Cosmic accent for consistency
        CosmicAccentPalette::map_accent_color(self.accent_color, !self.is_light)
    }
    
    fn foreground(&self) -> Color {
        self.foreground
    }
}

/// All available Omarchy themes
pub const OMARCHY_THEMES: &[OmarchyTheme] = &[
    // Dark themes
    OmarchyTheme {
        name: "catppuccin",
        is_light: false,
        window_background: Color::from_rgb(0.11, 0.11, 0.15), // #1E1E2E
        view_background: Color::from_rgb(0.15, 0.15, 0.20),   // #24273A
        accent_color: Color::from_rgb(0.24, 0.60, 0.89),     // #3E9BFF -> will map to Cosmic blue
        foreground: Color::from_rgb(0.91, 0.89, 0.84),        // #E8E4E0
    },
    OmarchyTheme {
        name: "dracula",
        is_light: false,  // Dracula is a dark theme
        window_background: Color::from_rgb(0.100, 0.101, 0.150), //rgb(11, 11, 15)
        view_background: Color::from_rgb(0.16, 0.16, 0.20),   // #282A36
        accent_color: Color::from_rgb(0.100, 0.20, 0.50),     // #E49BF8 -> will map to Cosmic purple
        foreground: Color::from_rgb(0.95, 0.95, 0.95),        // #F2F2F2
    },
    OmarchyTheme {
        name: "everforest",
        is_light: false,
        window_background: Color::from_rgb(0.20, 0.24, 0.20), // #323D43
        view_background: Color::from_rgb(0.25, 0.30, 0.25),   // #3F4944
        accent_color: Color::from_rgb(0.40, 0.95, 0.60),     // #66FF99 -> will map to Cosmic green
        foreground: Color::from_rgb(0.85, 0.85, 0.80),        // #D8D8D0
    },
    OmarchyTheme {
        name: "gruvbox",
        is_light: false,
        window_background: Color::from_rgb(0.15, 0.13, 0.10), // #262626
        view_background: Color::from_rgb(0.20, 0.18, 0.15),    // #32302F
        accent_color: Color::from_rgb(0.85, 0.55, 0.20),      // #D79921 -> will map to Cosmic orange
        foreground: Color::from_rgb(0.85, 0.80, 0.70),        // #D5C4A1
    },
    OmarchyTheme {
        name: "kanagawa",
        is_light: false,
        window_background: Color::from_rgb(0.09, 0.09, 0.12), // #16161D
        view_background: Color::from_rgb(0.12, 0.12, 0.16),   // #1F1F28
        accent_color: Color::from_rgb(0.24, 0.60, 0.89),     // #3E9BFF -> will map to Cosmic blue
        foreground: Color::from_rgb(0.85, 0.85, 0.80),        // #D8D8D0
    },
    OmarchyTheme {
        name: "matte-black",
        is_light: false,
        window_background: Color::from_rgb(0.05, 0.05, 0.05), // #0D0D0D
        view_background: Color::from_rgb(0.10, 0.10, 0.10),   // #1A1A1A
        accent_color: Color::from_rgb(0.24, 0.60, 0.89),     // #3E9BFF -> will map to Cosmic blue
        foreground: Color::from_rgb(0.90, 0.90, 0.90),        // #E6E6E6
    },
    OmarchyTheme {
        name: "nord",
        is_light: false,
        window_background: Color::from_rgb(0.15, 0.18, 0.22), // #242933
        view_background: Color::from_rgb(0.18, 0.22, 0.27),   // #2E3440
        accent_color: Color::from_rgb(0.24, 0.60, 0.89),     // #3E9BFF -> will map to Cosmic blue
        foreground: Color::from_rgb(0.85, 0.85, 0.80),        // #D8D8D0
    },
    OmarchyTheme {
        name: "osaka-jade",
        is_light: false,
        window_background: Color::from_rgb(0.05, 0.10, 0.08), // #0D1A14
        view_background: Color::from_rgb(0.08, 0.15, 0.12),   // #14261F
        accent_color: Color::from_rgb(0.20, 0.80, 0.40),     // #33CC66 -> will map to Cosmic green
        foreground: Color::from_rgb(0.85, 0.90, 0.85),        // #D9E6D9
    },
    OmarchyTheme {
        name: "ristretto",
        is_light: false,
        window_background: Color::from_rgb(0.10, 0.08, 0.08), // #1A1414
        view_background: Color::from_rgb(0.15, 0.12, 0.12),   // #261F1F
        accent_color: Color::from_rgb(0.80, 0.40, 0.40),     // #CC6666 -> will map to Cosmic red
        foreground: Color::from_rgb(0.85, 0.80, 0.80),        // #D9CCCC
    },
    OmarchyTheme {
        name: "tokyo-night",
        is_light: false,
        window_background: Color::from_rgb(0.08, 0.08, 0.12), // #14141F
        view_background: Color::from_rgb(0.12, 0.12, 0.18),   // #1F1F2E
        accent_color: Color::from_rgb(0.24, 0.60, 0.89),     // #3E9BFF -> will map to Cosmic blue
        foreground: Color::from_rgb(0.85, 0.85, 0.80),        // #D8D8D0
    },
    // Light themes
    OmarchyTheme {
        name: "catppuccin-latte",
        is_light: true,
        window_background: Color::from_rgb(0.95, 0.95, 0.95), // #F2F2F2
        view_background: Color::from_rgb(1.0, 1.0, 1.0),      // #FFFFFF
        accent_color: Color::from_rgb(0.24, 0.60, 0.89),     // #3E9BFF -> will map to Cosmic blue
        foreground: Color::from_rgb(0.20, 0.20, 0.20),        // #333333
    },
    OmarchyTheme {
        name: "rose-pine",
        is_light: true,
        window_background: Color::from_rgb(0.95, 0.95, 0.95), // #F2F2F2
        view_background: Color::from_rgb(1.0, 1.0, 1.0),      // #FFFFFF
        accent_color: Color::from_rgb(0.60, 0.40, 0.60),     // #996699 -> will map to Cosmic purple
        foreground: Color::from_rgb(0.20, 0.20, 0.20),        // #333333
    },
];

/// Detect current Omarchy theme
pub fn detect_omarchy_theme() -> Option<ThemeInfo> {
    log::info!("🎭 Starting Omarchy theme detection...");
    
    // Check if omarchy-theme-current command exists
    let output = SafeCommand::new("omarchy-theme-current")
        .output_text()
        .ok()?;
    
    let theme_name = output.trim();
    log::info!("🎨 Omarchy command output: '{}'", theme_name);
    
    if theme_name.is_empty() {
        log::warn!("⚠️ Omarchy command returned empty output");
        return None;
    }
    
    log::info!("🔍 Looking for theme: '{}' in {} available themes", theme_name, OMARCHY_THEMES.len());
    
    // Find matching theme (case-insensitive)
    for theme in OMARCHY_THEMES {
        log::debug!("🔍 Checking theme: '{}' against '{}'", theme.name, theme_name);
        if theme.name.to_lowercase() == theme_name.to_lowercase() {
            // Map accent color to Cosmic palette
            let mapped_accent = theme.accent_color();
            
            log::info!("✅ Found matching Omarchy theme: '{}' (light: {})", theme.name, theme.is_light);
            log::info!("🎨 Original accent color: {:?}", theme.accent_color);
            log::info!("🎨 Mapped to Cosmic accent: {:?}", mapped_accent);
            log::info!("🎨 Theme colors - window_bg: {:?}, view_bg: {:?}, accent: {:?}, fg: {:?}", 
                theme.window_background, theme.view_background, mapped_accent, theme.foreground);
            
            let theme_info = ThemeInfo::new(
                theme.name.to_string(),
                theme.is_light,
                theme.window_background,
                theme.view_background,
                mapped_accent,
                theme.foreground,
            );
            
            log::info!("🎭 Successfully created ThemeInfo for '{}'", theme.name);
            return Some(theme_info);
        }
    }
    
    // If theme not found, return None
    log::warn!("❌ Unknown Omarchy theme: '{}'", theme_name);
    log::info!("📋 Available themes: {:?}", OMARCHY_THEMES.iter().map(|t| t.name).collect::<Vec<_>>());
    None
}
