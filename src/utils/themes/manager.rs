// Advanced theme manager using ThemeBuilder for dynamic color changes
// Based on cosmic settings app theme management

use cosmic::cosmic_config::{Config, ConfigSet, CosmicConfigEntry};
use cosmic::cosmic_theme::palette::{Srgb, Srgba};
use cosmic::cosmic_theme::{
    CornerRadii, Spacing, Theme, ThemeBuilder, ThemeMode,
};
use cosmic::iced_core::Color;
use cosmic::Task;
use cosmic::theme::ThemeType;
use std::sync::Arc;

use super::ThemeInfo;
use crate::utils::desktop_theme::DesktopEnvironment;
use super::cosmic_palette::CosmicAccentPalette;

/// Context for color customization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorContext {
    CustomAccent,
    ApplicationBackground,
    ContainerBackground,
    InterfaceText,
    ControlComponent,
    AccentWindowHint,
}

/// Theme staging for applying changes
#[derive(Debug)]
pub enum ThemeStaged {
    Current,
    Both,
}

/// Advanced theme manager that can dynamically modify Cosmic themes
#[derive(Debug)]
pub struct ThemeManager {
    mode: (ThemeMode, Option<Config>),
    light: ThemeCustomizer,
    dark: ThemeCustomizer,
    custom_accent: Option<Srgb>,
    desktop_environment: DesktopEnvironment,
}

/// Customizer for individual theme variants
#[derive(Debug)]
pub struct ThemeCustomizer {
    builder: (ThemeBuilder, Option<Config>),
    theme: (Theme, Option<Config>),
    accent_palette: Option<Vec<Srgba>>,
    custom_window_hint: Option<Srgb>,
}

impl From<(Option<Config>, Option<Config>, Option<Vec<Srgba>>)> for ThemeCustomizer {
    fn from(
        (theme_config, builder_config, palette): (
            Option<Config>,
            Option<Config>,
            Option<Vec<Srgba>>,
        ),
    ) -> Self {
        let theme = match Theme::get_entry(theme_config.as_ref().unwrap()) {
            Ok(theme) => theme,
            Err((errs, theme)) => {
                for err in errs {
                    log::warn!("Error while loading theme: {err:?}");
                }
                theme
            }
        };

        let mut theme_builder = match ThemeBuilder::get_entry(builder_config.as_ref().unwrap()) {
            Ok(t) => t,
            Err((errors, t)) => {
                for e in errors {
                    log::error!("{e}");
                }
                t
            }
        };

        theme_builder = theme_builder
            .accent(theme.accent.base.color)
            .bg_color(theme.bg_color())
            .corner_radii(theme.corner_radii)
            .destructive(theme.destructive.base.color)
            .spacing(theme.spacing)
            .success(theme.success.base.color)
            .warning(theme.warning.base.color)
            .neutral_tint(theme.palette.neutral_5.color)
            .text_tint(theme.background.on.color);

        theme_builder.gaps = theme.gaps;

        let mut customizer = Self {
            builder: (theme_builder, builder_config),
            theme: (theme, theme_config),
            accent_palette: palette,
            custom_window_hint: None,
        };

        if customizer.accent_palette.is_none() {
            let palette = customizer.builder.0.palette.as_ref();
            customizer.accent_palette = Some(vec![
                palette.accent_blue,
                palette.accent_indigo,
                palette.accent_purple,
                palette.accent_pink,
                palette.accent_red,
                palette.accent_orange,
                palette.accent_yellow,
                palette.accent_green,
                palette.accent_warm_grey,
            ]);
        }

        customizer
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        let theme_mode_config = ThemeMode::config().ok();
        let theme_mode = theme_mode_config
            .as_ref()
            .map(|c| match ThemeMode::get_entry(c) {
                Ok(t) => t,
                Err((errors, t)) => {
                    for e in errors {
                        log::error!("{e}");
                    }
                    t
                }
            })
            .unwrap_or_default();

        let mut manager = Self {
            mode: (theme_mode, theme_mode_config),
            light: (
                Theme::light_config().ok(),
                ThemeBuilder::light_config().ok(),
                None, // No custom palette by default
            )
                .into(),
            dark: (
                Theme::dark_config().ok(),
                ThemeBuilder::dark_config().ok(),
                None, // No custom palette by default
            )
                .into(),
            custom_accent: None,
            desktop_environment: DesktopEnvironment::Unknown,
        };

        let customizer = manager.selected_customizer();
        manager.custom_accent = customizer.builder.0.accent.filter(|c| {
            let c = Srgba::new(c.red, c.green, c.blue, 1.0);
            let theme = &customizer.theme.0;
            c != theme.palette.accent_blue
                && c != theme.palette.accent_green
                && c != theme.palette.accent_indigo
                && c != theme.palette.accent_orange
                && c != theme.palette.accent_pink
                && c != theme.palette.accent_purple
                && c != theme.palette.accent_red
                && c != theme.palette.accent_warm_grey
                && c != theme.palette.accent_yellow
        });

        manager
    }
}

