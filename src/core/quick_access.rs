// Quick Access core functionality for Vortex File Manager

use std::{
    path::PathBuf,
    time::SystemTime,
};

// Enhanced drive detection using comprehensive cross-platform detection

// Library folder entry
#[derive(Clone, Debug)]
pub struct LibraryFolder {
    pub name: String,
    pub path: PathBuf,
    pub icon: String,  // system icon name
}

// Drive entry with comprehensive drive information
#[derive(Clone, Debug)]
pub struct DriveInfo {
    pub label: String,
    pub path: PathBuf,
    pub total_space: Option<u64>,
    pub free_space: Option<u64>,
    pub is_accessible: bool,
    pub drive_type: DriveType,
}

#[derive(Clone, Debug)]
pub enum DriveType {
    FixedDisk,
    RemovableDisk,
    NetworkDrive,
    CDRom,
    RamDisk,
    Unknown,
}

// Recent file entry
#[derive(Clone, Debug)]
pub struct RecentFile {
    pub name: String,
    pub path: PathBuf,
    pub modified: SystemTime,
}

impl DriveInfo {
    pub fn usage_percent(&self) -> f32 {
        if let (Some(total), Some(free)) = (self.total_space, self.free_space) {
            if total == 0 { 0.0 }
            else { ((total - free) as f32 / total as f32) * 100.0 }
        } else {
            0.0
        }
    }

    pub fn progress_color(&self) -> cosmic::iced::Color {
        let pct = self.usage_percent();
        if pct >= 95.0 { cosmic::iced::Color::from_rgb(0.8, 0.0, 0.0) }      // red
        else if pct >= 80.0 { cosmic::iced::Color::from_rgb(1.0, 0.5, 0.0) } // orange
        else if pct >= 70.0 { cosmic::iced::Color::from_rgb(1.0, 0.8, 0.0) } // #ffcc00
        else if pct >= 50.0 { cosmic::iced::Color::from_rgb(0.0, 0.5, 1.0) } // blue
        else { cosmic::iced::Color::from_rgb(0.0, 0.8, 0.0) }                 // green
    }

    pub fn format_size(&self) -> String {
        match (self.total_space, self.free_space) {
            (Some(total), Some(free)) => {
                format!(
                    "{:.1} GB free of {:.1} GB",
                    free as f64 / 1_000_000_000.0,
                    total as f64 / 1_000_000_000.0
                )
            }
            _ => "Size unknown".to_string(),
        }
    }
}

// XDG User Directories Reading
pub fn get_library_folders() -> Vec<LibraryFolder> {
    let mut folders = Vec::new();

    log::debug!("get_library_folders called");

    // Desktop
    if let Some(path) = dirs::desktop_dir() {
        log::debug!("Found desktop directory: {:?}", path);
        folders.push(LibraryFolder {
            name: "Desktop".to_string(), // Will be localized in view
            path,
            icon: "user-desktop".to_string(),
        });
    }

    // Downloads
    if let Some(path) = dirs::download_dir() {
        log::debug!("Found downloads directory: {:?}", path);
        folders.push(LibraryFolder {
            name: "Downloads".to_string(),
            path,
            icon: "folder-download".to_string(),
        });
    } else {
        log::debug!("Downloads directory not found");
    }

    // Documents
    if let Some(path) = dirs::document_dir() {
        log::debug!("Found documents directory: {:?}", path);
        folders.push(LibraryFolder {
            name: "Documents".to_string(),
            path,
            icon: "folder-documents".to_string(),
        });
    } else {
        log::debug!("Documents directory not found");
    }

    // Music
    if let Some(path) = dirs::audio_dir() {
        log::debug!("Found music directory: {:?}", path);
        folders.push(LibraryFolder {
            name: "Music".to_string(),
            path,
            icon: "folder-music".to_string(),
        });
    } else {
        log::debug!("Music directory not found");
    }

    // Pictures
    if let Some(path) = dirs::picture_dir() {
        log::debug!("Found pictures directory: {:?}", path);
        folders.push(LibraryFolder {
            name: "Pictures".to_string(),
            path,
            icon: "folder-pictures".to_string(),
        });
    } else {
        log::debug!("Pictures directory not found");
    }

    // Videos
    if let Some(path) = dirs::video_dir() {
        log::debug!("Found videos directory: {:?}", path);
        folders.push(LibraryFolder {
            name: "Videos".to_string(),
            path,
            icon: "folder-videos".to_string(),
        });
    } else {
        log::debug!("Videos directory not found");
    }

    log::debug!("Total library folders found: {}", folders.len());
    folders
}

