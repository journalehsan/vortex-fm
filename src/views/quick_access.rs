// Quick Access view for Vortex File Manager

use cosmic::{
    Element,
    iced::{Alignment, Background, Color, Length},
    widget,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    time::SystemTime,
};

use crate::{
    app::Message,
    core::services::mount::{MounterItem, MounterItems, MounterKey},
    fl,
};

// Section expansion state (saved in config)
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct QuickAccessState {
    pub library_expanded: bool,
    pub drives_expanded: bool,
    pub recent_expanded: bool,
}

impl Default for QuickAccessState {
    fn default() -> Self {
        Self {
            library_expanded: true,
            drives_expanded: true,
            recent_expanded: true,
        }
    }
}

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
    
    pub fn progress_color(&self) -> Color {
        let pct = self.usage_percent();
        if pct >= 95.0 { Color::from_rgb(0.8, 0.0, 0.0) }      // red
        else if pct >= 80.0 { Color::from_rgb(1.0, 0.5, 0.0) } // orange
        else if pct >= 70.0 { Color::from_rgb(1.0, 0.8, 0.0) } // #ffcc00
        else if pct >= 50.0 { Color::from_rgb(0.0, 0.5, 1.0) } // blue
        else { Color::from_rgb(0.0, 0.8, 0.0) }                 // green
    }
}

// XDG User Directories Reading
pub fn get_library_folders() -> Vec<LibraryFolder> {
    let mut folders = Vec::new();
    
    // Desktop
    if let Some(path) = dirs::desktop_dir() {
        folders.push(LibraryFolder {
            name: fl!("desktop"),
            path,
            icon: "user-desktop".to_string(),
        });
    }
    
    // Downloads
    if let Some(path) = dirs::download_dir() {
        folders.push(LibraryFolder {
            name: fl!("downloads"),
            path,
            icon: "folder-download".to_string(),
        });
    }
    
    // Documents
    if let Some(path) = dirs::document_dir() {
        folders.push(LibraryFolder {
            name: fl!("documents"),
            path,
            icon: "folder-documents".to_string(),
        });
    }
    
    // Music
    if let Some(path) = dirs::audio_dir() {
        folders.push(LibraryFolder {
            name: fl!("music"),
            path,
            icon: "folder-music".to_string(),
        });
    }
    
    // Pictures
    if let Some(path) = dirs::picture_dir() {
        folders.push(LibraryFolder {
            name: fl!("pictures"),
            path,
            icon: "folder-pictures".to_string(),
        });
    }
    
    // Videos
    if let Some(path) = dirs::video_dir() {
        folders.push(LibraryFolder {
            name: fl!("videos"),
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
                total_space: get_space_total(&item),
                used_space: get_space_used(&item),
                is_mounted: item.is_mounted(),
                drive_type: detect_drive_type(&item),
                mounter_key: Some(key.clone()),
                mounter_item: Some(item.clone()),
            };
            drives.push(drive);
        }
    }

    // Add demo drives if no real drives are detected
    if drives.is_empty() {
        drives.push(DriveInfo {
            label: "System Disk".to_string(),
            path: Some(std::path::PathBuf::from("/")),
            total_space: 100_000_000_000, // 100GB
            used_space: 65_000_000_000,   // 65GB used
            is_mounted: true,
            drive_type: DriveType::System,
            mounter_key: None,
            mounter_item: None,
        });

        drives.push(DriveInfo {
            label: "Data Disk".to_string(),
            path: Some(std::path::PathBuf::from("/home")),
            total_space: 500_000_000_000, // 500GB
            used_space: 150_000_000_000,  // 150GB used
            is_mounted: true,
            drive_type: DriveType::Partition,
            mounter_key: None,
            mounter_item: None,
        });

        drives.push(DriveInfo {
            label: "USB Drive".to_string(),
            path: Some(std::path::PathBuf::from("/media/usb")),
            total_space: 32_000_000_000,  // 32GB
            used_space: 12_000_000_000,   // 12GB used
            is_mounted: true,
            drive_type: DriveType::Usb,
            mounter_key: None,
            mounter_item: None,
        });
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
    // TODO: Read from recently-used.xbel (XDG recent files)
    // For now, return demo files with better variety
    let mut files = Vec::new();

    // Add some demo recent files with different file types and realistic dates
    if let Some(home) = dirs::home_dir() {
        let demo_files = vec![
            ("Project_Proposal.pdf", "Documents", 1),  // 1 day ago
            ("Family_Photo.jpg", "Pictures", 2),       // 2 days ago
            ("Vacation_Video.mp4", "Videos", 3),       // 3 days ago
            ("Favorite_Song.mp3", "Music", 4),         // 4 days ago
            ("Archive_2024.zip", "Downloads", 5),      // 5 days ago
            ("main.rs", "Documents/Projects", 1),       // 1 day ago
            ("Screenshot_001.png", "Pictures", 2),     // 2 days ago
            ("Budget_2024.xlsx", "Documents", 3),      // 3 days ago
            ("Meeting_Notes.txt", "Documents", 1),     // 1 day ago
            ("Presentation.pptx", "Documents", 2),     // 2 days ago
        ];

        for (filename, folder, days_ago) in demo_files {
            let path = if folder.contains('/') {
                let parts: Vec<&str> = folder.split('/').collect();
                home.join(parts[0]).join(parts[1]).join(filename)
            } else {
                home.join(folder).join(filename)
            };

            // Create a modified time that's a few days ago
            let modified = std::time::SystemTime::now() - std::time::Duration::from_secs(days_ago * 24 * 60 * 60);
            files.push(RecentFile {
                name: filename.to_string(),
                path,
                modified,
            });
        }
    }

    files
}

pub fn clear_recent_files() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Clear the recently-used.xbel file
    Ok(())
}

