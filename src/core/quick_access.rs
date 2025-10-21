// Quick Access core functionality for Vortex File Manager

use std::{
    collections::HashMap,
    path::PathBuf,
    time::SystemTime,
};

use crate::core::services::mount::{MounterItem, MounterItems, MounterKey};

// Library folder entry
#[derive(Clone, Debug)]
pub struct LibraryFolder {
    pub name: String,
    pub path: PathBuf,
    pub icon: String,  // system icon name
}

// Drive entry with mount info
#[derive(Clone, Debug)]
pub struct DriveInfo {
    pub label: String,
    pub path: Option<PathBuf>,
    pub total_space: u64,
    pub used_space: u64,
    pub is_mounted: bool,
    pub drive_type: DriveType,
    pub mounter_key: Option<MounterKey>,
    pub mounter_item: Option<MounterItem>,
}

#[derive(Clone, Debug)]
pub enum DriveType {
    System,
    Partition,
    External,
    Optical,
    Usb,
    Network,
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
        if self.total_space == 0 { 0.0 }
        else { (self.used_space as f32 / self.total_space as f32) * 100.0 }
    }

    pub fn progress_color(&self) -> cosmic::iced::Color {
        let pct = self.usage_percent();
        if pct >= 95.0 { cosmic::iced::Color::from_rgb(0.8, 0.0, 0.0) }      // red
        else if pct >= 80.0 { cosmic::iced::Color::from_rgb(1.0, 0.5, 0.0) } // orange
        else if pct >= 70.0 { cosmic::iced::Color::from_rgb(1.0, 0.8, 0.0) } // #ffcc00
        else if pct >= 50.0 { cosmic::iced::Color::from_rgb(0.0, 0.5, 1.0) } // blue
        else { cosmic::iced::Color::from_rgb(0.0, 0.8, 0.0) }                 // green
    }
}

// XDG User Directories Reading
pub fn get_library_folders() -> Vec<LibraryFolder> {
    let mut folders = Vec::new();

    // Desktop
    if let Some(path) = dirs::desktop_dir() {
        folders.push(LibraryFolder {
            name: "Desktop".to_string(), // Will be localized in view
            path,
            icon: "user-desktop".to_string(),
        });
    }

    // Downloads
    if let Some(path) = dirs::download_dir() {
        folders.push(LibraryFolder {
            name: "Downloads".to_string(),
            path,
            icon: "folder-download".to_string(),
        });
    }

    // Documents
    if let Some(path) = dirs::document_dir() {
        folders.push(LibraryFolder {
            name: "Documents".to_string(),
            path,
            icon: "folder-documents".to_string(),
        });
    }

    // Music
    if let Some(path) = dirs::audio_dir() {
        folders.push(LibraryFolder {
            name: "Music".to_string(),
            path,
            icon: "folder-music".to_string(),
        });
    }

    // Pictures
    if let Some(path) = dirs::picture_dir() {
        folders.push(LibraryFolder {
            name: "Pictures".to_string(),
            path,
            icon: "folder-pictures".to_string(),
        });
    }

    // Videos
    if let Some(path) = dirs::video_dir() {
        folders.push(LibraryFolder {
            name: "Videos".to_string(),
            path,
            icon: "folder-videos".to_string(),
        });
    }

    folders
}

// Drive Detection
pub fn get_drives(mounter_items: &HashMap<MounterKey, MounterItems>) -> Vec<DriveInfo> {
    let mut drives = Vec::new();

    // Use existing MOUNTERS system
    for (key, items) in mounter_items {
        for item in items {
            let drive = DriveInfo {
                label: item.name(),
                path: item.path(),
                total_space: get_space_total(item),
                used_space: get_space_used(item),
                is_mounted: item.is_mounted(),
                drive_type: detect_drive_type(item),
                mounter_key: Some(*key),
                mounter_item: Some(item.clone()),
            };
            drives.push(drive);
        }
    }

    drives
}

fn detect_drive_type(item: &MounterItem) -> DriveType {
    // Check mount point, device name patterns
    if let Some(path) = item.path() {
        let path_str = path.to_string_lossy();
        if path_str.contains("/dev/sr") {
            DriveType::Optical
        } else if path_str.contains("/media/") || path_str.contains("/run/media/") {
            DriveType::Usb
        } else if path_str.contains("/dev/sd") {
            DriveType::System
        } else if path_str.starts_with("network://") {
            DriveType::Network
        } else {
            DriveType::External
        }
    } else {
        DriveType::External
    }
}

fn get_space_total(item: &MounterItem) -> u64 {
    if let Some(path) = item.path() {
        // Use libc statvfs to get filesystem statistics
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;

        if let Ok(c_path) = CString::new(path.as_os_str().as_bytes()) {
            unsafe {
                let mut stat: libc::statvfs = std::mem::zeroed();
                if libc::statvfs(c_path.as_ptr(), &mut stat) == 0 {
                    return stat.f_blocks as u64 * stat.f_frsize as u64;
                }
            }
        }
    }
    0
}

fn get_space_used(item: &MounterItem) -> u64 {
    if let Some(path) = item.path() {
        // Use libc statvfs to get filesystem statistics
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;

        if let Ok(c_path) = CString::new(path.as_os_str().as_bytes()) {
            unsafe {
                let mut stat: libc::statvfs = std::mem::zeroed();
                if libc::statvfs(c_path.as_ptr(), &mut stat) == 0 {
                    return (stat.f_blocks - stat.f_bavail) as u64 * stat.f_frsize as u64;
                }
            }
        }
    }
    0
}

// Recent Files
pub fn get_recent_files() -> Vec<RecentFile> {
    let mut files = Vec::new();

    // Read from recently-used.xbel (XDG recent files)
    if let Some(recently_used_dir) = recently_used_xbel::dir() {
        let xbel_path = recently_used_dir.join("recently-used.xbel");

        if let Ok(content) = std::fs::read_to_string(&xbel_path) {
            // Simple XML parsing for XBEL format
            // Look for <bookmark href="file://..." ...>
            for line in content.lines() {
                if let Some(href_start) = line.find("href=\"file://") {
                    let href_part = &line[href_start + 6..]; // Skip href="
                    if let Some(quote_end) = href_part.find('"') {
                        let uri = &href_part[..quote_end];
                        if let Ok(url) = url::Url::parse(uri) {
                            if let Ok(path) = url.to_file_path() {
                                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                                    // Get modification time from file metadata
                                    let modified = std::fs::metadata(&path)
                                        .ok()
                                        .and_then(|m| m.modified().ok())
                                        .unwrap_or(SystemTime::UNIX_EPOCH);

                                    files.push(RecentFile {
                                        name: file_name.to_string(),
                                        path,
                                        modified,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort by modification time (most recent first)
    files.sort_by(|a, b| b.modified.cmp(&a.modified));

    // Limit to 10 most recent files
    files.into_iter().take(10).collect()
}

pub fn clear_recent_files() -> Result<(), Box<dyn std::error::Error>> {
    // Clear the recently-used.xbel file
    if let Some(recently_used_dir) = recently_used_xbel::dir() {
        let xbel_path = recently_used_dir.join("recently-used.xbel");
        std::fs::remove_file(xbel_path)?;
    }
    Ok(())
}