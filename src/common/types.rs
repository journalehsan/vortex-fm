// Shared types for Vortex File Manager

use std::path::PathBuf;

/// File system item types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    File,
    Directory,
    Symlink,
    Other,
}

/// View modes for the file manager
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Grid,
    List,
    Details,
}

/// Sort options for file listings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOption {
    Name,
    Size,
    Modified,
    Type,
    Extension,
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// File operation types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileOperation {
    Copy,
    Move,
    Delete,
    Rename,
    Create,
}

/// Navigation direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationDirection {
    Back,
    Forward,
    Up,
    Down,
}

/// File selection state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionState {
    pub selected_paths: Vec<PathBuf>,
    pub primary_selection: Option<PathBuf>,
    pub selection_mode: SelectionMode,
}

/// Selection mode for files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Single,
    Multiple,
    Range,
}

/// File metadata
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub modified: std::time::SystemTime,
    pub permissions: u32,
    pub item_type: ItemType,
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub name: String,
    pub match_score: f32,
    pub context: Option<String>,
}

/// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    pub current_path: PathBuf,
    pub view_mode: ViewMode,
    pub sort_option: SortOption,
    pub sort_direction: SortDirection,
    pub show_hidden: bool,
    pub selection: SelectionState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            view_mode: ViewMode::Grid,
            sort_option: SortOption::Name,
            sort_direction: SortDirection::Ascending,
            show_hidden: false,
            selection: SelectionState {
                selected_paths: Vec::new(),
                primary_selection: None,
                selection_mode: SelectionMode::Single,
            },
        }
    }
}
