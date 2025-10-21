// Quick Access view for Vortex File Manager

use cosmic::{
    Element,
    iced::{Alignment, Background, Color, Length},
    widget,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::{
    app::Message,
    core::quick_access::{self, DriveInfo, DriveType, LibraryFolder, RecentFile},
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
                .on_press(Message::OpenItemLocation(None))
            )
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(100.0))
            .padding(16)
            .style(|_theme: &cosmic::Theme| {
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
            // Create rows of drives (3 per row) that wrap properly
            let drives_per_row = 3;
            let mut drives_column = widget::column().spacing(16).padding(16);

            for chunk in drives.chunks(drives_per_row) {
                let mut row = widget::row().spacing(16).width(Length::Fill);

                for drive in chunk {
                    row = row.push(drive_tile(drive));
                }

                // Fill remaining slots with spacers if needed (calculate how many we need)
                let spacers_needed = drives_per_row - chunk.len();
                for _ in 0..spacers_needed {
                    row = row.push(widget::horizontal_space().width(Length::Fixed(160.0)).height(Length::Fixed(140.0)));
                }

                drives_column = drives_column.push(row);
            }

            // Wrap in scrollable for cases where content is still too wide
            let drives_scrollable = widget::scrollable(
                widget::container(drives_column)
                    .width(Length::Shrink)
                    .height(Length::Fixed(156.0)) // Account for tile height (140) + padding (16)
            );

            column = column.push(drives_scrollable);
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

    let space_text = widget::text(drive.format_size()).size(12)
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

    if drive.is_accessible {
        if let Some(_path) = drive.path.to_str() {
            button_content = button_content.on_press(Message::OpenItemLocation(None));
        }
    } else {
        // For inaccessible drives, we could add mount functionality here
        // For now, just show them as disabled
    }

    let button = widget::container(button_content)
        .width(Length::Fixed(160.0))
        .height(Length::Fixed(140.0))
        .style(|_theme: &cosmic::Theme| {
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
        crate::core::quick_access::DriveType::FixedDisk => "drive-harddisk",
        crate::core::quick_access::DriveType::RemovableDisk => "drive-removable-media",
        crate::core::quick_access::DriveType::NetworkDrive => "network-workgroup",
        crate::core::quick_access::DriveType::CDRom => "media-optical",
        crate::core::quick_access::DriveType::RamDisk => "media-flash",
        crate::core::quick_access::DriveType::Unknown => "drive-harddisk",
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
                .on_press(Message::OpenItemLocation(None))
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

// format_size function removed - now using DriveInfo.format_size() method

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
) -> Element<'static, Message> {
    let library_folders = quick_access::get_library_folders();
    let drives = quick_access::get_drives();
    let recent_files = quick_access::get_recent_files();

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
