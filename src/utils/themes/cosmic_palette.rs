// Cosmic accent palette strategy pattern for color mapping
// Maps arbitrary theme colors to the nearest Cosmic accent color for consistency

use cosmic::iced::Color;

/// Cosmic default accent palette for dark mode
/// Based on COSMIC desktop environment standards
pub const COSMIC_DARK_ACCENTS: &[(&str, Color)] = &[
    // Blue
    ("blue", Color::from_rgb(0.3882353, 0.81568627, 0.87450981)),
    // Indigo
    ("indigo", Color::from_rgb(0.63137255, 0.75294118, 0.92156863)),
    // Purple
    ("purple", Color::from_rgb(0.90588235, 0.61176471, 0.99607843)),
    // Pink
    ("pink", Color::from_rgb(1.0, 0.61176471, 0.69411765)),
    // Red
    ("red", Color::from_rgb(0.99215686, 0.63137255, 0.62745098)),
    // Orange
    ("orange", Color::from_rgb(1.0, 0.67843137, 0.0)),
    // Yellow
    ("yellow", Color::from_rgb(0.96862745, 0.87843137, 0.38431373)),
    // Green
    ("green", Color::from_rgb(0.57254902, 0.81176471, 0.61176471)),
    // Warm Grey
    ("warm_grey", Color::from_rgb(0.79215686, 0.72941176, 0.70588235)),
];

/// Cosmic default accent palette for light mode
/// Based on COSMIC desktop environment standards
pub const COSMIC_LIGHT_ACCENTS: &[(&str, Color)] = &[
    // Blue
    ("blue", Color::from_rgb(0.0, 0.32156863, 0.35294118)),
    // Indigo
    ("indigo", Color::from_rgb(0.18039216, 0.28627451, 0.42745098)),
    // Purple
    ("purple", Color::from_rgb(0.40784314, 0.12941176, 0.48627451)),
    // Pink
    ("pink", Color::from_rgb(0.52549020, 0.01568627, 0.22745098)),
    // Red
    ("red", Color::from_rgb(0.47058824, 0.16078431, 0.18039216)),
    // Orange
    ("orange", Color::from_rgb(0.38431373, 0.25098039, 0.0)),
    // Yellow
    ("yellow", Color::from_rgb(0.32549020, 0.28235294, 0.0)),
    // Green
    ("green", Color::from_rgb(0.09411765, 0.33333333, 0.16078431)),
    // Warm Grey
    ("warm_grey", Color::from_rgb(0.33333333, 0.27843137, 0.25882353)),
];

/// Strategy for selecting the best Cosmic accent color
#[derive(Debug, Clone, Copy)]
pub enum CosmicAccentStrategy {
    /// Map to the nearest color by Euclidean distance in RGB space
    NearestNeighbor,
    /// Map to the nearest color by perceptual distance (CIE76)
    Perceptual,
}

/// Cosmic accent palette manager
pub struct CosmicAccentPalette;

impl CosmicAccentPalette {
    /// Calculate Euclidean distance between two RGB colors
    fn rgb_distance(c1: &Color, c2: &Color) -> f32 {
        let dr = (c1.r - c2.r) * (c1.r - c2.r);
        let dg = (c1.g - c2.g) * (c1.g - c2.g);
        let db = (c1.b - c2.b) * (c1.b - c2.b);
        (dr + dg + db).sqrt()
    }

    /// Calculate perceptual distance (CIE76 simplified)
    /// Uses weighted RGB distance based on human perception
    fn perceptual_distance(c1: &Color, c2: &Color) -> f32 {
        // Weights based on human color perception
        // Red and Blue are perceived less accurately than Green
        let r_weight = 2.0; // Red weight
        let g_weight = 4.0; // Green weight (most visible)
        let b_weight = 3.0; // Blue weight

        let dr = (c1.r - c2.r) * r_weight;
        let dg = (c1.g - c2.g) * g_weight;
        let db = (c1.b - c2.b) * b_weight;

        (dr * dr + dg * dg + db * db).sqrt()
    }

    /// Get the nearest Cosmic accent color for a given color
    pub fn nearest_accent(
        color: Color,
        is_dark: bool,
        strategy: CosmicAccentStrategy,
    ) -> (String, Color) {
        let palette = if is_dark {
            COSMIC_DARK_ACCENTS
        } else {
            COSMIC_LIGHT_ACCENTS
        };

        let distance_fn = match strategy {
            CosmicAccentStrategy::NearestNeighbor => Self::rgb_distance,
            CosmicAccentStrategy::Perceptual => Self::perceptual_distance,
        };

        let (name, best_color) = palette
            .iter()
            .min_by(|a, b| {
                let dist_a = distance_fn(&color, &a.1);
                let dist_b = distance_fn(&color, &b.1);
                dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(name, color)| (*name, *color))
            .unwrap_or(("blue", palette[0].1));

        log::info!(
            "🎨 Mapped color {:?} to Cosmic {} accent: {} ({:?})",
            color,
            if is_dark { "dark" } else { "light" },
            name,
            best_color
        );

        (name.to_string(), best_color)
    }

    /// Get all Cosmic accent colors for a theme mode
    pub fn get_palette(is_dark: bool) -> Vec<(String, Color)> {
        let palette = if is_dark {
            COSMIC_DARK_ACCENTS
        } else {
            COSMIC_LIGHT_ACCENTS
        };

        palette
            .iter()
            .map(|(name, color)| (name.to_string(), *color))
            .collect()
    }

    /// Get all Cosmic accent colors as just Color values
    pub fn get_palette_colors(is_dark: bool) -> Vec<Color> {
        let palette = if is_dark {
            COSMIC_DARK_ACCENTS
        } else {
            COSMIC_LIGHT_ACCENTS
        };

        palette.iter().map(|(_, color)| *color).collect()
    }

    /// Map a theme accent color to the nearest Cosmic accent
    /// This should be called when applying Omarchy or other theme accents
    pub fn map_accent_color(
        color: Color,
        is_dark: bool,
    ) -> Color {
        let (_, mapped_color) = Self::nearest_accent(color, is_dark, CosmicAccentStrategy::Perceptual);
        mapped_color
    }

    /// Get a color by name from the palette
    pub fn get_by_name(name: &str, is_dark: bool) -> Option<Color> {
        let palette = if is_dark {
            COSMIC_DARK_ACCENTS
        } else {
            COSMIC_LIGHT_ACCENTS
        };

        palette
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, color)| *color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearest_neighbor_dark() {
        let purple = Color::from_rgb(0.90588235, 0.61176471, 0.99607843);
        let (name, _color) = CosmicAccentPalette::nearest_accent(
            purple,
            true,
            CosmicAccentStrategy::NearestNeighbor,
        );
        assert_eq!(name, "purple");
    }

    #[test]
    fn test_perceptual_distance() {
        let blue_dark = Color::from_rgb(0.3882353, 0.81568627, 0.87450981);
        let (name, _color) = CosmicAccentPalette::nearest_accent(
            blue_dark,
            true,
            CosmicAccentStrategy::Perceptual,
        );
        assert_eq!(name, "blue");
    }

    #[test]
    fn test_palette_colors_count() {
        let dark_colors = CosmicAccentPalette::get_palette_colors(true);
        assert_eq!(dark_colors.len(), 9);

        let light_colors = CosmicAccentPalette::get_palette_colors(false);
        assert_eq!(light_colors.len(), 9);
    }
}