// UI Components
fn section_header(
    title: String,
    _is_expanded: bool,
    toggle_message: Message,
) -> Element<'static, Message> {
    widget::button::text(title)
        .on_press(toggle_message)
        .into()
}

fn library_section(
    folders: Vec<LibraryFolder>,
    expanded: bool,
) -> Element<'static, Message> {
    let mut column = widget::column().spacing(4);
    
    column = column.push(section_header(
        fl!("library").to_string(),
        expanded,
        Message::ToggleQuickAccessSection(Section::Library),
    ));
    
    if expanded {
        let mut grid = widget::grid()
            .padding(16.into())
            .row_spacing(12);
            
        for folder in folders {
            let tile = widget::container(
                widget::mouse_area(
                    widget::column()
                        .push(widget::icon::from_name(&*folder.icon).size(48))
                        .push(widget::text(folder.name.clone()))
                        .align_x(Alignment::Center)
                        .spacing(8)
                )
                .on_press(Message::OpenItemLocation(None)) // TODO: Fix this to open the folder
            )
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(100.0))
            .padding(16)
            .style(|theme: &cosmic::Theme| {
                cosmic::widget::container::Style {
                    icon_color: Some(Color::TRANSPARENT),
                    text_color: Some(Color::TRANSPARENT),
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    border: cosmic::iced::Border::default(),
                    shadow: cosmic::iced::Shadow::default(),
                }
            });

            grid = grid.push(tile);
        }
        
        column = column.push(grid);
    }
    
    column.into()
}

fn drives_section(
    drives: Vec<DriveInfo>,
    expanded: bool,
) -> Element<'static, Message> {
    let mut column = widget::column().spacing(4);
    
    column = column.push(section_header(
        fl!("drives").to_string(),
        expanded,
        Message::ToggleQuickAccessSection(Section::Drives),
    ));
    
    if expanded {
        if !drives.is_empty() {
            let mut grid = widget::grid()
                .padding(16.into())
                .row_spacing(16)
                .column_spacing(16);
                
            for drive in drives {
                let tile = drive_tile(&drive);
                grid = grid.push(tile);
            }
            
            column = column.push(grid);
        } else {
            // Show placeholder when no drives are available
            column = column.push(
                widget::container(
                    widget::text("No drives available")
                        .size(14)
                )
                .padding(16)
            );
        }
    }
    
    column.into()
}

fn drive_tile(drive: &DriveInfo) -> Element<'static, Message> {
    let icon = widget::icon::from_name(drive_icon_name(&drive.drive_type)).size(48);
    
    let label = widget::text(drive.label.clone())
        .size(16)
        .width(Length::Fill);
    
    let usage_pct = drive.usage_percent();
    let progress_bar = widget::progress_bar(0.0..=100.0, usage_pct)
        .height(6.0);
    
    let space_text = widget::text(format!(
        "{} / {}",
        format_size(drive.used_space),
        format_size(drive.total_space)
    )).size(12)
    .width(Length::Fill);
    
    let content = widget::column()
        .push(icon)
        .push(widget::vertical_space())
        .push(label)
        .push(widget::vertical_space())
        .push(progress_bar)
        .push(widget::vertical_space())
        .push(space_text)
        .align_x(Alignment::Center)
        .spacing(0)
        .padding(16);

    let mut button_content = widget::mouse_area(content);

    if drive.is_mounted {
        if let Some(_path) = &drive.path {
            button_content = button_content.on_press(Message::OpenItemLocation(None)); // TODO: Fix this to open the drive
        }
    } else {
        if let (Some(key), Some(item)) = (&drive.mounter_key, &drive.mounter_item) {
            button_content = button_content.on_press(Message::MountDrive(key.clone(), item.clone()));
        }
    }

    let mut button = widget::container(button_content)
        .width(Length::Fixed(160.0))
        .height(Length::Fixed(140.0))
        .style(|theme: &cosmic::Theme| {
            cosmic::widget::container::Style {
                icon_color: Some(Color::TRANSPARENT),
                text_color: Some(Color::TRANSPARENT),
                background: Some(Background::Color(Color::TRANSPARENT)),
                border: cosmic::iced::Border::default(),
                shadow: cosmic::iced::Shadow::default(),
            }
        });

    button.into()
}

