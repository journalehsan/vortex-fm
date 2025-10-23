// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    Element,
    iced::{
        Alignment, Length,
        widget::{container, text, text_input, scrollable, column, row},
    },
    widget::{self},
};

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::common::terminal_types::{
    TerminalPosition, TerminalBackend, TerminalMessage, TerminalOutputLine,
};
use crate::core::terminal::{TerminalStrategy, TerminalStrategyFactory, TerminalSessionManager};
use crate::utils::terminal_utils;

/// Main terminal panel widget
pub struct TerminalPanel {
    strategy: Box<dyn TerminalStrategy>,
    session_manager: Arc<Mutex<TerminalSessionManager>>,
    position: TerminalPosition,
    is_visible: bool,
    current_dir: PathBuf,
    
    // Fallback terminal state
    command_input: String,
    output_buffer: Vec<TerminalOutputLine>,
    scroll_offset: usize,
}

impl Default for TerminalPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalPanel {
    pub fn new() -> Self {
        let strategy = TerminalStrategyFactory::create_best_strategy();
        let session_manager = Arc::new(Mutex::new(TerminalSessionManager::new()));
        
        Self {
            strategy,
            session_manager,
            position: TerminalPosition::Bottom,
            is_visible: false,
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/tmp")),
            command_input: String::new(),
            output_buffer: Vec::new(),
            scroll_offset: 0,
        }
    }

    pub fn set_position(&mut self, position: TerminalPosition) {
        self.position = position;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn sync_directory(&mut self, path: &PathBuf) -> Result<(), String> {
        let safe_path = terminal_utils::get_safe_working_directory(path);
        self.current_dir = safe_path.clone();
        
        // Update session manager if it exists
        let session_manager = self.session_manager.clone();
        let path = safe_path.clone();
        tokio::spawn(async move {
            let mut manager = session_manager.lock().await;
            let _ = manager.update_directory(path);
        });

        Ok(())
    }

    pub fn toggle_position(&mut self) {
        self.position = match self.position {
            TerminalPosition::Bottom => TerminalPosition::Right,
            TerminalPosition::Right => TerminalPosition::Bottom,
        };
    }

    pub fn view(&self) -> Element<'_, TerminalMessage> {
        // Simple terminal panel with toolbar for now
        widget::container(
            widget::text(format!("🖥️ Terminal Panel\n📁 Current Dir: {}\n\nTerminal output will appear here...\n\nToolbar: Position: {:?} | Sync Button", self.current_dir.display(), self.position))
                .size(14)
                .font(cosmic::iced::Font::MONOSPACE)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(16)
        .style(|_theme| {
            let mut style = widget::container::Style::default();
            style.background = Some(cosmic::iced::Background::Color(cosmic::iced::Color::from_rgb(0.02, 0.02, 0.02)));
            style.border = cosmic::iced::Border {
                radius: 4.0.into(),
                width: 1.0,
                color: cosmic::iced::Color::from_rgb(0.3, 0.3, 0.3),
            };
            style
        })
        .into()
    }


    pub fn get_current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub async fn execute_command(&mut self, command: &str) -> Result<(), String> {
        if !terminal_utils::validate_command(command) {
            return Err("Command validation failed".to_string());
        }

        let session_manager = self.session_manager.clone();
        let command = command.to_string();
        
        tokio::spawn(async move {
            let mut manager = session_manager.lock().await;
            let _ = manager.execute_command(&command).await;
        });

        Ok(())
    }


    fn fallback_terminal_view(&self) -> Element<'_, TerminalMessage> {
        let output_area = self.output_display();
        let input_area = self.command_input_area();

        let content = column![
            output_area,
            input_area,
        ]
        .spacing(4);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn embedded_terminal_view(&self) -> Element<'_, TerminalMessage> {
        // For embedded terminals, we'd show a placeholder or embed the actual terminal
        // This is a simplified version
        container(
            column![
                text("Terminal (Embedded)").size(14),
                text(format!("Working directory: {}", self.current_dir.display())).size(12),
            ]
            .spacing(8)
            .padding(16)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn output_display(&self) -> Element<'_, TerminalMessage> {
        let output_text = if self.output_buffer.is_empty() {
            "Terminal ready. Type a command and press Enter.".to_string()
        } else {
            self.output_buffer
                .iter()
                .map(|line| {
                    if line.is_error {
                        format!("[ERROR] {}\n", line.content)
                    } else {
                        format!("{}\n", line.content)
                    }
                })
                .collect::<String>()
        };

        scrollable(
            text(output_text)
                .size(12)
                .font(cosmic::iced::Font::MONOSPACE)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn command_input_area(&self) -> Element<'_, TerminalMessage> {
        row![
            text("$ ")
                .size(12)
                .font(cosmic::iced::Font::MONOSPACE),
            text_input("Enter command...", &self.command_input)
                .on_input(TerminalMessage::CommandInput)
                .on_submit(TerminalMessage::ExecuteCommand(String::new()))
                .size(12)
                .font(cosmic::iced::Font::MONOSPACE)
                .width(Length::Fill),
        ]
        .align_y(Alignment::Center)
        .spacing(4)
        .into()
    }

    pub fn update(&mut self, message: TerminalMessage) {
        match message {
            TerminalMessage::CommandInput(input) => {
                self.command_input = input;
            }
            TerminalMessage::ExecuteCommand(_) => {
                if !self.command_input.trim().is_empty() {
                    let command = self.command_input.clone();
                    self.command_input.clear();
                    
                    // Add command to output buffer
                    self.output_buffer.push(TerminalOutputLine::new(
                        format!("$ {}", command),
                        false,
                    ));

                    // Execute command asynchronously
                    let session_manager = self.session_manager.clone();
                    tokio::spawn(async move {
                        let mut manager = session_manager.lock().await;
                        let _ = manager.execute_command(&command).await;
                    });
                }
            }
            TerminalMessage::TogglePosition => {
                self.toggle_position();
            }
            TerminalMessage::SyncDirectory => {
                // This would be handled by the parent component
            }
            _ => {
                // Handle other messages
            }
        }
    }
}
