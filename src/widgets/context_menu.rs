// Context menu widget for Vortex File Manager

use cosmic::{
    Element,
    widget,
};

use crate::{
    app::Message,
    tab::Tab,
};

/// Context menu widget implementation
pub struct ContextMenuWidget {
    // TODO: Add context menu state
}

impl ContextMenuWidget {
    /// Create a new context menu widget
    pub fn new() -> Self {
        Self {
            // TODO: Initialize context menu state
        }
    }

    /// Build the context menu for a tab
    pub fn build(&self, _tab: &Tab) -> Element<'_, Message> {
        // TODO: Extract context menu logic from menu.rs
        // This will contain the right-click menu for files and folders
        widget::text("Context Menu Widget - TODO: Extract from menu.rs")
            .size(14)
            .into()
    }

    /// Build context menu for navigation items
    pub fn build_nav(&self) -> Element<'_, Message> {
        // TODO: Extract navigation context menu logic
        widget::text("Navigation Context Menu - TODO: Extract from menu.rs")
            .size(14)
            .into()
    }
}

impl Default for ContextMenuWidget {
    fn default() -> Self {
        Self::new()
    }
}
