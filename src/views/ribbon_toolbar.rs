// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    Element,
    iced::{
        Alignment, Length, Padding,
        widget::{button, container, row, Space},
    },
    widget::icon,
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
        }
    }
}

pub struct RibbonToolbar {
    sort_options: Vec<String>,
    view_options: Vec<String>,
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
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let toolbar_row = row![
            // New dropdown
            self.new_dropdown(),
            Space::with_width(Length::Fixed(8.0)),
            
            // Cut, Copy, Paste buttons
            self.action_buttons(),
            Space::with_width(Length::Fixed(8.0)),
            
            // Sort dropdown
            self.sort_dropdown(),
            Space::with_width(Length::Fixed(8.0)),
            
            // View dropdown
            self.view_dropdown(),
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
            .on_press(RibbonMessage::NewFile.to_app_message());

        // For now, we'll use a simple button. In a full implementation,
        // this would be a dropdown with New File and New Folder options
        new_button.into()
    }

    fn action_buttons(&self) -> Element<'_, Message> {
        row![
            // Cut button
            button(icon::from_name("edit-cut").size(16))
                .on_press(RibbonMessage::Cut.to_app_message()),
            
            // Copy button
            button(icon::from_name("edit-copy").size(16))
                .on_press(RibbonMessage::Copy.to_app_message()),
            
            // Paste button
            button(icon::from_name("edit-paste").size(16))
                .on_press(RibbonMessage::Paste.to_app_message()),
        ]
        .spacing(4)
        .into()
    }

    fn sort_dropdown(&self) -> Element<'_, Message> {
        // For now, we'll use a simple button. In a full implementation,
        // this would be a dropdown with sort options
        button("Sort")
            .on_press(RibbonMessage::SortBy("name".to_string()).to_app_message())
            .into()
    }

    fn view_dropdown(&self) -> Element<'_, Message> {
        // For now, we'll use a simple button. In a full implementation,
        // this would be a dropdown with view options
        button("View")
            .on_press(RibbonMessage::ViewMode("list".to_string()).to_app_message())
            .into()
    }

    fn trash_button(&self) -> Element<'_, Message> {
        button(icon::from_name("user-trash").size(16))
            .on_press(RibbonMessage::MoveToTrash.to_app_message())
            .into()
    }

    fn terminal_button(&self) -> Element<'_, Message> {
        button(icon::from_name("utilities-terminal").size(16))
            .on_press(RibbonMessage::OpenTerminal.to_app_message())
            .into()
    }
}

