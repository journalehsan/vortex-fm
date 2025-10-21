// Shared constants for Vortex File Manager

/// Application metadata
pub const APP_NAME: &str = "Vortex File Manager";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHOR: &str = "Ehsan Tork";

/// File system constants
pub const MAX_PATH_LENGTH: usize = 4096;
pub const MAX_FILENAME_LENGTH: usize = 255;
pub const HIDDEN_FILE_PREFIX: char = '.';

/// UI constants
pub const DEFAULT_ICON_SIZE: u32 = 48;
pub const MIN_ICON_SIZE: u32 = 16;
pub const MAX_ICON_SIZE: u32 = 256;
pub const DEFAULT_GRID_SPACING: u32 = 8;
pub const DEFAULT_LIST_ROW_HEIGHT: u32 = 32;

/// Navigation constants
pub const MAX_HISTORY_SIZE: usize = 100;
pub const MAX_RECENT_PATHS: usize = 20;
pub const MAX_BOOKMARKS: usize = 50;

/// File operation constants
pub const MAX_CONCURRENT_OPERATIONS: usize = 10;
pub const OPERATION_TIMEOUT_SECONDS: u64 = 300; // 5 minutes
pub const MAX_FILE_SIZE_FOR_PREVIEW: u64 = 10 * 1024 * 1024; // 10MB

/// Search constants
pub const MAX_SEARCH_RESULTS: usize = 1000;
pub const MIN_SEARCH_QUERY_LENGTH: usize = 2;
pub const SEARCH_TIMEOUT_MS: u64 = 500;

/// Thumbnail constants
pub const THUMBNAIL_SIZE: u32 = 128;
pub const THUMBNAIL_CACHE_SIZE: usize = 1000;
pub const THUMBNAIL_CACHE_TTL_SECONDS: u64 = 3600; // 1 hour

/// Keyboard shortcuts
pub const KEY_COPY: &str = "Ctrl+C";
pub const KEY_CUT: &str = "Ctrl+X";
pub const KEY_PASTE: &str = "Ctrl+V";
pub const KEY_DELETE: &str = "Delete";
pub const KEY_RENAME: &str = "F2";
pub const KEY_REFRESH: &str = "F5";
pub const KEY_NEW_FOLDER: &str = "Ctrl+Shift+N";
pub const KEY_SELECT_ALL: &str = "Ctrl+A";
pub const KEY_INVERT_SELECTION: &str = "Ctrl+I";

/// File type icons (emoji)
pub const ICON_FOLDER: &str = "📁";
pub const ICON_FILE: &str = "📄";
pub const ICON_IMAGE: &str = "🖼️";
pub const ICON_VIDEO: &str = "🎥";
pub const ICON_AUDIO: &str = "🎵";
pub const ICON_ARCHIVE: &str = "📦";
pub const ICON_EXECUTABLE: &str = "⚙️";
pub const ICON_DOCUMENT: &str = "📄";
pub const ICON_CODE: &str = "💻";
pub const ICON_PDF: &str = "📕";
pub const ICON_SPREADSHEET: &str = "📊";
pub const ICON_PRESENTATION: &str = "📽️";

/// MIME type categories
pub const MIME_CATEGORY_IMAGE: &str = "image/";
pub const MIME_CATEGORY_VIDEO: &str = "video/";
pub const MIME_CATEGORY_AUDIO: &str = "audio/";
pub const MIME_CATEGORY_ARCHIVE: &str = "application/zip";
pub const MIME_CATEGORY_DOCUMENT: &str = "text/";
pub const MIME_CATEGORY_CODE: &str = "text/plain";

/// Error messages
pub const ERROR_FILE_NOT_FOUND: &str = "File not found";
pub const ERROR_PERMISSION_DENIED: &str = "Permission denied";
pub const ERROR_DISK_FULL: &str = "Disk full";
pub const ERROR_INVALID_PATH: &str = "Invalid path";
pub const ERROR_OPERATION_FAILED: &str = "Operation failed";
pub const ERROR_UNKNOWN: &str = "Unknown error";

/// Success messages
pub const SUCCESS_FILE_COPIED: &str = "File copied successfully";
pub const SUCCESS_FILE_MOVED: &str = "File moved successfully";
pub const SUCCESS_FILE_DELETED: &str = "File deleted successfully";
pub const SUCCESS_FILE_RENAMED: &str = "File renamed successfully";
pub const SUCCESS_FOLDER_CREATED: &str = "Folder created successfully";