fn drive_icon_name(drive_type: &DriveType) -> &'static str {
    match drive_type {
        DriveType::System => "drive-harddisk",
        DriveType::Partition => "drive-harddisk",
        DriveType::External => "drive-removable-media",
        DriveType::Optical => "media-optical",
        DriveType::Usb => "drive-removable-media-usb",
        DriveType::Network => "network-workgroup",
    }
}

fn recent_section(
    files: Vec<RecentFile>,
    expanded: bool,
) -> Element<'static, Message> {
    let mut column = widget::column().spacing(4);
    
    let header = widget::row()
        .push(section_header(
            fl!("recent-files").to_string(),
            expanded,
            Message::ToggleQuickAccessSection(Section::Recent),
        ))
        .push(widget::horizontal_space())
        .push(widget::button::icon(widget::icon::from_name("edit-clear-symbolic"))
            .on_press(Message::ClearRecentFiles)
            .tooltip(fl!("clear-recent"))
        );
    
    column = column.push(header);
    
    if expanded {
        if !files.is_empty() {
            // Modern list of recent files
            for file in files.iter().take(8) { // Limit to 8 items
                let file_icon = widget::icon::from_name(get_file_icon(&file.name)).size(20);

                let row = widget::button::custom(
                    widget::row()
                        .push(file_icon)
                        .push(widget::horizontal_space())
                        .push(widget::column()
                            .push(widget::text(file.name.clone()).size(14))
                            .push(widget::text(format_date(&file.modified)).size(11))
                            .spacing(2)
                        )
                        .push(widget::horizontal_space())
                        .align_y(Alignment::Center)
                        .padding(12)
                )
                .on_press(Message::OpenItemLocation(None)) // TODO: Fix this to open the file
                .width(Length::Fill)
                .height(Length::Fixed(56.0));

                column = column.push(row);
            }
        } else {
            // Show placeholder when no recent files
            column = column.push(
                widget::container(
                    widget::text("No recent files")
                        .size(14)
                )
                .padding(16)
            );
        }
    }
    
    column.into()
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

fn get_file_icon(filename: &str) -> &'static str {
    if let Some(extension) = std::path::Path::new(filename).extension() {
        match extension.to_str().unwrap_or("").to_lowercase().as_str() {
            "pdf" => "application-pdf",
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" => "image-x-generic",
            "mp4" | "avi" | "mkv" | "mov" | "wmv" => "video-x-generic",
            "mp3" | "wav" | "flac" | "ogg" => "audio-x-generic",
            "zip" | "rar" | "7z" | "tar" | "gz" => "package-x-generic",
            "rs" | "py" | "js" | "html" | "css" => "text-x-script",
            "xlsx" | "xls" => "x-office-spreadsheet",
            "docx" | "doc" => "x-office-document",
            "pptx" | "ppt" => "x-office-presentation",
            "txt" => "text-x-generic",
            _ => "text-x-generic",
        }
    } else {
        "text-x-generic"
    }
}

fn format_date(time: &SystemTime) -> String {

    if let Ok(duration) = time.elapsed() {
        let days = duration.as_secs() / (24 * 60 * 60);
        let hours = (duration.as_secs() % (24 * 60 * 60)) / (60 * 60);
        let minutes = (duration.as_secs() % (60 * 60)) / 60;

        if days == 0 {
            if hours == 0 {
                if minutes == 0 {
                    "Just now".to_string()
                } else {
                    format!("{} minute{} ago", minutes, if minutes == 1 { "" } else { "s" })
                }
            } else {
                format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
            }
        } else if days == 1 {
            "Yesterday".to_string()
        } else if days < 7 {
            format!("{} days ago", days)
        } else if days < 30 {
            format!("{} weeks ago", days / 7)
        } else {
            "Older".to_string()
        }
    } else {
        "Unknown".to_string()
    }
}

// Main QuickAccess View Function
pub fn quick_access_view(
    state: &QuickAccessState,
    mounter_items: &HashMap<MounterKey, MounterItems>,
) -> Element<'static, Message> {
    let library_folders = get_library_folders();
    let drives = get_drives(mounter_items);
    let recent_files = get_recent_files();
    
    widget::scrollable(
        widget::column()
            .push(library_section(library_folders, state.library_expanded))
            .push(widget::horizontal_space())
            .push(drives_section(drives, state.drives_expanded))
            .push(widget::horizontal_space())
            .push(recent_section(recent_files, state.recent_expanded))
            .padding(16)
            .spacing(16)
    )
    .into()
}

// Section enum for messages
#[derive(Clone, Debug)]
pub enum Section {
    Library,
    Drives,
    Recent,
}