impl ThemeManager {
    /// Create a new theme manager for a specific desktop environment
    pub fn new(desktop_environment: DesktopEnvironment) -> Self {
        let mut manager = Self::default();
        manager.desktop_environment = desktop_environment;
        manager
    }

    /// Apply external theme colors to the current theme
    pub fn apply_external_theme(&mut self, theme_info: &ThemeInfo) -> Result<(), String> {
        log::info!("🎨 Applying external theme '{}' to ThemeManager", theme_info.name);
        
        // Apply custom colors on Cosmic desktop or when using Omarchy themes
        if self.desktop_environment != DesktopEnvironment::Cosmic && 
           self.desktop_environment != DesktopEnvironment::Omarchy {
            log::info!("🎨 Skipping color application - not on Cosmic or Omarchy desktop");
            return Ok(());
        }

        // Map accent color to nearest Cosmic accent for consistency
        let mapped_accent = CosmicAccentPalette::map_accent_color(
            theme_info.accent_color,
            !theme_info.is_light,
        );
        
        log::info!("🎨 Original accent: {:?}, Mapped to Cosmic: {:?}", 
            theme_info.accent_color, mapped_accent);

        // Convert our Color to Srgb/Srgba
        let accent_srgb = Srgb::new(
            mapped_accent.r,
            mapped_accent.g,
            mapped_accent.b,
        );
        
        let bg_srgba = Srgba::new(
            theme_info.window_background.r,
            theme_info.window_background.g,
            theme_info.window_background.b,
            theme_info.window_background.a,
        );
        
        let container_bg_srgba = Srgba::new(
            theme_info.view_background.r,
            theme_info.view_background.g,
            theme_info.view_background.b,
            theme_info.view_background.a,
        );
        
        let text_srgb = Srgb::new(
            theme_info.foreground.r,
            theme_info.foreground.g,
            theme_info.foreground.b,
        );

        // Apply colors to the appropriate theme variant
        let customizer = self.selected_customizer_mut();
        
        // Set accent color (now mapped to Cosmic palette)
        log::info!("🎨 DEBUG: Attempting to set Cosmic accent color: {:?}", accent_srgb);
        if let Some(_staged) = customizer.set_accent(Some(accent_srgb)) {
            log::info!("✅ DEBUG: Successfully applied Cosmic accent color: {:?}", accent_srgb);
        } else {
            log::warn!("❌ DEBUG: Failed to apply Cosmic accent color: {:?}", accent_srgb);
        }
        
        // Set background color
        log::info!("🎨 DEBUG: Attempting to set background color: {:?}", bg_srgba);
        if let Some(_staged) = customizer.set_bg_color(Some(bg_srgba)) {
            log::info!("✅ DEBUG: Successfully applied background color: {:?}", bg_srgba);
        } else {
            log::warn!("❌ DEBUG: Failed to apply background color: {:?}", bg_srgba);
        }
        
        // Set container background
        log::info!("🎨 DEBUG: Attempting to set container background: {:?}", container_bg_srgba);
        if let Some(_staged) = customizer.set_primary_container_bg(Some(container_bg_srgba)) {
            log::info!("✅ DEBUG: Successfully applied container background: {:?}", container_bg_srgba);
        } else {
            log::warn!("❌ DEBUG: Failed to apply container background: {:?}", container_bg_srgba);
        }
        
        // Set text color
        log::info!("🎨 DEBUG: Attempting to set text color: {:?}", text_srgb);
        if let Some(_staged) = customizer.set_text_tint(Some(text_srgb)) {
            log::info!("✅ DEBUG: Successfully applied text color: {:?}", text_srgb);
        } else {
            log::warn!("❌ DEBUG: Failed to apply text color: {:?}", text_srgb);
        }

        // Apply the changes
        log::info!("🎨 DEBUG: Applying builder changes...");
        customizer.apply_builder();
        log::info!("🎨 DEBUG: Applying theme changes...");
        customizer.apply_theme();

        // Build the theme to apply changes
        log::info!("🎨 DEBUG: Building theme to apply changes...");
        let _build_task = self.build_theme(ThemeStaged::Current);
        log::info!("🎨 DEBUG: Theme build task created");
        
        // Force the theme to be rebuilt immediately
        log::info!("🎨 DEBUG: Forcing theme rebuild...");
        let customizer = self.selected_customizer_mut();
        let new_theme = customizer.builder.0.clone().build();
        customizer.theme.0 = new_theme;
        log::info!("🎨 DEBUG: Theme rebuilt with new colors (mapped Cosmic accents)");

        log::info!("✅ Successfully applied external theme colors with Cosmic palette mapping");
        Ok(())
    }

