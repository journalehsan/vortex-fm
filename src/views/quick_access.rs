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
        if !folders.is_empty() {
            // Create responsive grid layout for library folders
            let folders_per_row = 3; // Start with 3 per row, will adapt
            let mut folders_column = widget::column().spacing(12).padding(16);

            for chunk in folders.chunks(folders_per_row) {
                let mut row = widget::row().spacing(12).width(Length::Fill);

                for folder in chunk {
                    let tile = widget::container(
                        widget::mouse_area(
                            widget::column()
                                .push(widget::icon::from_name(&*folder.icon).size(48))
                                .push(widget::text(folder.name.clone()))
                                .align_x(Alignment::Center)
                                .spacing(8)
                        )
                        .on_press(Message::TabMessage(None, tab::Message::Open(Some(folder.path.clone()))))
                    )
                    .width(Length::Fill)
                    .height(Length::Fixed(100.0))
                    .padding(16)
                    .style(|theme: &cosmic::Theme| {
                        cosmic::widget::container::Style {
                            icon_color: Some(Color::TRANSPARENT),
                            text_color: Some(theme.cosmic().accent_text_color().into()),
                            background: Some(Background::Color(Color::TRANSPARENT)),
                            border: cosmic::iced::Border::default(),
                            shadow: cosmic::iced::Shadow::default(),
                        }
                    });

                    row = row.push(tile);
                }

                // Fill remaining slots with spacers if needed
                let spacers_needed = folders_per_row - chunk.len();
                for _ in 0..spacers_needed {
                    row = row.push(widget::horizontal_space().width(Length::Fill).height(Length::Fixed(100.0)));
                }

                folders_column = folders_column.push(row);
            }

            // Wrap in scrollable for cases where content is too wide
            let folders_scrollable = widget::scrollable(
                widget::container(folders_column)
                    .width(Length::Fill)
            );

            column = column.push(folders_scrollable);
        } else {
            // Show placeholder when no library folders
            column = column.push(
                widget::container(
                    widget::text("No library folders available")
                        .size(14)
                )
                .padding(16)
            );
        }
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
            button_content = button_content.on_press(Message::TabMessage(None, tab::Message::Open(Some(drive.path.clone()))));
        }
    } else {
        // For inaccessible drives, we could add mount functionality here
        // For now, just show them as disabled
    }

    let button = widget::container(button_content)
        .width(Length::Fixed(160.0))
        .height(Length::Fixed(140.0))
        .style(|theme: &cosmic::Theme| {
            cosmic::widget::container::Style {
                icon_color: Some(Color::TRANSPARENT),
                text_color: Some(theme.cosmic().accent_text_color().into()),
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

// File type badge component
fn file_type_badge(file_name: &str) -> Element<'static, Message> {
    let file_type = get_file_type_label(file_name);

    widget::container(
        widget::text(file_type)
            .size(10)
    )
    .padding(4)
    .style(|theme: &cosmic::Theme| {
        cosmic::widget::container::Style {
            icon_color: Some(Color::TRANSPARENT),
            text_color: Some(Color::WHITE.into()),
            background: Some(Background::Color(
                theme.cosmic().accent_color().into()
            )),
            border: cosmic::iced::Border {
                radius: 12.0.into(),
                ..Default::default()
            },
            shadow: cosmic::iced::Shadow::default(),
        }
    })
    .into()
}

fn get_file_type_label(filename: &str) -> &'static str {
    if let Some(extension) = std::path::Path::new(filename).extension() {
        match extension.to_str().unwrap_or("").to_lowercase().as_str() {
            "pdf" => "PDF",
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" => "Image",
            "mp4" | "avi" | "mkv" | "mov" | "wmv" => "Video",
            "mp3" | "wav" | "flac" | "ogg" => "Audio",
            "zip" | "rar" | "7z" | "tar" | "gz" => "Archive",
            "rs" => "Rust",
            "py" => "Python",
            "js" => "JavaScript",
            "html" => "HTML",
            "css" => "CSS",
            "xlsx" | "xls" => "Excel",
            "docx" | "doc" => "Document",
            "pptx" | "ppt" => "Presentation",
            "txt" => "Text",
            _ => "File",
        }
    } else {
        "File"
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
            // Modern card-based layout for recent files with full width
            let mut files_column = widget::column().spacing(8).padding(16);
            
            for file in files.iter().take(8) { // Limit to 8 items
                let card = recent_file_card(file);
                files_column = files_column.push(card);
            }
            
            column = column.push(files_column);
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

fn recent_file_card(file: &RecentFile) -> Element<'static, Message> {
    // Left section: File icon and name
    let file_icon = widget::icon::from_name(get_file_icon(&file.name)).size(24);
    let file_name = widget::text(file.name.clone())
        .size(13)
        .width(Length::Fill);

    let left_section = widget::container(
        widget::row()
            .push(file_icon)
            .push(widget::horizontal_space().width(Length::Fixed(12.0)))
            .push(file_name)
            .align_y(Alignment::Center)
            .spacing(0)
    )
    .width(Length::Fill)
    .height(Length::Fixed(60.0))
    .align_x(Alignment::Start)
    .align_y(Alignment::Center)
    .padding(12);

    // Center section: Date
    let center_section = widget::container(
        widget::text(format_date(&file.modified))
            .size(12)
    )
    .width(Length::Fixed(100.0))
    .height(Length::Fixed(60.0))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center);

    // Right section: File type badge
    let right_section = widget::container(
        file_type_badge(&file.name)
    )
    .width(Length::Fixed(80.0))
    .height(Length::Fixed(60.0))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center);

    // Combine all three sections with proper spacing
    let content = widget::row()
        .push(left_section)
        .push(center_section)
        .push(right_section)
        .width(Length::Fill)
        .spacing(0);

    // Create the card container with hover effect
    widget::button::custom(
        widget::container(content)
            .style(|theme: &cosmic::Theme| {
                cosmic::widget::container::Style {
                    icon_color: Some(Color::TRANSPARENT),
                    text_color: Some(theme.cosmic().accent_text_color().into()),
                    background: Some(Background::Color(
                        theme.cosmic().background.base.into()
                    )),
                    border: cosmic::iced::Border {
                        radius: 8.0.into(),
                        width: 1.0,
                        color: theme.cosmic().background.component.border.into(),
                    },
                    shadow: cosmic::iced::Shadow::default(),
                }
            })
            .padding(0)
    )
    .on_press(Message::TabMessage(None, tab::Message::Open(Some(file.path.clone()))))
    .width(Length::Fill)
    .height(Length::Fixed(60.0))
    .into()
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
