// Default fallback theme

use cosmic::iced::Color;
use super::{DesktopTheme, ThemeInfo};

/// Default light theme
pub struct DefaultLightTheme;

impl DesktopTheme for DefaultLightTheme {
    fn name(&self) -> &str {
        "Default Light"
    }
    
    fn is_light(&self) -> bool {
        true
    }
    
    fn window_background(&self) -> Color {
        Color::from_rgb(0.98, 0.98, 0.98) // #FAFAFA
    }
    
    fn view_background(&self) -> Color {
        Color::from_rgb(1.0, 1.0, 1.0) // #FFFFFF
    }
    
    fn accent_color(&self) -> Color {
        Color::from_rgb(0.0, 0.46, 0.85) // #0078D7
    }
    
    fn foreground(&self) -> Color {
        Color::from_rgb(0.13, 0.13, 0.13) // #212121
    }
}

/// Default dark theme
pub struct DefaultDarkTheme;

impl DesktopTheme for DefaultDarkTheme {
    fn name(&self) -> &str {
        "Default Dark"
    }
    
    fn is_light(&self) -> bool {
        false
    }
    
    fn window_background(&self) -> Color {
        Color::from_rgb(0.18, 0.18, 0.18) // #2E2E2E
    }
    
    fn view_background(&self) -> Color {
        Color::from_rgb(0.24, 0.24, 0.24) // #3D3D3D
    }
    
    fn accent_color(&self) -> Color {
        Color::from_rgb(0.24, 0.68, 0.91) // #3DADEB
    }
    
    fn foreground(&self) -> Color {
        Color::from_rgb(0.88, 0.88, 0.88) // #E0E0E0
    }
}

/// Get default theme based on system preference
pub fn get_default_theme() -> ThemeInfo {
    // Try to detect system dark mode preference
    let is_dark = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase()
        .contains("cosmic"); // Cosmic defaults to dark
    
    if is_dark {
        let dark_theme = DefaultDarkTheme;
        ThemeInfo::new(
            dark_theme.name().to_string(),
            dark_theme.is_light(),
            dark_theme.window_background(),
            dark_theme.view_background(),
            dark_theme.accent_color(),
            dark_theme.foreground(),
        )
    } else {
        let light_theme = DefaultLightTheme;
        ThemeInfo::new(
            light_theme.name().to_string(),
            light_theme.is_light(),
            light_theme.window_background(),
            light_theme.view_background(),
            light_theme.accent_color(),
            light_theme.foreground(),
        )
    }
}
