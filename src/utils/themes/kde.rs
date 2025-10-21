// KDE Plasma color scheme parser

use cosmic::iced::Color;
use super::{DesktopTheme, ThemeInfo};
use crate::utils::command_utils::SafeCommand;
use std::path::PathBuf;
use std::fs;

/// KDE color scheme parser
pub struct KdeColorScheme {
    pub name: String,
    pub is_light: bool,
    pub window_background: Color,
    pub view_background: Color,
    pub accent_color: Color,
    pub foreground: Color,
}

impl DesktopTheme for KdeColorScheme {
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

/// Parse RGB color from KDE format (R,G,B)
fn parse_kde_color(color_str: &str) -> Option<Color> {
    let parts: Vec<&str> = color_str.split(',').collect();
    if parts.len() >= 3 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            parts[0].trim().parse::<u8>(),
            parts[1].trim().parse::<u8>(),
            parts[2].trim().parse::<u8>(),
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

/// Get current KDE color scheme name
fn get_current_kde_scheme() -> Option<String> {
    // Try Plasma 6 first
    if let Ok(output) = SafeCommand::new("kreadconfig6")
        .args(["--file", "kdeglobals", "--group", "General", "--key", "ColorScheme"])
        .output_text()
    {
        let scheme = output.trim();
        if !scheme.is_empty() {
            return Some(scheme.to_string());
        }
    }
    
    // Fallback to Plasma 5
    if let Ok(output) = SafeCommand::new("kreadconfig5")
        .args(["--file", "kdeglobals", "--group", "General", "--key", "ColorScheme"])
        .output_text()
    {
        let scheme = output.trim();
        if !scheme.is_empty() {
            return Some(scheme.to_string());
        }
    }
    
    None
}

/// Find KDE color scheme file
fn find_kde_scheme_file(scheme_name: &str) -> Option<PathBuf> {
    let possible_paths = [
        format!("{}/.local/share/color-schemes/{}.colors", 
                std::env::var("HOME").unwrap_or_default(), scheme_name),
        format!("/usr/share/color-schemes/{}.colors", scheme_name),
        format!("/usr/local/share/color-schemes/{}.colors", scheme_name),
    ];
    
    for path_str in &possible_paths {
        let path = PathBuf::from(path_str);
        if path.exists() {
            return Some(path);
        }
    }
    
    None
}

/// Parse KDE color scheme file
fn parse_kde_scheme_file(file_path: &PathBuf) -> Option<KdeColorScheme> {
    let content = fs::read_to_string(file_path).ok()?;
    
    let mut window_bg = None;
    let mut window_fg = None;
    let mut view_bg = None;
    let mut selection_bg = None;
    
    let mut in_window_section = false;
    let mut in_view_section = false;
    let mut in_selection_section = false;
    
    for line in content.lines() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // Check for section headers
        if line.starts_with('[') && line.ends_with(']') {
            let section = &line[1..line.len()-1];
            in_window_section = section == "Colors:Window";
            in_view_section = section == "Colors:View";
            in_selection_section = section == "Colors:Selection";
            continue;
        }
        
        // Parse color definitions
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            if in_window_section {
                match key {
                    "BackgroundNormal" => window_bg = parse_kde_color(value),
                    "ForegroundNormal" => window_fg = parse_kde_color(value),
                    _ => {}
                }
            } else if in_view_section {
                match key {
                    "BackgroundNormal" => view_bg = parse_kde_color(value),
                    _ => {}
                }
            } else if in_selection_section {
                match key {
                    "BackgroundNormal" => selection_bg = parse_kde_color(value),
                    _ => {}
                }
            }
        }
    }
    
    // Use fallback colors if parsing failed
    let window_background = window_bg.unwrap_or_else(|| Color::from_rgb(0.2, 0.2, 0.2));
    let view_background = view_bg.unwrap_or_else(|| Color::from_rgb(0.25, 0.25, 0.25));
    let accent_color = selection_bg.unwrap_or_else(|| Color::from_rgb(0.24, 0.60, 0.89));
    let foreground = window_fg.unwrap_or_else(|| Color::from_rgb(0.9, 0.9, 0.9));
    
    // Determine if it's a light theme based on background brightness
    let (r, g, b) = (window_background.r, window_background.g, window_background.b);
    let brightness = 0.299 * r + 0.587 * g + 0.114 * b;
    let is_light = brightness > 0.5;
    
    Some(KdeColorScheme {
        name: file_path.file_stem()
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

/// Detect current KDE theme
pub fn detect_kde_theme() -> Option<ThemeInfo> {
    let scheme_name = get_current_kde_scheme()?;
    let scheme_file = find_kde_scheme_file(&scheme_name)?;
    let kde_theme = parse_kde_scheme_file(&scheme_file)?;
    
    Some(ThemeInfo::new(
        kde_theme.name,
        kde_theme.is_light,
        kde_theme.window_background,
        kde_theme.view_background,
        kde_theme.accent_color,
        kde_theme.foreground,
    ))
}
