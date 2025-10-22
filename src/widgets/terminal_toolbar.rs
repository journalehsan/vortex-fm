// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    Element,
    iced::{
        Alignment, Length, Padding,
        widget::{button, container, row, Space, tooltip, text},
    },
    widget::{self, icon},
};

use crate::common::terminal_types::{TerminalPosition, TerminalMessage};

/// Terminal toolbar widget with controls
pub struct TerminalToolbar {
    position: TerminalPosition,
    current_path: String,
    is_synced: bool,
}

impl Default for TerminalToolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalToolbar {
    pub fn new() -> Self {
        Self {
            position: TerminalPosition::Bottom,
            current_path: "~".to_string(),
            is_synced: false,
        }
    }

    pub fn set_position(&mut self, position: TerminalPosition) {
        self.position = position;
    }

    pub fn set_current_path(&mut self, path: String) {
        self.current_path = path;
    }

    pub fn set_synced(&mut self, synced: bool) {
        self.is_synced = synced;
    }

    pub fn view(&self) -> Element<'_, TerminalMessage> {
        let toolbar_row = row![
            // Current directory display
            self.path_display(),
            Space::with_width(Length::Fixed(8.0)),

            // Position toggle button
            self.position_toggle_button(),
            Space::with_width(Length::Fixed(4.0)),

            // Sync button
            self.sync_button(),
        ]
        .align_y(Alignment::Center)
        .spacing(0)
        .padding(Padding::from([4, 8, 4, 8]));

        container(toolbar_row)
            .width(Length::Fill)
            .into()
    }

    fn path_display(&self) -> Element<'_, TerminalMessage> {
        let path_text = if self.current_path.len() > 50 {
            format!("...{}", &self.current_path[self.current_path.len() - 47..])
        } else {
            self.current_path.clone()
        };

        row![
            icon::from_name("folder-symbolic").size(14),
            Space::with_width(Length::Fixed(4.0)),
            text(path_text).size(12)
        ]
        .align_y(Alignment::Center)
        .into()
    }

    fn position_toggle_button(&self) -> Element<'_, TerminalMessage> {
        let (icon_name, tooltip_text) = match self.position {
            TerminalPosition::Bottom => ("view-sort-descending-symbolic", "Move to Right Panel"),
            TerminalPosition::Right => ("view-sort-ascending-symbolic", "Move to Bottom Panel"),
        };

        tooltip(
            button(
                container(icon::from_name(icon_name).size(14))
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_theme| {
                        let mut style = widget::container::Style::default();
                        style.background = None;
                        style
                    })
            )
            .on_press(TerminalMessage::TogglePosition),
            tooltip_text,
            tooltip::Position::Top
        )
        .into()
    }

    fn sync_button(&self) -> Element<'_, TerminalMessage> {
        let (icon_name, tooltip_text) = if self.is_synced {
            ("view-refresh-symbolic", "Synced with current directory")
        } else {
            ("view-refresh-symbolic", "Sync with current directory")
        };

        tooltip(
            button(
                container(icon::from_name(icon_name).size(14))
                    .width(Length::Fixed(24.0))
                    .height(Length::Fixed(24.0))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_theme| {
                        let mut style = widget::container::Style::default();
                        style.background = None;
                        style
                    })
            )
            .on_press(TerminalMessage::SyncDirectory),
            tooltip_text,
            tooltip::Position::Top
        )
        .into()
    }
}
