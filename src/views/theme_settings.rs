// Theme settings page - inspired by Cosmic settings app
// This shows how to integrate our ThemeManager with a real UI

use cosmic::{
    iced::ContentFit,
    iced_core::{Alignment, Length},
    widget::{
        self, button, container, settings, text,
        icon::{from_name, icon},
    },
    Apply, Element,
};
use cosmic::cosmic_theme::Spacing;
// use cosmic::cosmic_theme::palette::Srgba;
use std::collections::HashMap;

use crate::utils::desktop_theme::{get_desktop_theme, apply_advanced_theme, get_theme_manager, detect_desktop_environment};
use crate::utils::themes::manager::{ColorContext, ThemeManager};
use crate::utils::themes::CosmicAccentPalette;

/// Messages for theme settings interactions
#[derive(Clone, Debug)]
pub enum ThemeMessage {
    DarkMode(bool),
    AutoSwitch(bool),
    PaletteAccent(cosmic::iced::Color),
    CustomAccent(cosmic::iced::Color),
    ApplicationBackground(cosmic::iced::Color),
    ContainerBackground(cosmic::iced::Color),
    InterfaceText(cosmic::iced::Color),
    ControlComponent(cosmic::iced::Color),
    AccentWindowHint(cosmic::iced::Color),
    UseDefaultWindowHint,
}

/// Theme settings page state
pub struct ThemeSettingsPage {
    pub theme_manager: Option<ThemeManager>,
    pub current_theme: crate::utils::themes::ThemeInfo,
    pub custom_colors: HashMap<ColorContext, cosmic::iced::Color>,
}

impl Default for ThemeSettingsPage {
    fn default() -> Self {
        let current_theme = get_desktop_theme();
        Self {
            theme_manager: None,
            current_theme,
            custom_colors: HashMap::new(),
        }
    }
}

impl ThemeSettingsPage {
    pub fn new() -> Self {
        let mut page = Self::default();
        
        // Initialize theme manager if on Cosmic desktop
        if let Some(theme_manager_mutex) = get_theme_manager() {
            let mut theme_manager_guard = theme_manager_mutex.lock().unwrap();
            if theme_manager_guard.is_none() {
                let desktop = detect_desktop_environment();
                *theme_manager_guard = Some(ThemeManager::new(desktop));
            }
            page.theme_manager = theme_manager_guard.take();
        }
        
        page
    }

    /// Create the main theme settings section
    pub fn section() -> Element<'static, ThemeMessage> {
        let _descriptions = Self::i18n();
        