    /// Build and apply the current theme
    pub fn build_theme(&mut self, stage: ThemeStaged) -> Task<()> {
        log::info!("🎨 ThemeManager::build_theme called with stage: {:?}", stage);
        
        macro_rules! theme_transaction {
            ($config:ident, $current_theme:ident, $new_theme:ident, { $($name:ident;)+ }) => {
                log::info!("🎨 Starting theme transaction");
                let tx = $config.transaction();

                $(
                    if $current_theme.$name != $new_theme.$name {
                        log::info!("🎨 Updating theme field: {}", stringify!($name));
                        _ = tx.set(stringify!($name), $new_theme.$name.clone());
                    }
                )+

                log::info!("🎨 Committing theme transaction");
                _ = tx.commit();
            }
        }

        let map_data_fn = |customizer: &ThemeCustomizer| {
            (customizer.builder.0.clone(), customizer.theme.1.clone())
        };

        let current = map_data_fn(if self.mode.0.is_dark {
            &self.dark
        } else {
            &self.light
        });

        let other = if let ThemeStaged::Both = stage {
            Some(map_data_fn(if !self.mode.0.is_dark {
                &self.dark
            } else {
                &self.light
            }))
        } else {
            None
        };

        let mut data = std::iter::once(current).chain(other);

        cosmic::task::future(async move {
            for (builder, config) in data.by_ref() {
                if let Some(config) = config {
                    let current_theme = match Theme::get_entry(&config) {
                        Ok(theme) => theme,
                        Err((_errs, theme)) => theme,
                    };

                    let new_theme = builder.build();
                    theme_transaction!(config, current_theme, new_theme, {
                        accent;
                        accent_button;
                        background;
                        button;
                        destructive;
                        destructive_button;
                        link_button;
                        icon_button;
                        palette;
                        primary;
                        secondary;
                        shade;
                        success;
                        text_button;
                        warning;
                        warning_button;
                        window_hint;
                        accent_text;
                    });
                }
            }
        })
    }

    /// Get the current cosmic theme
    pub fn cosmic_theme(&self) -> cosmic::Theme {
        log::info!("🎨 ThemeManager::cosmic_theme called");
        
        let theme = self.theme();
        log::info!("🎨 Current theme: {:?}", theme);
        log::info!("🎨 Theme accent color: {:?}", theme.accent.base.color);
        log::info!("🎨 Theme background color: {:?}", theme.bg_color());
        
        // Debug: Check if our custom colors are actually applied
        let builder = self.builder();
        log::info!("🎨 DEBUG: Builder accent color: {:?}", builder.accent);
        log::info!("🎨 DEBUG: Builder background color: {:?}", builder.bg_color);
        log::info!("🎨 DEBUG: Builder text tint: {:?}", builder.text_tint);
        log::info!("🎨 DEBUG: Builder primary container bg: {:?}", builder.primary_container_bg);
        
        let cosmic_theme = cosmic::Theme {
            theme_type: ThemeType::Custom(Arc::new(theme.clone())),
            ..cosmic::Theme::default()
        };
        
        log::info!("🎨 Created cosmic theme with custom theme type");
        cosmic_theme
    }

    /// Get the current theme
    pub fn theme(&self) -> &Theme {
        &self.selected_customizer().theme.0
    }

    /// Get the current theme mode
    pub fn mode(&self) -> &ThemeMode {
        &self.mode.0
    }

    /// Get the current builder
    pub fn builder(&self) -> &ThemeBuilder {
        &self.selected_customizer().builder.0
    }

    /// Get the selected customizer
    pub fn selected_customizer(&self) -> &ThemeCustomizer {
        if self.mode.0.is_dark {
            &self.dark
        } else {
            &self.light
        }
    }

    /// Get the selected customizer mutably
    pub fn selected_customizer_mut(&mut self) -> &mut ThemeCustomizer {
        if self.mode.0.is_dark {
            &mut self.dark
        } else {
            &mut self.light
        }
    }

