// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    Element,
    iced::{
        Alignment, Length, Padding,
        widget::{button, container, row, Space, tooltip},
    },
    widget::{self, icon},
};

use crate::app::Message;
use crate::tab::{HeadingOptions, View};

#[derive(Clone, Debug)]
pub enum RibbonMessage {
    NewFile,
    NewFolder,
    Cut,
    Copy,
    Paste,
    Delete,
    MoveToTrash,
    OpenTerminal,
    ToggleSort,  // Cycles through sort options
    ToggleView,  // Cycles through view modes
    ShowHidden(bool),
    FoldersFirst(bool),
}

impl RibbonMessage {
    pub fn to_app_message(&self) -> Message {
        match self {
            RibbonMessage::NewFile => Message::NewItem(None, false),
            RibbonMessage::NewFolder => Message::NewItem(None, true),
            RibbonMessage::Cut => Message::Cut(None),
            RibbonMessage::Copy => Message::Copy(None),
            RibbonMessage::Paste => Message::Paste(None),
            RibbonMessage::Delete => Message::Delete(None),
            RibbonMessage::MoveToTrash => Message::Delete(None),
            RibbonMessage::OpenTerminal => Message::OpenTerminal(None),
            RibbonMessage::ToggleSort => {
                // This will be handled in the app's RibbonMessage handler
                Message::None
            }
            RibbonMessage::ToggleView => {
                // This will be handled in the app's RibbonMessage handler
                Message::None
            }
            RibbonMessage::ShowHidden(show) => {
                if *show {
                    Message::ToggleShowHidden
                } else {
                    Message::None
                }
            }
            RibbonMessage::FoldersFirst(folders_first) => {
                if *folders_first {
                    Message::ToggleFoldersFirst
                } else {
                    Message::None
                }
            }
        }
    }
}

pub struct RibbonToolbar {
    current_view: View,
    current_sort: HeadingOptions,
    show_hidden: bool,
    folders_first: bool,
}

impl Default for RibbonToolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl RibbonToolbar {
    pub fn new() -> Self {
        Self {
            current_view: View::Grid,
            current_sort: HeadingOptions::Name,
            show_hidden: false,
            folders_first: false,
        }
    }

    pub fn set_view(&mut self, view: View) {
        self.current_view = view;
    }

    pub fn set_sort(&mut self, sort: HeadingOptions) {
        self.current_sort = sort;
    }

    pub fn get_view(&self) -> View {
        log::debug!("📖 RibbonToolbar::get_view() = {:?}", self.current_view);
        self.current_view
    }

    pub fn get_sort(&self) -> HeadingOptions {
        log::debug!("📖 RibbonToolbar::get_sort() = {:?}", self.current_sort);
        self.current_sort
    }

    pub fn update(&mut self, message: RibbonMessage) {
        match message {
            RibbonMessage::ShowHidden(show) => {
                log::debug!("RibbonToolbar: ShowHidden = {}", show);
                self.show_hidden = show;
            }
            RibbonMessage::FoldersFirst(folders_first) => {
                log::debug!("RibbonToolbar: FoldersFirst = {}", folders_first);
                self.folders_first = folders_first;
            }
            RibbonMessage::ToggleView => {
                let old_view = self.current_view;
                // Cycle through view modes
                self.current_view = match self.current_view {
                    View::Grid => View::List,
                    View::List => View::Grid,
                };
                log::debug!("🔄 RibbonToolbar::ToggleView - OLD: {:?} -> NEW: {:?}", old_view, self.current_view);
            }
            RibbonMessage::ToggleSort => {
                let old_sort = self.current_sort;
                // Cycle through sort options
                self.current_sort = match self.current_sort {
                    HeadingOptions::Name => HeadingOptions::Modified,
                    HeadingOptions::Modified => HeadingOptions::Size,
                    HeadingOptions::Size => HeadingOptions::TrashedOn,
                    HeadingOptions::TrashedOn => HeadingOptions::Name,
                };
                log::debug!("⇅ RibbonToolbar::ToggleSort - OLD: {:?} -> NEW: {:?}", old_sort, self.current_sort);
            }
            _ => {
                log::debug!("RibbonToolbar: Other message: {:?}", message);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let toolbar_row = row![
            // New File button
            self.new_file_button(),
            Space::with_width(Length::Fixed(4.0)),

            // New Folder button
            self.new_folder_button(),
            Space::with_width(Length::Fixed(12.0)),

            // Cut, Copy, Paste buttons
            self.action_buttons(),
            Space::with_width(Length::Fixed(12.0)),

            // View toggle (Grid/List)
            self.view_toggle(),
            Space::with_width(Length::Fixed(4.0)),

            // Sort toggle
            self.sort_toggle(),
            Space::with_width(Length::Fixed(12.0)),

            // Move to trash button
            self.trash_button(),
            Space::with_width(Length::Fixed(4.0)),

            // Terminal button
            self.terminal_button(),
        ]
        .align_y(Alignment::Center)
        .spacing(0)
        .padding(Padding::from([8, 12, 8, 12]));

        container(toolbar_row)
            .width(Length::Fill)
            .into()
    }

    fn new_file_button(&self) -> Element<'_, Message> {
        tooltip(
            button(
                container(icon::from_name("document-new-symbolic").size(16))
                    .width(Length::Fixed(28.0))
                    .height(Length::Fixed(28.0))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_theme| {
                        let mut style = widget::container::Style::default();
                        style.background = None; // Transparent background
                        style
                    })
            )
            .on_press(RibbonMessage::NewFile.to_app_message()),
            "New File",
            tooltip::Position::Bottom
        )
        .into()
    }

