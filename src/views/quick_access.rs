// Quick Access view for Vortex File Manager

use cosmic::{
    Element,
    iced::{Alignment, Color, Length},
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
    tab,
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
    // Use statvfs on mount point if available
    if let Some(path) = item.path() {
        if let Ok(stat) = std::fs::metadata(path) {
            if stat.is_dir() {
                // TODO: Implement actual space calculation using statvfs
                return 100_000_000_000; // 100GB placeholder
            }
        }
    }
    0
}

fn get_space_used(item: &MounterItem) -> u64 {
    // TODO: Implement actual used space calculation
    if let Some(path) = item.path() {
        if let Ok(stat) = std::fs::metadata(path) {
            if stat.is_dir() {
                return 50_000_000_000; // 50GB placeholder
            }
        }
    }
    0
}

// Recent Files
pub fn get_recent_files() -> Vec<RecentFile> {
    // TODO: Read from recently-used.xbel (XDG recent files)
    // For now, return empty list
    Vec::new()
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
            let tile = widget::button::custom(
                widget::column()
                    .push(widget::icon::from_name(&*folder.icon).size(48))
                    .push(widget::text(folder.name.clone()))
                    .align_x(Alignment::Center)
                    .spacing(8)
            )
            .on_press(Message::OpenItemLocation(None)) // TODO: Fix this to open the folder
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(100.0));
            
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
        for drive in drives {
            let tile = drive_tile(&drive);
            column = column.push(tile);
        }
    }
    
    column.into()
}

fn drive_tile(drive: &DriveInfo) -> Element<'static, Message> {
    let icon = widget::icon::from_name(drive_icon_name(&drive.drive_type)).size(32);
    
    let label = widget::text(drive.label.clone())
        .size(14)
        .width(Length::Fill);
    
    let usage_pct = drive.usage_percent();
    let progress_bar = widget::progress_bar(0.0..=100.0, usage_pct);
    
    let space_text = widget::text(format!(
        "{} / {}",
        format_size(drive.used_space),
        format_size(drive.total_space)
    )).size(12);
    
    let content = widget::row()
        .push(icon)
        .push(widget::column()
            .push(label)
            .push(progress_bar)
            .push(space_text)
            .spacing(4)
        )
        .spacing(12)
        .align_y(Alignment::Center);
    
    let mut button = widget::button::custom(content)
        .padding(12);
    
    if drive.is_mounted {
        if let Some(_path) = &drive.path {
            button = button.on_press(Message::OpenItemLocation(None)); // TODO: Fix this to open the drive
        }
    } else {
        if let (Some(key), Some(item)) = (&drive.mounter_key, &drive.mounter_item) {
            button = button.on_press(Message::MountDrive(key.clone(), item.clone()));
        }
    }
    
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
        // Three-column list: Name, Date, Path
        for file in files.iter().take(10) {
            let row = widget::button::custom(
                widget::row()
                    .push(widget::text(file.name.clone()).width(Length::FillPortion(3)))
                    .push(widget::text(format_date(&file.modified)).width(Length::FillPortion(2)))
                    .push(widget::text(file.path.display().to_string()).width(Length::FillPortion(5)))
                    .spacing(12)
            )
            .on_press(Message::OpenItemLocation(None)); // TODO: Fix this to open the file
            
            column = column.push(row);
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

fn format_date(time: &SystemTime) -> String {
    // TODO: Implement proper date formatting
    "Today".to_string()
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
