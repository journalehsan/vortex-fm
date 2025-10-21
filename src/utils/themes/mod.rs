// Theme trait and module definitions for adaptive desktop theming

use cosmic::iced::Color;

/// Trait for desktop environment themes
pub trait DesktopTheme {
    /// Get the theme name
    fn name(&self) -> &str;
    
    /// Check if this is a light theme
    fn is_light(&self) -> bool;
    
    /// Get window background color
    fn window_background(&self) -> Color;
    
    /// Get primary container background color (for views, panels, etc.)
    fn view_background(&self) -> Color;
    
    /// Get accent color (for highlights, buttons, etc.)
    fn accent_color(&self) -> Color;
    
    /// Get foreground/text color
    fn foreground(&self) -> Color;
}

/// Theme detection result
#[derive(Debug, Clone)]
pub struct ThemeInfo {
    pub name: String,
    pub is_light: bool,
    pub window_background: Color,
    pub view_background: Color,
    pub accent_color: Color,
    pub foreground: Color,
}

impl ThemeInfo {
    pub fn new(
        name: String,
        is_light: bool,
        window_background: Color,
        view_background: Color,
        accent_color: Color,
        foreground: Color,
    ) -> Self {
        Self {
            name,
            is_light,
            window_background,
            view_background,
            accent_color,
            foreground,
        }
    }
}

impl DesktopTheme for ThemeInfo {
    fn name(&self) -> &str {
        &self.name
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
        self.accent_color
    }
    
    fn foreground(&self) -> Color {
        self.foreground
    }
}

pub mod omarchy;
pub mod kde;
pub mod gnome;
pub mod default;