    fn new_folder_button(&self) -> Element<'_, Message> {
        tooltip(
            button(
                container(icon::from_name("folder-new-symbolic").size(16))
                    .width(Length::Fixed(28.0))
                    .height(Length::Fixed(28.0))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_theme| {
                        let mut style = widget::container::Style::default();
                        style.background = None; // Transparent background
                        style
                    })
            )
            .on_press(RibbonMessage::NewFolder.to_app_message()),
            "New Folder",
            tooltip::Position::Bottom
        )
        .into()
    }

    fn action_buttons(&self) -> Element<'_, Message> {
        row![
            // Cut button
            tooltip(
                button(
                    container(icon::from_name("edit-cut-symbolic").size(16))
                        .width(Length::Fixed(28.0))
                        .height(Length::Fixed(28.0))
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .style(|_theme| {
                            let mut style = widget::container::Style::default();
                            style.background = None; // Transparent background
                            style
                        })
                )
                .on_press(RibbonMessage::Cut.to_app_message()),
                "Cut (Ctrl+X)",
                tooltip::Position::Bottom
            ),

            // Copy button
            tooltip(
                button(
                    container(icon::from_name("edit-copy-symbolic").size(16))
                        .width(Length::Fixed(28.0))
                        .height(Length::Fixed(28.0))
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .style(|_theme| {
                            let mut style = widget::container::Style::default();
                            style.background = None; // Transparent background
                            style
                        })
                )
                .on_press(RibbonMessage::Copy.to_app_message()),
                "Copy (Ctrl+C)",
                tooltip::Position::Bottom
            ),

            // Paste button
            tooltip(
                button(
                    container(icon::from_name("edit-paste-symbolic").size(16))
                        .width(Length::Fixed(28.0))
                        .height(Length::Fixed(28.0))
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .style(|_theme| {
                            let mut style = widget::container::Style::default();
                            style.background = None; // Transparent background
                            style
                        })
                )
                .on_press(RibbonMessage::Paste.to_app_message()),
                "Paste (Ctrl+V)",
                tooltip::Position::Bottom
            ),
        ]
        .spacing(4)
        .into()
    }

    fn view_toggle(&self) -> Element<'_, Message> {
        let (icon_name, label) = match self.current_view {
            View::Grid => ("view-grid-symbolic", "Grid View (click to toggle to List)"),
            View::List => ("view-list-symbolic", "List View (click to toggle to Grid)"),
        };

        tooltip(
            button(
                container(icon::from_name(icon_name).size(16))
                    .width(Length::Fixed(28.0))
                    .height(Length::Fixed(28.0))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_theme| {
                        let mut style = widget::container::Style::default();
                        style.background = None; // Transparent background
                        style
                    })
            )
            .on_press(Message::RibbonMessage(RibbonMessage::ToggleView)),
            label,
            tooltip::Position::Bottom
        )
        .into()
    }

    fn sort_toggle(&self) -> Element<'_, Message> {
        let sort_label = match self.current_sort {
            HeadingOptions::Name => "Sort by Name",
            HeadingOptions::Modified => "Sort by Date",
            HeadingOptions::Size => "Sort by Size",
            HeadingOptions::TrashedOn => "Sort by Trashed",
        };

        tooltip(
            button(
                container(icon::from_name("view-sort-ascending-symbolic").size(16))
                    .width(Length::Fixed(28.0))
                    .height(Length::Fixed(28.0))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_theme| {
                        let mut style = widget::container::Style::default();
                        style.background = None; // Transparent background
                        style
                    })
            )
            .on_press(Message::RibbonMessage(RibbonMessage::ToggleSort)),
            widget::text(format!("{} (click to cycle)", sort_label)),
            tooltip::Position::Bottom
        )
        .into()
    }

    fn trash_button(&self) -> Element<'_, Message> {
        tooltip(
            button(
                container(icon::from_name("user-trash-symbolic").size(16))
                    .width(Length::Fixed(28.0))
                    .height(Length::Fixed(28.0))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_theme| {
                        let mut style = widget::container::Style::default();
                        style.background = None; // Transparent background
                        style
                    })
            )
            .on_press(RibbonMessage::MoveToTrash.to_app_message()),
            "Move to Trash (Delete)",
            tooltip::Position::Bottom
        )
        .into()
    }

    fn terminal_button(&self) -> Element<'_, Message> {
        tooltip(
            button(
                container(icon::from_name("utilities-terminal-symbolic").size(16))
                    .width(Length::Fixed(28.0))
                    .height(Length::Fixed(28.0))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_theme| {
                        let mut style = widget::container::Style::default();
                        style.background = None; // Transparent background
                        style
                    })
            )
            .on_press(RibbonMessage::OpenTerminal.to_app_message()),
            "Open Terminal",
            tooltip::Position::Bottom
        )
        .into()
    }
}