    /// Get a color from the current theme
    pub fn get_color(&self, context: ColorContext) -> Option<Color> {
        match context {
            ColorContext::CustomAccent => self.custom_accent().map(Color::from),
            ColorContext::ApplicationBackground => self.builder().bg_color.map(Color::from),
            ColorContext::ContainerBackground => {
                self.builder().primary_container_bg.map(Color::from)
            }
            ColorContext::InterfaceText => self.builder().text_tint.map(Color::from),
            ColorContext::ControlComponent => self.builder().neutral_tint.map(Color::from),
            ColorContext::AccentWindowHint => self.builder().window_hint.map(Color::from),
        }
    }

    /// Set a color in the current theme
    pub fn set_color(&mut self, color: Option<Color>, context: ColorContext) -> Option<ThemeStaged> {
        log::info!("🎨 ThemeManager::set_color called with context: {:?}, color: {:?}", context, color);
        
        let theme_customizer = self.selected_customizer_mut();
        log::info!("🎨 Selected customizer: {:?}", std::ptr::addr_of!(theme_customizer));
        
        let result = match context {
            ColorContext::CustomAccent => {
                log::info!("🎨 Setting custom accent color: {:?}", color);
                let srgb_color = color.map(Srgb::from);
                log::info!("🎨 Converted to Srgb: {:?}", srgb_color);
                theme_customizer.set_accent(srgb_color)
            }
            ColorContext::ApplicationBackground => {
                log::info!("🎨 Setting application background color: {:?}", color);
                let srgba_color = color.map(Srgba::from);
                log::info!("🎨 Converted to Srgba: {:?}", srgba_color);
                theme_customizer.set_bg_color(srgba_color)
            }
            ColorContext::ContainerBackground => {
                log::info!("🎨 Setting container background color: {:?}", color);
                let srgba_color = color.map(Srgba::from);
                log::info!("🎨 Converted to Srgba: {:?}", srgba_color);
                theme_customizer.set_primary_container_bg(srgba_color)
            }
            ColorContext::InterfaceText => {
                log::info!("🎨 Setting interface text color: {:?}", color);
                let srgb_color = color.map(Srgb::from);
                log::info!("🎨 Converted to Srgb: {:?}", srgb_color);
                theme_customizer.set_text_tint(srgb_color)
            }
            ColorContext::ControlComponent => {
                log::info!("🎨 Setting control component color: {:?}", color);
                let srgb_color = color.map(Srgb::from);
                log::info!("🎨 Converted to Srgb: {:?}", srgb_color);
                theme_customizer.set_neutral_tint(srgb_color)
            }
            ColorContext::AccentWindowHint => {
                log::info!("🎨 Setting accent window hint color: {:?}", color);
                let srgb_color = color.map(Srgb::from);
                log::info!("🎨 Converted to Srgb: {:?}", srgb_color);
                theme_customizer.set_window_hint(srgb_color)
            }
        };
        
        log::info!("🎨 ThemeManager::set_color result: {:?}", result);
        result
    }

    /// Get custom accent color
    pub fn custom_accent(&self) -> &Option<Srgb> {
        &self.custom_accent
    }

    /// Get accent palette
    pub fn accent_palette(&self) -> &Option<Vec<Srgba>> {
        &self.selected_customizer().accent_palette
    }

    /// Get custom window hint
    pub fn custom_window_hint(&self) -> &Option<Srgb> {
        self.selected_customizer().custom_window_hint()
    }
}

impl ThemeCustomizer {
    /// Set theme builder without writing to cosmic-config
    pub fn set_builder(&mut self, builder: ThemeBuilder) -> &mut Self {
        self.builder.0 = builder;
        self
    }

    /// Write theme builder to cosmic-config, notifying all subscribers
    pub fn apply_builder(&mut self) -> &mut Self {
        if let Some(config) = self.builder.1.as_ref() {
            let _ = self.builder.0.write_entry(config);
        }
        self
    }

    /// Set theme without writing to cosmic-config
    pub fn set_theme(&mut self, theme: Theme) -> &mut Self {
        self.theme.0 = theme;
        self
    }

    /// Write theme to cosmic-config, notifying all subscribers
    pub fn apply_theme(&mut self) -> &mut Self {
        if let Some(config) = self.theme.1.as_ref() {
            let _ = self.theme.0.write_entry(config);
        }
        self
    }