        settings::section()
            .title("Theme & Colors")
            .add(Self::theme_mode_section())
            .add(Self::auto_switch_section())
            .add(Self::accent_color_section())
            .add(Self::application_background_section())
            .add(Self::container_background_section())
            .add(Self::interface_text_section())
            .add(Self::control_component_section())
            .apply(Element::from)
    }

    /// Theme mode selection (light/dark)
    fn theme_mode_section() -> Element<'static, ThemeMessage> {
        let dark_mode_illustration = from_name("illustration-appearance-mode-dark").handle();
        let light_mode_illustration = from_name("illustration-appearance-mode-light").handle();

        container(
            cosmic::iced::widget::row![
                cosmic::iced::widget::column![
                    button::custom_image_button(
                        icon(dark_mode_illustration)
                            .content_fit(ContentFit::Contain)
                            .width(Length::Fill)
                            .height(Length::Fixed(100.0)),
                        None
                    )
                    .class(button::ButtonClass::Image)
                    .on_press(ThemeMessage::DarkMode(true))
                    .padding(1)
                    .apply(widget::container)
                    .max_width(191),
                    text::body("Dark")
                ]
                .spacing(8)
                .width(Length::FillPortion(1))
                .align_x(Alignment::Center),
                cosmic::iced::widget::column![
                    button::custom_image_button(
                        icon(light_mode_illustration)
                            .content_fit(ContentFit::Contain)
                            .width(Length::Fill)
                            .height(Length::Fixed(100.0)),
                        None
                    )
                    .class(button::ButtonClass::Image)
                    .on_press(ThemeMessage::DarkMode(false))
                    .padding(1)
                    .apply(widget::container)
                    .max_width(191),
                    text::body("Light")
                ]
                .spacing(8)
                .width(Length::FillPortion(1))
                .align_x(Alignment::Center)
            ]
            .spacing(8)
            .width(Length::Fixed(478.0))
            .align_y(Alignment::Center),
        )
        .center_x(Length::Fill)
        .into()
    }

    /// Auto switch toggle
    fn auto_switch_section() -> Element<'static, ThemeMessage> {
        settings::item::builder("Auto Switch")
            .description("Automatically switch between light and dark themes based on time of day")
            .toggler(false, ThemeMessage::AutoSwitch)
            .into()
    }

    /// Accent color palette
    fn accent_color_section() -> Element<'static, ThemeMessage> {
        let Spacing { space_xxs, .. } = cosmic::theme::spacing();
        
        // Get default Cosmic accent colors based on theme mode
        // For now, use dark mode palette (will be updated dynamically based on theme)
        let cosmic_colors = CosmicAccentPalette::get_palette_colors(false);
        
        // Use only the first 6 colors for the UI (Blue, Indigo, Purple, Pink, Red, Orange)
        let accent_colors: Vec<cosmic::iced::Color> = cosmic_colors
            .iter()
            .take(6)
            .map(|c| cosmic::iced::Color::from_rgb(c.r, c.g, c.b))
            .collect();

        let mut accent_palette_row = Vec::with_capacity(accent_colors.len() + 1);

        for color in accent_colors {
            accent_palette_row.push(Self::color_button(
                Some(ThemeMessage::PaletteAccent(color)),
                color,
                false, // TODO: Check if this is the current accent
                48,
                48,
            ));
        }

        // Custom accent color picker
        accent_palette_row.push(
            container(
                Self::color_button(
                    Some(ThemeMessage::CustomAccent(cosmic::iced::Color::from_rgb(0.5, 0.5, 0.5))),
                    cosmic::iced::Color::from_rgb(0.5, 0.5, 0.5),
                    false,
                    48,
                    48,
                )
            )
            .into(),
        );

        cosmic::iced::widget::column![
            text::body("Accent Color"),
            widget::flex_row(accent_palette_row).spacing(16)
        ]
        .spacing(space_xxs)
        .into()
    }

    /// Application background color picker
    fn application_background_section() -> Element<'static, ThemeMessage> {
        settings::item::builder("Application Background")
            .description("Main application background color")
            .control(
                Self::color_button(
                    Some(ThemeMessage::ApplicationBackground(cosmic::iced::Color::from_rgb(0.95, 0.95, 0.95))),
                    cosmic::iced::Color::from_rgb(0.95, 0.95, 0.95),
                    false,
                    48,
                    24,
                )
            )
            .into()
    }

    /// Container background color picker
    fn container_background_section() -> Element<'static, ThemeMessage> {
        settings::item::builder("Container Background")
            .description("Background color for containers and panels")
            .control(
                Self::color_button(
                    Some(ThemeMessage::ContainerBackground(cosmic::iced::Color::from_rgb(1.0, 1.0, 1.0))),
                    cosmic::iced::Color::from_rgb(1.0, 1.0, 1.0),
                    false,
                    48,
                    24,
                )
            )
            .into()
    }

    /// Interface text color picker
    fn interface_text_section() -> Element<'static, ThemeMessage> {
        settings::item::builder("Interface Text")
            .description("Text color for interface elements")
            .control(
                Self::color_button(
                    Some(ThemeMessage::InterfaceText(cosmic::iced::Color::from_rgb(0.2, 0.2, 0.2))),
                    cosmic::iced::Color::from_rgb(0.2, 0.2, 0.2),
                    false,
                    48,
                    24,
                )
            )
            .into()
    }

    /// Control component color picker
    fn control_component_section() -> Element<'static, ThemeMessage> {
        settings::item::builder("Control Components")
            .description("Color for UI controls and components")
            .control(
                Self::color_button(
                    Some(ThemeMessage::ControlComponent(cosmic::iced::Color::from_rgb(0.6, 0.6, 0.6))),
                    cosmic::iced::Color::from_rgb(0.6, 0.6, 0.6),
                    false,
                    48,
                    24,
                )
            )
            .into()
    }

    /// Color picker button
    fn color_button(
        on_press: Option<ThemeMessage>,
        _color: cosmic::iced::Color,
        selected: bool,
        width: u16,
        height: u16,
    ) -> Element<'static, ThemeMessage> {
        // Create a simple color preview using a basic container
        let color_preview = container(cosmic::iced::widget::text(""))
            .width(Length::Fixed(f32::from(width)))
            .height(Length::Fixed(f32::from(height)));

        button::custom_image_button(color_preview, None)
            .padding(0)
            .selected(selected)
            .class(button::ButtonClass::Image)
            .on_press_maybe(on_press)
            .width(Length::Fixed(f32::from(width)))
            .height(Length::Fixed(f32::from(height)))
            .into()
    }

    /// Handle theme messages
    pub fn update(&mut self, message: ThemeMessage) {
        match message {
            ThemeMessage::DarkMode(is_dark) => {
                log::info!("🎨 Setting theme mode to: {}", if is_dark { "dark" } else { "light" });
                // Apply theme with new mode
                let mut theme = self.current_theme.clone();
                theme.is_light = !is_dark;
                let _ = apply_advanced_theme(&theme);
            }
            ThemeMessage::AutoSwitch(enabled) => {
                log::info!("🎨 Auto switch: {}", enabled);
                // TODO: Implement auto switch logic
            }
            ThemeMessage::PaletteAccent(color) => {
                log::info!("🎨 Setting palette accent color: {:?}", color);
                self.apply_color_change(ColorContext::CustomAccent, color);
            }
            ThemeMessage::CustomAccent(color) => {
                log::info!("🎨 Setting custom accent color: {:?}", color);
                self.apply_color_change(ColorContext::CustomAccent, color);
            }
            ThemeMessage::ApplicationBackground(color) => {
                log::info!("🎨 Setting application background: {:?}", color);
                self.apply_color_change(ColorContext::ApplicationBackground, color);
            }
            ThemeMessage::ContainerBackground(color) => {
                log::info!("🎨 Setting container background: {:?}", color);
                self.apply_color_change(ColorContext::ContainerBackground, color);
            }
            ThemeMessage::InterfaceText(color) => {
                log::info!("🎨 Setting interface text: {:?}", color);
                self.apply_color_change(ColorContext::InterfaceText, color);
            }
            ThemeMessage::ControlComponent(color) => {
                log::info!("🎨 Setting control component: {:?}", color);
                self.apply_color_change(ColorContext::ControlComponent, color);
            }
            ThemeMessage::AccentWindowHint(color) => {
                log::info!("🎨 Setting accent window hint: {:?}", color);
                self.apply_color_change(ColorContext::AccentWindowHint, color);
            }
            ThemeMessage::UseDefaultWindowHint => {
                log::info!("🎨 Using default window hint");
                // TODO: Reset window hint to default
            }
        }
    }

    /// Apply color changes to the theme manager
    fn apply_color_change(&mut self, context: ColorContext, color: cosmic::iced::Color) {
        if let Some(theme_manager_mutex) = get_theme_manager() {
            let mut theme_manager_guard = theme_manager_mutex.lock().unwrap();
            
            if let Some(theme_manager) = theme_manager_guard.as_mut() {
                if let Some(staged) = theme_manager.set_color(Some(color), context) {
                    let _ = theme_manager.build_theme(staged);
                    log::info!("✅ Applied color change to theme");
                }
            }
        }
        
        // Store the color for UI updates
        self.custom_colors.insert(context, color);
    }

    /// Get internationalization strings
    fn i18n() -> HashMap<String, String> {
        HashMap::from([
            ("auto".to_string(), "Auto".to_string()),
            ("auto_switch".to_string(), "Auto Switch".to_string()),
            ("accent_color".to_string(), "Accent Color".to_string()),
            ("app_bg".to_string(), "Application Background".to_string()),
            ("container_bg".to_string(), "Container Background".to_string()),
            ("text_tint".to_string(), "Interface Text".to_string()),
            ("control_tint".to_string(), "Control Components".to_string()),
            ("window_hint".to_string(), "Window Hint".to_string()),
            ("dark".to_string(), "Dark".to_string()),
            ("light".to_string(), "Light".to_string()),
        ])
    }
}