// Enhanced Drive Detection using comprehensive cross-platform detection
pub fn get_drives() -> Vec<DriveInfo> {
    log::debug!("get_drives called - using enhanced cross-platform detection");

    // Use the new comprehensive drive detection system
    let detected_drives = crate::core::drive_detector::DriveDetector::get_available_drives();

    log::debug!("Detected {} drives using enhanced detection", detected_drives.len());

    // Convert to our DriveInfo format and add debug logging
    let mut drives: Vec<DriveInfo> = Vec::new();

    for drive in detected_drives {
        log::debug!("Processing drive: {}", drive.label);
        log::debug!("  - path: {:?}", drive.path);
        log::debug!("  - type: {:?}", drive.drive_type);
        log::debug!("  - accessible: {}", drive.is_accessible);

        if let Some(total) = drive.total_space {
            log::debug!("  - total_space: {} bytes ({:.2} GB)", total, total as f64 / 1_000_000_000.0);
        } else {
            log::debug!("  - total_space: unknown");
        }

        if let Some(free) = drive.free_space {
            log::debug!("  - free_space: {} bytes ({:.2} GB)", free, free as f64 / 1_000_000_000.0);
        } else {
            log::debug!("  - free_space: unknown");
        }

        // Convert drive type to match our enum
        let drive_type = match drive.drive_type {
            crate::core::drive_detector::DriveType::FixedDisk => DriveType::FixedDisk,
            crate::core::drive_detector::DriveType::RemovableDisk => DriveType::RemovableDisk,
            crate::core::drive_detector::DriveType::NetworkDrive => DriveType::NetworkDrive,
            crate::core::drive_detector::DriveType::CDRom => DriveType::CDRom,
            crate::core::drive_detector::DriveType::RamDisk => DriveType::RamDisk,
            crate::core::drive_detector::DriveType::Unknown => DriveType::Unknown,
        };

        drives.push(DriveInfo {
            label: drive.label,
            path: drive.path,
            total_space: drive.total_space,
            free_space: drive.free_space,
            is_accessible: drive.is_accessible,
            drive_type,
        });
    }

    log::debug!("Returning {} drives from enhanced detection", drives.len());
    drives
}

// Old helper functions removed - now using comprehensive DriveDetector

// Recent Files
pub fn get_recent_files() -> Vec<RecentFile> {
    let mut files = Vec::new();

    log::debug!("get_recent_files called");

    // Use the proper recently_used_xbel API instead of manual XML parsing
    match recently_used_xbel::parse_file() {
        Ok(recent_files) => {
            log::debug!("Successfully parsed recent files, count: {}", recent_files.bookmarks.len());

            for bookmark in recent_files.bookmarks {
                log::debug!("Processing bookmark: {}", bookmark.href);

                if let Ok(url) = url::Url::parse(&bookmark.href) {
                    if let Ok(path) = url.to_file_path() {
                        // Check if file exists before adding it
                        if path.exists() {
                            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                                // Parse the timestamp from the bookmark
                                let modified = if let Ok(datetime) = bookmark.modified.parse::<chrono::DateTime<chrono::Utc>>() {
                                    datetime.into()
                                } else {
                                    log::debug!("  - Could not parse modified time: {}", bookmark.modified);
                                    SystemTime::UNIX_EPOCH
                                };

                                log::debug!("  - File name: {}", file_name);
                                log::debug!("  - Modified time: {:?}", modified);

                                files.push(RecentFile {
                                    name: file_name.to_string(),
                                    path,
                                    modified,
                                });
                            } else {
                                log::debug!("  - Could not get file name from path: {:?}", path);
                            }
                        } else {
                            log::debug!("  - File does not exist: {:?}", path);
                        }
                    } else {
                        log::debug!("  - Could not convert URL to file path: {}", bookmark.href);
                    }
                } else {
                    log::debug!("  - Could not parse URL: {}", bookmark.href);
                }
            }
        }
        Err(err) => {
            log::debug!("Error parsing recent files: {:?}", err);
        }
    }

    // Sort by modification time (most recent first)
    files.sort_by(|a, b| b.modified.cmp(&a.modified));

    // Limit to 10 most recent files
    let result: Vec<RecentFile> = files.into_iter().take(10).collect();
    log::debug!("Returning {} recent files", result.len());

    result
}

pub fn clear_recent_files() -> Result<(), Box<dyn std::error::Error>> {
    // Clear the recently-used.xbel file
    if let Some(recently_used_dir) = recently_used_xbel::dir() {
        let xbel_path = recently_used_dir.join("recently-used.xbel");
        std::fs::remove_file(xbel_path)?;
    }
    Ok(())
}