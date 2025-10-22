// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

// Example of how to integrate the ribbon toolbar into the main app
// This file shows how to use the RibbonToolbar in your application

use cosmic::Element;
use crate::{
    app::Message,
    views::ribbon_toolbar::{RibbonMessage, RibbonToolbar},
};

/// Example of how to integrate the ribbon toolbar into your app
pub struct AppWithRibbon {
    ribbon_toolbar: RibbonToolbar,
    // ... other app fields
}

impl AppWithRibbon {
    pub fn new() -> Self {
        Self {
            ribbon_toolbar: RibbonToolbar::new(),
            // ... initialize other fields
        }
    }

    /// Example of how to handle ribbon toolbar messages
    pub fn update_ribbon(&mut self, message: RibbonMessage) -> Message {
        // Update the toolbar state
        self.ribbon_toolbar.update(message.clone());
        
        // Convert ribbon message to app message
        message.to_app_message()
    }

    /// Example of how to include the ribbon toolbar in your view
    pub fn view_with_ribbon(&self) -> Element<'_, Message> {
        // Return just the ribbon toolbar for now
        // In a real implementation, you would combine it with your main content
        self.ribbon_toolbar.view()
    }
}

/// Example of how to add the ribbon toolbar to the main app's header
/// Note: In practice, you would store the RibbonToolbar in your app struct
/// and call ribbon.view() from there to avoid lifetime issues

/// Example of how to handle ribbon messages in your app's update function
pub fn handle_ribbon_message(message: RibbonMessage) -> Message {
    match message {
        RibbonMessage::NewFile => {
            // Handle new file creation
            Message::NewItem(None, false)
        }
        RibbonMessage::NewFolder => {
            // Handle new folder creation
            Message::NewItem(None, true)
        }
        RibbonMessage::Cut => {
            // Handle cut operation
            Message::Cut(None)
        }
        RibbonMessage::Copy => {
            // Handle copy operation
            Message::Copy(None)
        }
        RibbonMessage::Paste => {
            // Handle paste operation
            Message::Paste(None)
        }
        RibbonMessage::Delete => {
            // Handle delete operation
            Message::Delete(None)
        }
        RibbonMessage::MoveToTrash => {
            // Handle move to trash
            Message::Delete(None)
        }
        RibbonMessage::OpenTerminal => {
            // Handle open terminal
            Message::OpenTerminal(None)
        }
        RibbonMessage::ToggleView => {
            // Handle view toggle (cycles between Grid and List)
            // This is handled in the app.rs RibbonMessage handler
            Message::None
        }
        RibbonMessage::ToggleSort => {
            // Handle sort toggle (cycles through sort options)
            // This is handled in the app.rs RibbonMessage handler
            Message::None
        }
        RibbonMessage::ShowHidden(show) => {
            if show {
                Message::ToggleShowHidden
            } else {
                Message::None
            }
        }
        RibbonMessage::FoldersFirst(folders_first) => {
            if folders_first {
                Message::ToggleFoldersFirst
            } else {
                Message::None
            }
        }
    }
}
