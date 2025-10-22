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
    SortBy(String),
    ViewMode(String),
    ShowHidden(bool),
    FoldersFirst(bool),
    ToggleNewDropdown,
    ToggleSortDropdown,
    ToggleViewDropdown,
    CloseDropdowns,
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
            RibbonMessage::SortBy(sort_type) => {
                // Map string to actual sort action
                match sort_type.as_str() {
                    "name" => Message::TabMessage(None, crate::tab::Message::SetSort(crate::tab::HeadingOptions::Name, false)),
                    "size" => Message::TabMessage(None, crate::tab::Message::SetSort(crate::tab::HeadingOptions::Size, false)),
                    "modified" => Message::TabMessage(None, crate::tab::Message::SetSort(crate::tab::HeadingOptions::Modified, false)),
                    "trashed" => Message::TabMessage(None, crate::tab::Message::SetSort(crate::tab::HeadingOptions::TrashedOn, false)),
                    _ => Message::None,
                }
            }
            RibbonMessage::ViewMode(view_mode) => {
                match view_mode.as_str() {
                    "list" => Message::TabView(None, crate::tab::View::List),
                    "grid" => Message::TabView(None, crate::tab::View::Grid),
                    _ => Message::None,
                }
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
            RibbonMessage::ToggleNewDropdown => Message::None,
            RibbonMessage::ToggleSortDropdown => Message::None,
            RibbonMessage::ToggleViewDropdown => Message::None,
            RibbonMessage::CloseDropdowns => Message::None,
        }
    }
}

pub struct RibbonToolbar {
    sort_options: Vec<String>,
    view_options: Vec<String>,
    show_hidden: bool,
    folders_first: bool,
    new_dropdown_open: bool,
    sort_dropdown_open: bool,
    view_dropdown_open: bool,
}

impl Default for RibbonToolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl RibbonToolbar {
    pub fn new() -> Self {
        Self {
            sort_options: vec![
                "name".to_string(),
                "size".to_string(),
                "date".to_string(),
                "type".to_string(),
            ],
            view_options: vec![
                "list".to_string(),
                "grid".to_string(),
            ],
            show_hidden: false,
            folders_first: false,
            new_dropdown_open: false,
            sort_dropdown_open: false,
            view_dropdown_open: false,
        }
    }

    pub fn update(&mut self, message: RibbonMessage) {
        match message {
            RibbonMessage::ShowHidden(show) => {
                self.show_hidden = show;
            }
            RibbonMessage::FoldersFirst(folders_first) => {
                self.folders_first = folders_first;
            }
            RibbonMessage::ToggleNewDropdown => {
                self.new_dropdown_open = !self.new_dropdown_open;
                // Close other dropdowns
                self.sort_dropdown_open = false;
                self.view_dropdown_open = false;
            }
            RibbonMessage::ToggleSortDropdown => {
                self.sort_dropdown_open = !self.sort_dropdown_open;
                // Close other dropdowns
                self.new_dropdown_open = false;
                self.view_dropdown_open = false;
            }
            RibbonMessage::ToggleViewDropdown => {
                self.view_dropdown_open = !self.view_dropdown_open;
                // Close other dropdowns
                self.new_dropdown_open = false;
                self.sort_dropdown_open = false;
            }
            RibbonMessage::CloseDropdowns => {
                self.new_dropdown_open = false;
                self.sort_dropdown_open = false;
                self.view_dropdown_open = false;
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let toolbar_row = row![
            // New dropdown
            container(self.new_dropdown())
                .align_x(Alignment::Start),
            Space::with_width(Length::Fixed(8.0)),

            // Cut, Copy, Paste buttons
            self.action_buttons(),
            Space::with_width(Length::Fixed(8.0)),

            // Sort dropdown
            container(self.sort_dropdown())
                .align_x(Alignment::Start),
            Space::with_width(Length::Fixed(8.0)),

            // View dropdown
            container(self.view_dropdown())
                .align_x(Alignment::Start),
            Space::with_width(Length::Fixed(8.0)),

            // Move to trash button
            self.trash_button(),
            Space::with_width(Length::Fixed(8.0)),

            // Terminal button
            self.terminal_button(),
        ]
        .align_y(Alignment::Center)
        .spacing(4)
        .padding(Padding::from([8, 12, 8, 12]));

        container(toolbar_row)
            .width(Length::Fill)
            .into()
    }

    fn new_dropdown(&self) -> Element<'_, Message> {
        let new_button = button("New")
            .on_press(RibbonMessage::ToggleNewDropdown.to_app_message());

        if self.new_dropdown_open {
            widget::column::with_children(vec![
                tooltip(new_button, "New (Ctrl+Shift+N)", tooltip::Position::Bottom).into(),
                Space::with_height(Length::Fixed(4.0)).into(),
                button("New File")
                    .on_press(RibbonMessage::NewFile.to_app_message())
                    .into(),
                Space::with_height(Length::Fixed(4.0)).into(),
                button("New Folder")
                    .on_press(RibbonMessage::NewFolder.to_app_message())
                    .into(),
            ])
            .spacing(4)
            .into()
        } else {
            tooltip(new_button, "New (Ctrl+Shift+N)", tooltip::Position::Bottom)
                .into()
        }
    }

    fn action_buttons(&self) -> Element<'_, Message> {
        row![
            // Cut button
            tooltip(
                button(icon::from_name("edit-cut-symbolic").size(16))
                    .on_press(RibbonMessage::Cut.to_app_message()),
                "Cut (Ctrl+X)",
                tooltip::Position::Bottom
            ),

            // Copy button
            tooltip(
                button(icon::from_name("edit-copy-symbolic").size(16))
                    .on_press(RibbonMessage::Copy.to_app_message()),
                "Copy (Ctrl+C)",
                tooltip::Position::Bottom
            ),

            // Paste button
            tooltip(
                button(icon::from_name("edit-paste-symbolic").size(16))
                    .on_press(RibbonMessage::Paste.to_app_message()),
                "Paste (Ctrl+V)",
                tooltip::Position::Bottom
            ),
        ]
        .spacing(4)
        .into()
    }

