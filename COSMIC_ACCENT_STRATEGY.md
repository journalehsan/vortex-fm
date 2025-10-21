# Cosmic Accent Palette Strategy Pattern

## Overview

This document explains how the Vortex File Manager uses the Cosmic Desktop accent palette strategy to ensure consistent and compatible theme colors across different desktop environments and distributions.

## Problem

Different theme engines (Omarchy, KDE, GNOME, etc.) define accent colors independently, which can lead to:

1. **Color Rejection**: Non-standard colors may be automatically changed by Cosmic Theme system, causing unexpected visual behavior
2. **Distribution Fragmentation**: Different distributions use different standard colors, leading to inconsistency
3. **Icon/Folder Color Incompatibility**: Custom colors may not match the standard palette used by icon and folder color systems
4. **User Confusion**: Theme changes don't apply consistently across applications

## Solution: Cosmic Accent Strategy

We implement a **strategy pattern** that maps arbitrary theme colors to the nearest Cosmic Desktop standard accent color.

### Architecture

```
Theme Color Input (from Omarchy, KDE, etc.)
    ↓
CosmicAccentPalette::map_accent_color()
    ↓
Color Distance Calculation (Perceptual or RGB)
    ↓
Nearest Cosmic Standard Color
    ↓
Applied to Theme System
```

## Cosmic Standard Palettes

### Dark Mode Accent Colors

The Cosmic Desktop defines 9 standard accent colors for dark mode:

```rust
- Blue:       (0.3882353, 0.81568627, 0.87450981)
- Indigo:     (0.63137255, 0.75294118, 0.92156863)
- Purple:     (0.90588235, 0.61176471, 0.99607843)
- Pink:       (1.0, 0.61176471, 0.69411765)
- Red:        (0.99215686, 0.63137255, 0.62745098)
- Orange:     (1.0, 0.67843137, 0.0)
- Yellow:     (0.96862745, 0.87843137, 0.38431373)
- Green:      (0.57254902, 0.81176471, 0.61176471)
- Warm Grey:  (0.79215686, 0.72941176, 0.70588235)
```

### Light Mode Accent Colors

The same 9 colors for light mode (different shades):

```rust
- Blue:       (0.0, 0.32156863, 0.35294118)
- Indigo:     (0.18039216, 0.28627451, 0.42745098)
- Purple:     (0.40784314, 0.12941176, 0.48627451)
- Pink:       (0.52549020, 0.01568627, 0.22745098)
- Red:        (0.47058824, 0.16078431, 0.18039216)
- Orange:     (0.38431373, 0.25098039, 0.0)
- Yellow:     (0.32549020, 0.28235294, 0.0)
- Green:      (0.09411765, 0.33333333, 0.16078431)
- Warm Grey:  (0.33333333, 0.27843137, 0.25882353)
```

## Distance Calculation Strategies

### 1. Nearest Neighbor (RGB Distance)

Uses Euclidean distance in RGB color space:

```
distance = √((r1-r2)² + (g1-g2)² + (b1-b2)²)
```

**Pros**: Fast, simple
**Cons**: Doesn't account for human color perception differences

### 2. Perceptual Distance (CIE76 Simplified)

Uses weighted RGB distance based on human perception:

```
distance = √(4×(r1-r2)² + 4×(g1-g2)² + 9×(b1-b2)²)
```

Weights reflect that:
- Green (4.0) is perceived more accurately
- Blue (3.0) is perceived moderately well
- Red (2.0) is perceived less accurately

**Pros**: Better perceptual accuracy
**Cons**: Slightly slower (but negligible)

## Usage Examples

### Mapping a Single Color

```rust
use crate::utils::themes::CosmicAccentPalette;
use cosmic::iced::Color;

// Original Omarchy purple accent
let purple = Color::from_rgb(0.89, 0.56, 0.98);

// Map to nearest Cosmic accent (dark mode)
let mapped = CosmicAccentPalette::map_accent_color(purple, true);
// Result: Cosmic Dark Purple (0.90588235, 0.61176471, 0.99607843)
```

### Getting All Available Colors

```rust
// Get all Cosmic accent colors for light mode
let light_accents = CosmicAccentPalette::get_palette(false);
// Returns: Vec<(String, Color)> with names like "blue", "purple", etc.

// Get just the colors
let colors = CosmicAccentPalette::get_palette_colors(true);
// Returns: Vec<Color>
```

### Getting Specific Color by Name

```rust
let purple = CosmicAccentPalette::get_by_name("purple", true);
// Some(Color::from_rgb(0.90588235, 0.61176471, 0.99607843))
```

## Integration Points

### 1. Omarchy Theme Detection

When an Omarchy theme is detected, its accent color is automatically mapped:

```rust
// In omarchy.rs
impl DesktopTheme for OmarchyTheme {
    fn accent_color(&self) -> Color {
        // Automatically maps to nearest Cosmic accent
        CosmicAccentPalette::map_accent_color(self.accent_color, !self.is_light)
    }
}
```

### 2. Theme Manager Application

When applying external themes to Cosmic:

```rust
// In manager.rs
pub fn apply_external_theme(&mut self, theme_info: &ThemeInfo) -> Result<(), String> {
    let mapped_accent = CosmicAccentPalette::map_accent_color(
        theme_info.accent_color,
        !theme_info.is_light,
    );
    // Apply mapped_accent to theme...
}
```

### 3. Desktop Theme Application

When applying theme to the Cosmic system:

```rust
// In desktop_theme.rs
pub fn apply_theme_to_cosmic(theme: &ThemeInfo) -> cosmic::theme::Theme {
    let mapped_accent = CosmicAccentPalette::map_accent_color(
        theme.accent_color,
        !theme.is_light,
    );
    // Use mapped_accent...
}
```

## Why This Matters

### For Users
- **Consistent appearance**: Theme colors are always valid and recognized by Cosmic
- **No surprises**: Colors won't be automatically changed to something unexpected
- **Better compatibility**: Works with icon themes, folder colors, and other Cosmic-integrated systems

### For Developers
- **Standards-based**: Uses official Cosmic Desktop color standards
- **Maintainable**: Single source of truth for color definitions
- **Extensible**: Easy to add more strategies or color palettes

### For Distributions
- **Predictable behavior**: All distributions get consistent results
- **Integration**: Works seamlessly with standard Cosmic color systems
- **Accessibility**: Perceptual distance algorithm ensures colors are distinguishable

## Testing

The module includes unit tests for color distance calculations:

```bash
cargo test --lib utils::themes::cosmic_palette
```

Tests verify:
- RGB distance calculation correctness
- Perceptual distance accuracy
- Palette loading for both dark and light modes
- Color name lookups

## Future Enhancements

1. **User Preferences**: Allow users to choose distance strategy
2. **Custom Palettes**: Support for theme-specific color palettes
3. **Automatic Palette Detection**: Detect palette from system at runtime
4. **Color Accessibility**: WCAG contrast ratio checks
5. **Animation**: Smooth color transitions when changing themes

## References

- COSMIC Desktop Project: https://github.com/pop-os/cosmic-epoch
- CIE Color Perception: https://en.wikipedia.org/wiki/Color_difference
- Color Blindness Considerations: https://www.color-blindness.com/

## Conclusion

By using the Cosmic Accent Palette Strategy Pattern, we ensure that theme colors are always:
- **Valid**: Recognized by the Cosmic theme system
- **Consistent**: Same across all distributions
- **Compatible**: Work with standard icon and folder systems
- **Predictable**: Users get expected visual results