    /// Set window hint color
    pub fn set_window_hint(&mut self, color: Option<Srgb>) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;

        self.custom_window_hint = color;
        self.builder.0.set_window_hint(config, color).ok()?;
        self.theme
            .0
            .set_window_hint(self.theme.1.as_ref()?, color)
            .ok()?;

        Some(ThemeStaged::Current)
    }

    /// Get custom window hint
    pub fn custom_window_hint(&self) -> &Option<Srgb> {
        &self.custom_window_hint
    }

    /// Set background color
    pub fn set_bg_color(&mut self, color: Option<Srgba>) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;
        self.builder.0.set_bg_color(config, color).ok()?;
        Some(ThemeStaged::Current)
    }

    /// Set primary container background color
    pub fn set_primary_container_bg(&mut self, color: Option<Srgba>) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;
        self.builder
            .0
            .set_primary_container_bg(config, color)
            .ok()?;
        Some(ThemeStaged::Current)
    }

    /// Set accent color
    pub fn set_accent(&mut self, color: Option<Srgb>) -> Option<ThemeStaged> {
        log::info!("🎨 ThemeCustomizer::set_accent called with color: {:?}", color);
        
        let config = self.builder.1.as_ref();
        log::info!("🎨 Builder config available: {}", config.is_some());
        
        if let Some(config) = config {
            log::info!("🎨 Setting accent color in ThemeBuilder");
            log::info!("🎨 DEBUG: Before set_accent - builder accent: {:?}", self.builder.0.accent);
            match self.builder.0.set_accent(config, color) {
                Ok(_) => {
                    log::info!("✅ Successfully set accent color in ThemeBuilder");
                    log::info!("🎨 DEBUG: After set_accent - builder accent: {:?}", self.builder.0.accent);
                    Some(ThemeStaged::Current)
                }
                Err(err) => {
                    log::error!("❌ Failed to set accent color in ThemeBuilder: {:?}", err);
                    None
                }
            }
        } else {
            log::warn!("❌ No builder config available for set_accent");
            None
        }
    }

    /// Set text tint color
    pub fn set_text_tint(&mut self, color: Option<Srgb>) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;
        self.builder.0.set_text_tint(config, color).ok()?;
        Some(ThemeStaged::Current)
    }

    /// Set neutral tint color
    pub fn set_neutral_tint(&mut self, color: Option<Srgb>) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;
        self.builder.0.set_neutral_tint(config, color).ok()?;
        Some(ThemeStaged::Current)
    }

    /// Set spacing
    pub fn set_spacing(&mut self, spacing: Spacing) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;
        self.builder.0.set_spacing(config, spacing).ok()?;
        self.theme
            .0
            .set_spacing(self.theme.1.as_ref()?, spacing)
            .ok()?;
        Some(ThemeStaged::Current)
    }

    /// Set corner radii
    pub fn set_corner_radii(&mut self, corner_radii: CornerRadii) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;
        self.builder.0.set_corner_radii(config, corner_radii).ok()?;
        self.theme
            .0
            .set_corner_radii(self.theme.1.as_ref()?, corner_radii)
            .ok()?;
        Some(ThemeStaged::Current)
    }

    /// Set gap size
    pub fn set_gap_size(&mut self, gap: u32) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;
        let builder = &mut self.builder.0;
        let mut gaps = builder.gaps;

        gaps.1 = if gap < builder.active_hint {
            builder.active_hint
        } else {
            gap
        };

        if let Err(err) = builder.set_gaps(config, gaps) {
            log::error!("Error setting the gap: {err:?}");
            return None;
        }

        self.theme.0.set_gaps(self.theme.1.as_ref()?, gaps).ok()?;
        Some(ThemeStaged::Current)
    }

    /// Set active hint
    pub fn set_active_hint(&mut self, active_hint: u32) -> Option<ThemeStaged> {
        let config = self.builder.1.as_ref()?;
        let builder = &mut self.builder.0;

        if let Err(err) = builder.set_active_hint(config, active_hint) {
            log::error!("Error setting the active hint: {err:?}");
            return None;
        }

        if active_hint > builder.gaps.1 {
            let mut gaps = builder.gaps;
            gaps.1 = active_hint;
            if builder.set_gaps(config, gaps).unwrap_or_default() {
                let _ = self.theme.0.set_active_hint(self.theme.1.as_ref()?, gaps.1);
            }
        }

        self.theme
            .0
            .set_active_hint(self.theme.1.as_ref()?, active_hint)
            .ok()?;

        Some(ThemeStaged::Current)
    }
}
