# Advanced Theme System

This document explains the advanced theme system implemented in Vortex FM, which provides comprehensive theme integration across different desktop environments with special support for Cosmic desktop's ThemeBuilder.

## Overview

The theme system consists of several components:

1. **Desktop Environment Detection** - Automatically detects the current desktop environment
2. **Theme Detection** - Extracts theme information from various sources (Omarchy, GNOME, KDE, etc.)
3. **Theme Manager** - Advanced theme customization using Cosmic's ThemeBuilder
4. **Color Application** - Applies detected colors to the application interface

## Architecture

```
Desktop Environment Detection
    ↓
Theme Detection (Omarchy/GNOME/KDE/etc.)
    ↓
Theme Manager (ThemeBuilder integration)
    ↓
Color Application (Cosmic theme system)
```

## Components

### 1. Desktop Environment Detection

Located in `src/utils/desktop_theme.rs`, this module detects the current desktop environment:

- **Omarchy** - Custom theme system with `omarchy-theme-current` command
- **Hyprland** - Window manager with optional Omarchy integration
- **Cosmic** - System76's desktop environment with ThemeBuilder support
- **KDE** - KDE Plasma desktop
- **GNOME** - GNOME desktop environment
- **Unknown** - Fallback for unrecognized environments

### 2. Theme Detection

The system supports multiple theme sources:

#### Omarchy Themes
- **catppuccin** - Popular dark theme with blue accents
- **dracula** - Dark theme with red accents
- **everforest** - Dark theme with green accents
- **gruvbox** - Retro dark theme with yellow accents
- **kanagawa** - Dark theme inspired by Japanese art
- **matte-black** - Minimalist dark theme
- **nord** - Arctic-inspired dark theme
- **osaka-jade** - Dark theme with green accents
- **ristretto** - Dark theme with red accents
- **tokyo-night** - Dark theme with blue accents
- **catppuccin-latte** - Light version of catppuccin
- **rose-pine** - Light theme with purple accents

#### GNOME/KDE Themes
- Automatically detects GTK themes
- Parses CSS files for color information
- Extracts accent colors and background colors

### 3. Theme Manager

The `ThemeManager` (in `src/utils/themes/manager.rs`) provides advanced theme customization:

#### Features
- **Dynamic Color Changes** - Modify theme colors at runtime
- **ThemeBuilder Integration** - Uses Cosmic's ThemeBuilder for deep customization
- **Light/Dark Mode Support** - Handles both theme variants
- **Color Context System** - Different color contexts for different UI elements

#### Color Contexts
- `CustomAccent` - Primary accent color
- `ApplicationBackground` - Main application background
- `ContainerBackground` - Container/panel backgrounds
- `InterfaceText` - Text color
- `ControlComponent` - UI control colors
- `AccentWindowHint` - Window hint colors

### 4. Color Application

The system applies detected colors to the Cosmic theme system:

#### For Cosmic Desktop
- Uses `ThemeBuilder` for deep theme customization
- Modifies theme configuration files
- Applies colors to all theme variants
- Supports real-time theme updates

#### For Other Desktops
- Uses basic theme preference setting
- Applies light/dark mode preferences
- Limited color customization

## Usage

### Basic Theme Application

```rust
use vortex_fm::utils::desktop_theme::{get_desktop_theme, apply_theme_to_cosmic};

// Get the current desktop theme
let theme = get_desktop_theme();

// Apply the theme
let cosmic_theme = apply_theme_to_cosmic(&theme);
```

### Advanced Theme Customization

```rust
use vortex_fm::utils::desktop_theme::{get_theme_manager, apply_advanced_theme};
use vortex_fm::utils::themes::manager::{ColorContext, ThemeStaged};

// Get the theme manager (only available on Cosmic)
if let Some(theme_manager) = get_theme_manager() {
    // Set a custom accent color
    let custom_color = Color::from_rgb(0.86, 0.20, 0.20); // Red
    theme_manager.set_color(Some(custom_color), ColorContext::CustomAccent);
    
    // Build the theme with changes
    let _ = theme_manager.build_theme(ThemeStaged::Current);
}
```

### Theme Detection Examples

```rust
// Detect Omarchy theme
use vortex_fm::utils::themes::omarchy::detect_omarchy_theme;
if let Some(theme) = detect_omarchy_theme() {
    println!("Detected Omarchy theme: {}", theme.name);
}

// Detect GNOME theme
use vortex_fm::utils::themes::gnome::detect_gnome_theme;
if let Some(theme) = detect_gnome_theme() {
    println!("Detected GNOME theme: {}", theme.name);
}
```

## Configuration

### Environment Variables

- `XDG_CURRENT_DESKTOP` - Used for desktop environment detection
- `GTK_THEME` - Used for GTK theme detection

### Commands

- `omarchy-theme-current` - Returns current Omarchy theme name
- `gsettings` - Used for GNOME theme detection
- `kreadconfig5` - Used for KDE theme detection

## Examples

See `examples/theme_example.rs` for a complete example of theme detection and customization.

## Troubleshooting

### Common Issues

1. **Theme not detected** - Ensure the theme detection commands are available
2. **Colors not applied** - Check if running on Cosmic desktop for full customization
3. **Theme conflicts** - Some themes may not be compatible with all desktop environments

### Debugging

Enable debug logging to see theme detection and application process:

```rust
env_logger::init();
```

The system will log:
- Desktop environment detection
- Theme detection results
- Color application process
- Any errors or warnings

## Future Enhancements

- Support for more desktop environments
- Additional theme sources
- Theme preview functionality
- User theme customization interface
- Theme import/export functionality
