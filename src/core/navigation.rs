// Navigation logic for Vortex File Manager

use std::collections::VecDeque;
use std::path::PathBuf;

/// Navigation history for back/forward functionality
#[derive(Debug, Clone)]
pub struct NavigationHistory {
    /// History of visited locations
    history: VecDeque<PathBuf>,
    /// Current position in history
    current_index: usize,
}

impl NavigationHistory {
    /// Create a new navigation history
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            current_index: 0,
        }
    }

    /// Add a new location to history
    pub fn push(&mut self, location: PathBuf) {
        // Remove any history after current position
        while self.history.len() > self.current_index + 1 {
            self.history.pop_back();
        }
        
        // Add new location
        self.history.push_back(location);
        self.current_index = self.history.len() - 1;
    }

    /// Go to previous location in history
    pub fn go_previous(&mut self) -> Option<PathBuf> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.history.get(self.current_index).cloned()
        } else {
            None
        }
    }

    /// Go to next location in history
    pub fn go_next(&mut self) -> Option<PathBuf> {
        if self.current_index + 1 < self.history.len() {
            self.current_index += 1;
            self.history.get(self.current_index).cloned()
        } else {
            None
        }
    }

    /// Get current location
    pub fn current(&self) -> Option<&PathBuf> {
        self.history.get(self.current_index)
    }

    /// Check if we can go back
    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    /// Check if we can go forward
    pub fn can_go_forward(&self) -> bool {
        self.current_index + 1 < self.history.len()
    }

    /// Get parent directory of current location
    pub fn parent(&self) -> Option<PathBuf> {
        self.current()
            .and_then(|path| path.parent())
            .map(|p| p.to_path_buf())
    }
}

impl Default for NavigationHistory {
    fn default() -> Self {
        Self::new()
    }
}