    fn sort_dropdown(&self) -> Element<'_, Message> {
        let sort_button = button("Sort")
            .on_press(RibbonMessage::ToggleSortDropdown.to_app_message());

        if self.sort_dropdown_open {
            widget::column::with_children(vec![
                tooltip(sort_button, "Sort Options", tooltip::Position::Bottom).into(),
                Space::with_height(Length::Fixed(4.0)).into(),
                button("Name")
                    .on_press(RibbonMessage::SortBy("name".to_string()).to_app_message())
                    .into(),
                Space::with_height(Length::Fixed(4.0)).into(),
                button("Size")
                    .on_press(RibbonMessage::SortBy("size".to_string()).to_app_message())
                    .into(),
                Space::with_height(Length::Fixed(4.0)).into(),
                button("Date")
                    .on_press(RibbonMessage::SortBy("date".to_string()).to_app_message())
                    .into(),
                Space::with_height(Length::Fixed(4.0)).into(),
                button("Type")
                    .on_press(RibbonMessage::SortBy("type".to_string()).to_app_message())
                    .into(),
            ])
            .spacing(4)
            .into()
        } else {
            tooltip(sort_button, "Sort Options", tooltip::Position::Bottom)
                .into()
        }
    }

    fn view_dropdown(&self) -> Element<'_, Message> {
        let view_button = button("View")
            .on_press(RibbonMessage::ToggleViewDropdown.to_app_message());

        if self.view_dropdown_open {
            widget::column::with_children(vec![
                tooltip(view_button, "View Options", tooltip::Position::Bottom).into(),
                Space::with_height(Length::Fixed(4.0)).into(),
                button("List")
                    .on_press(RibbonMessage::ViewMode("list".to_string()).to_app_message())
                    .into(),
                Space::with_height(Length::Fixed(4.0)).into(),
                button("Grid")
                    .on_press(RibbonMessage::ViewMode("grid".to_string()).to_app_message())
                    .into(),
            ])
            .spacing(4)
            .into()
        } else {
            tooltip(view_button, "View Options", tooltip::Position::Bottom)
                .into()
        }
    }

    fn trash_button(&self) -> Element<'_, Message> {
        tooltip(
            button(icon::from_name("user-trash-symbolic").size(16))
                .on_press(RibbonMessage::MoveToTrash.to_app_message()),
            "Move to Trash (Delete)",
            tooltip::Position::Bottom
        )
        .into()
    }

    fn terminal_button(&self) -> Element<'_, Message> {
        tooltip(
            button(icon::from_name("utilities-terminal-symbolic").size(16))
                .on_press(RibbonMessage::OpenTerminal.to_app_message()),
            "Open Terminal",
            tooltip::Position::Bottom
        )
        .into()
    }
   }

