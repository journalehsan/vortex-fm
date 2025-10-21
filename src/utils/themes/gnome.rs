// GNOME/GTK theme parser

use cosmic::iced::Color;
use super::{DesktopTheme, ThemeInfo};
use crate::utils::command_utils::SafeCommand;
use std::path::PathBuf;
use std::fs;

/// GNOME/GTK theme parser
pub struct GnomeTheme {
    pub name: String,
    pub is_light: bool,
    pub window_background: Color,
    pub view_background: Color,
    pub accent_color: Color,
    pub foreground: Color,
}

impl DesktopTheme for GnomeTheme {
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

/// Parse hex color from GTK CSS format
fn parse_hex_color(hex: &str) -> Option<Color> {
    let hex = hex.trim().trim_start_matches('#');
    if hex.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return Some(Color::from_rgb(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
            ));
        }
    }
    None
}

/// Get current GTK theme name
fn get_current_gtk_theme() -> Option<String> {
    // Try gsettings first
    if let Ok(output) = SafeCommand::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output_text()
    {
        let theme = output.trim().trim_matches('\'').trim_matches('"');
        if !theme.is_empty() {
            return Some(theme.to_string());
        }
    }
    
    // Try xfconf-query for XFCE
    if let Ok(output) = SafeCommand::new("xfconf-query")
        .args(["-c", "xsettings", "-p", "/Net/ThemeName"])
        .output_text()
    {
        let theme = output.trim();
        if !theme.is_empty() {
            return Some(theme.to_string());
        }
    }
    
    None
}

/// Find GTK theme directory
fn find_gtk_theme_dir(theme_name: &str) -> Option<PathBuf> {
    let possible_paths = [
        format!("{}/.themes/{}", std::env::var("HOME").unwrap_or_default(), theme_name),
        format!("{}/.local/share/themes/{}", std::env::var("HOME").unwrap_or_default(), theme_name),
        format!("/usr/share/themes/{}", theme_name),
        format!("/usr/local/share/themes/{}", theme_name),
    ];
    
    for path_str in &possible_paths {
        let path = PathBuf::from(path_str);
        if path.exists() {
            // Check for GTK versions in order of preference
            for gtk_ver in ["gtk-4.0", "gtk-3.0", "gtk-3.22", "gtk-3.20"] {
                let gtk_path = path.join(gtk_ver);
                if gtk_path.exists() {
                    return Some(gtk_path);
                }
            }
        }
    }
    
    None
}

/// Parse GTK CSS file for color variables
fn parse_gtk_css_file(css_path: &PathBuf) -> Option<GnomeTheme> {
    let content = fs::read_to_string(css_path).ok()?;
    
    let mut window_bg = None;
    let mut window_fg = None;
    let mut view_bg = None;
    let mut accent = None;
    
    // Parse @define-color variables
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("@define-color") {
            if let Some((_, rest)) = line.split_once(' ') {
                if let Some((name, value)) = rest.split_once(' ') {
                    let name = name.trim();
                    let value = value.trim().trim_end_matches(';');
                    
                    match name {
                        "theme_bg_color" | "background" => {
                            window_bg = parse_hex_color(value);
                        }
                        "theme_fg_color" | "foreground" => {
                            window_fg = parse_hex_color(value);
                        }
                        "theme_base_color" | "base" => {
                            view_bg = parse_hex_color(value);
                        }
                        "theme_selected_bg_color" | "accent" => {
                            accent = parse_hex_color(value);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    
    // Use fallback colors if parsing failed
    let window_background = window_bg.unwrap_or_else(|| Color::from_rgb(0.95, 0.95, 0.95));
    let view_background = view_bg.unwrap_or_else(|| Color::from_rgb(1.0, 1.0, 1.0));
    let accent_color = accent.unwrap_or_else(|| Color::from_rgb(0.24, 0.60, 0.89));
    let foreground = window_fg.unwrap_or_else(|| Color::from_rgb(0.2, 0.2, 0.2));
    
    // Determine if it's a light theme based on background brightness
    let (r, g, b) = (window_background.r, window_background.g, window_background.b);
    let brightness = 0.299 * r + 0.587 * g + 0.114 * b;
    let is_light = brightness > 0.5;
    
    Some(GnomeTheme {
        name: css_path.parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string(),
        is_light,
        window_background,
        view_background,
        accent_color,
        foreground,
    })
}

/// Detect current GNOME/GTK theme
pub fn detect_gnome_theme() -> Option<ThemeInfo> {
    let theme_name = get_current_gtk_theme()?;
    let theme_dir = find_gtk_theme_dir(&theme_name)?;
    
    // Look for CSS files in order of preference
    for css_name in ["gtk.css", "gtk-dark.css", "colors.css"] {
        let css_path = theme_dir.join(css_name);
        if css_path.exists() {
            if let Some(gnome_theme) = parse_gtk_css_file(&css_path) {
                return Some(ThemeInfo::new(
                    gnome_theme.name,
                    gnome_theme.is_light,
                    gnome_theme.window_background,
                    gnome_theme.view_background,
                    gnome_theme.accent_color,
                    gnome_theme.foreground,
                ));
            }
        }
    }
    
    None
}
