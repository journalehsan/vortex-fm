use gtk::prelude::*;
use gtk::{
    Box, Orientation, Label, Button, 
    ProgressBar, Revealer, Align
};
use std::path::PathBuf;
use std::fs;
use crate::utils::simple_debug::debug_info;
use crate::core::navigation::navigate_to_directory;
use crate::views::content_area::switch_to_browser_view;

#[derive(Clone)]
pub struct HomeScreen {
    pub container: Box,
    home_section: Box,
    storage_section: Box,
    recent_section: Box,
}

impl HomeScreen {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 12);
        container.set_margin_start(16);
        container.set_margin_end(16);
        container.set_margin_top(16);
        container.set_margin_bottom(16);

        // Create the three accordion sections
        let home_section = Self::create_home_section();
        let storage_section = Self::create_storage_section();
        let recent_section = Self::create_recent_section();

        container.append(&home_section);
        container.append(&storage_section);
        container.append(&recent_section);

        Self {
            container,
            home_section,
            storage_section,
            recent_section,
        }
    }

    fn create_home_section() -> Box {
        let section = Box::new(Orientation::Vertical, 8);
        
        // Section header with revealer
        let (header_container, revealer) = Self::create_section_header("🏠 Home Directories", true);
        
        // Section content
        let content = Self::create_home_directories();
        revealer.set_child(Some(&content));
        
        section.append(&header_container);
        section
    }

    fn create_storage_section() -> Box {
        let section = Box::new(Orientation::Vertical, 8);
        
        // Section header with revealer
        let (header_container, revealer) = Self::create_section_header("💾 Storage Drives", true);
        
        // Section content
        let content = Self::create_storage_drives();
        revealer.set_child(Some(&content));
        
        section.append(&header_container);
        section
    }

    fn create_recent_section() -> Box {
        let section = Box::new(Orientation::Vertical, 8);
        
        // Section header with revealer
        let (header_container, revealer) = Self::create_section_header("📁 Recent Files", true);
        
        // Section content
        let content = Self::create_recent_files_placeholder();
        revealer.set_child(Some(&content));
        
        section.append(&header_container);
        section
    }

    fn create_section_header(title: &str, expanded: bool) -> (Box, Revealer) {
        let section_container = Box::new(Orientation::Vertical, 0);
        
        // Header with toggle button
        let header = Box::new(Orientation::Horizontal, 8);
        header.set_margin_start(8);
        header.set_margin_end(8);
        header.set_margin_top(8);
        header.set_margin_bottom(8);
        
        // Toggle button (arrow)
        let toggle_btn = Button::new();
        toggle_btn.set_css_classes(&["flat", "circular"]);
        toggle_btn.set_size_request(32, 32);
        
        let arrow_icon = if expanded { "▼" } else { "▶" };
        toggle_btn.set_label(arrow_icon);
        
        // Title label
        let title_label = Label::new(Some(title));
        title_label.set_css_classes(&["title-2"]);
        title_label.set_halign(Align::Start);
        title_label.set_hexpand(true);
        
        header.append(&toggle_btn);
        header.append(&title_label);
        
        // Create revealer for content
        let revealer = Revealer::new();
        revealer.set_reveal_child(expanded);
        
        // Connect toggle functionality
        let revealer_clone = revealer.clone();
        let toggle_btn_clone = toggle_btn.clone();
        toggle_btn.connect_clicked(move |_| {
            let is_revealed = revealer_clone.reveals_child();
            revealer_clone.set_reveal_child(!is_revealed);
            let new_arrow = if !is_revealed { "▼" } else { "▶" };
            toggle_btn_clone.set_label(new_arrow);
        });
        
        section_container.append(&header);
        section_container.append(&revealer);
        
        (section_container, revealer)
    }

    fn create_home_directories() -> Box {
        let content = Box::new(Orientation::Vertical, 4);
        
        // Standard Linux home directories
        let home_dirs = vec![
            ("Desktop", "🖥️", "Desktop"),
            ("Documents", "📄", "Documents"),
            ("Downloads", "⬇️", "Downloads"),
            ("Pictures", "🖼️", "Pictures"),
            ("Videos", "🎬", "Videos"),
            ("Music", "🎵", "Music"),
            ("Templates", "📋", "Templates"),
            ("Public", "🌐", "Public"),
        ];
        
        for (name, icon, xdg_name) in home_dirs {
            let dir_item = Self::create_directory_item(name, icon, xdg_name);
            content.append(&dir_item);
        }
        
        content
    }

    fn create_directory_item(name: &str, icon: &str, _xdg_name: &str) -> Box {
        let item = Box::new(Orientation::Horizontal, 12);
        item.set_margin_start(16);
        item.set_margin_end(8);
        item.set_margin_top(4);
        item.set_margin_bottom(4);
        item.set_css_classes(&["directory-item"]);
        
        // Icon
        let icon_label = Label::new(Some(icon));
        icon_label.set_css_classes(&["directory-icon"]);
        icon_label.set_size_request(32, 32);
        
        // Name
        let name_label = Label::new(Some(name));
        name_label.set_css_classes(&["directory-name"]);
        name_label.set_halign(Align::Start);
        name_label.set_hexpand(true);
        
        item.append(&icon_label);
        item.append(&name_label);
        
        // Make clickable
        let clickable_item = Button::new();
        clickable_item.set_css_classes(&["flat", "directory-button"]);
        clickable_item.set_hexpand(true);
        clickable_item.set_child(Some(&item));
        
        // Connect click to navigate
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        let dir_path = PathBuf::from(&home).join(name);
        
        // Create directory if it doesn't exist
        if !dir_path.exists() {
            if let Err(e) = fs::create_dir_all(&dir_path) {
                debug_info("HOME_SCREEN", &format!("Failed to create directory {}: {}", dir_path.display(), e));
            }
        }
        
        clickable_item.connect_clicked(move |_| {
            debug_info("HOME_SCREEN", &format!("Opening directory: {}", dir_path.display()));
            navigate_to_directory(dir_path.clone());
            switch_to_browser_view();
        });
        
        // Create container
        let container = Box::new(Orientation::Vertical, 0);
        container.append(&clickable_item);
        container
    }

    fn create_storage_drives() -> Box {
        let content = Box::new(Orientation::Vertical, 8);
        
        // Get mounted drives
        let drives = Self::get_mounted_drives();
        
        for drive in drives {
            let drive_item = Self::create_drive_item(&drive);
            content.append(&drive_item);
        }
        
        content
    }

    fn get_mounted_drives() -> Vec<DriveInfo> {
        let mut drives = Vec::new();
        
        // Read /proc/mounts to get real mounted filesystems
        if let Ok(content) = fs::read_to_string("/proc/mounts") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let mount_point = parts[1];
                    let fs_type = parts[2];
                    
                    // Skip system filesystems and network mounts
                    if Self::should_include_mount(mount_point, fs_type) {
                        let path = PathBuf::from(mount_point);
                        if let Ok(drive_info) = Self::get_drive_info(&path, mount_point) {
                            drives.push(drive_info);
                        }
                    }
                }
            }
        }
        
        // If no drives found, add some common ones
        if drives.is_empty() {
            drives.push(DriveInfo {
                name: "Root".to_string(),
                path: PathBuf::from("/"),
                total_space: 100_000_000_000,
                used_space: 60_000_000_000,
                usage_percent: 60.0,
            });
        }
        
        drives
    }
    
    fn should_include_mount(mount_point: &str, fs_type: &str) -> bool {
        // Include common filesystem types
        let valid_fs_types = ["ext4", "ext3", "ext2", "xfs", "btrfs", "ntfs", "vfat", "fat32"];
        
        // Skip system mounts
        let skip_mounts = [
            "/proc", "/sys", "/dev", "/dev/pts", "/dev/shm", "/run", "/tmp",
            "/var/run", "/var/lock", "/sys/fs/cgroup", "/sys/fs/pstore",
            "/sys/kernel/debug", "/sys/kernel/security", "/sys/fs/fuse/connections"
        ];
        
        valid_fs_types.contains(&fs_type) && 
        !skip_mounts.iter().any(|&skip| mount_point.starts_with(skip)) &&
        !mount_point.starts_with("/snap/") &&
        !mount_point.starts_with("/var/lib/docker/")
    }
    
    fn get_drive_info(path: &PathBuf, mount_point: &str) -> Result<DriveInfo, std::io::Error> {
        // Get filesystem statistics
        let stat = nix::sys::statvfs::statvfs(path)?;
        
        let block_size = stat.fragment_size() as u64;
        let total_blocks = stat.blocks() as u64;
        let free_blocks = stat.blocks_available() as u64;
        let used_blocks = total_blocks - free_blocks;
        
        let total_space = total_blocks * block_size;
        let used_space = used_blocks * block_size;
        let usage_percent = if total_space > 0 {
            (used_space as f64 / total_space as f64) * 100.0
        } else {
            0.0
        };
        
        // Generate a nice display name
        let name = Self::get_drive_display_name(mount_point);
        
        Ok(DriveInfo {
            name,
            path: path.clone(),
            total_space,
            used_space,
            usage_percent,
        })
    }
    
    fn get_drive_display_name(mount_point: &str) -> String {
        match mount_point {
            "/" => "Root".to_string(),
            "/home" => "Home".to_string(),
            "/boot" => "Boot".to_string(),
            "/var" => "Var".to_string(),
            "/tmp" => "Temp".to_string(),
            _ => {
                // Extract meaningful name from mount point
                if mount_point.starts_with("/mnt/") {
                    mount_point.trim_start_matches("/mnt/").to_string()
                } else if mount_point.starts_with("/media/") {
                    let parts: Vec<&str> = mount_point.split('/').collect();
                    if parts.len() >= 3 {
                        format!("{} ({})", parts[2], parts[1])
                    } else {
                        mount_point.to_string()
                    }
                } else if mount_point.starts_with("/run/media/") {
                    let parts: Vec<&str> = mount_point.split('/').collect();
                    if parts.len() >= 4 {
                        format!("{} ({})", parts[3], parts[2])
                    } else {
                        mount_point.to_string()
                    }
                } else {
                    mount_point.trim_start_matches('/').to_string()
                }
            }
        }
    }

    fn create_drive_item(drive: &DriveInfo) -> Box {
        let item = Box::new(Orientation::Vertical, 8);
        item.set_margin_start(16);
        item.set_margin_end(8);
        item.set_margin_top(4);
        item.set_margin_bottom(4);
        item.set_css_classes(&["drive-item"]);
        
        // Drive header
        let header = Box::new(Orientation::Horizontal, 12);
        
        // Drive icon based on drive type
        let drive_icon = Self::get_drive_icon(&drive.name, &drive.path);
        let icon_label = Label::new(Some(&drive_icon));
        icon_label.set_css_classes(&["drive-icon"]);
        icon_label.set_size_request(32, 32);
        
        // Drive name
        let name_label = Label::new(Some(&drive.name));
        name_label.set_css_classes(&["drive-name"]);
        name_label.set_halign(Align::Start);
        name_label.set_hexpand(true);
        
        // Usage percentage
        let usage_label = Label::new(Some(&format!("{:.1}%", drive.usage_percent)));
        usage_label.set_css_classes(&["drive-usage"]);
        
        header.append(&icon_label);
        header.append(&name_label);
        header.append(&usage_label);
        
        // Progress bar
        let progress = ProgressBar::new();
        progress.set_fraction(drive.usage_percent / 100.0);
        
        // Set color based on usage
        let color_class = match drive.usage_percent {
            p if p < 60.0 => "progress-green",
            p if p < 80.0 => "progress-blue", 
            p if p < 90.0 => "progress-yellow",
            p if p < 95.0 => "progress-orange",
            _ => "progress-red",
        };
        progress.set_css_classes(&[color_class]);
        
        // Space info
        let space_info = Label::new(Some(&format!(
            "{} / {}",
            Self::format_bytes(drive.used_space),
            Self::format_bytes(drive.total_space)
        )));
        space_info.set_css_classes(&["drive-space-info"]);
        space_info.set_halign(Align::Start);
        
        item.append(&header);
        item.append(&progress);
        item.append(&space_info);
        
        // Make clickable
        let clickable_item = Button::new();
        clickable_item.set_css_classes(&["flat", "drive-button"]);
        clickable_item.set_hexpand(true);
        clickable_item.set_child(Some(&item));
        
        let path = drive.path.clone();
        clickable_item.connect_clicked(move |_| {
            debug_info("HOME_SCREEN", &format!("Opening drive: {}", path.display()));
            navigate_to_directory(path.clone());
            switch_to_browser_view();
        });
        
        // Create container
        let container = Box::new(Orientation::Vertical, 0);
        container.append(&clickable_item);
        container
    }

    fn create_recent_files_placeholder() -> Box {
        let content = Box::new(Orientation::Vertical, 8);
        
        // Category tabs placeholder
        let tabs_container = Box::new(Orientation::Horizontal, 8);
        tabs_container.set_margin_start(16);
        tabs_container.set_margin_end(8);
        tabs_container.set_margin_bottom(8);
        
        let categories = vec![
            ("📄", "Documents"),
            ("🖼️", "Images"),
            ("🎵", "Audio"),
            ("🎬", "Videos"),
            ("📦", "Archives"),
            ("💻", "Code"),
        ];
        
        for (icon, name) in categories {
            let tab = Self::create_category_tab(icon, name);
            tabs_container.append(&tab);
        }
        
        // Placeholder content
        let placeholder = Label::new(Some("🔍 Scanning for large files...\nThis feature will help you find and manage large files in your home directory."));
        placeholder.set_css_classes(&["recent-placeholder"]);
        placeholder.set_halign(Align::Center);
        placeholder.set_vexpand(true);
        placeholder.set_margin_start(16);
        placeholder.set_margin_end(16);
        placeholder.set_margin_top(16);
        placeholder.set_margin_bottom(16);
        
        content.append(&tabs_container);
        content.append(&placeholder);
        
        content
    }

    fn create_category_tab(icon: &str, name: &str) -> Button {
        let tab = Button::new();
        tab.set_css_classes(&["flat", "category-tab"]);
        tab.set_size_request(60, 40);
        
        let tab_content = Box::new(Orientation::Vertical, 4);
        tab_content.set_halign(Align::Center);
        
        let icon_label = Label::new(Some(icon));
        icon_label.set_css_classes(&["tab-icon"]);
        
        let name_label = Label::new(Some(name));
        name_label.set_css_classes(&["tab-name"]);
        name_label.set_halign(Align::Center);
        
        tab_content.append(&icon_label);
        tab_content.append(&name_label);
        tab.set_child(Some(&tab_content));
        
        tab
    }

    fn get_drive_icon(name: &str, path: &PathBuf) -> &'static str {
        match name.to_lowercase().as_str() {
            "root" => "💽",
            "home" => "🏠",
            "boot" => "🚀",
            "var" => "📊",
            "temp" => "🗂️",
            _ => {
                // Check if it's a removable drive
                if path.to_string_lossy().contains("/media/") || 
                   path.to_string_lossy().contains("/mnt/") ||
                   path.to_string_lossy().contains("/run/media/") {
                    "💿" // Removable drive
                } else if path.to_string_lossy().contains("ssd") || 
                         path.to_string_lossy().contains("nvme") {
                    "⚡" // SSD
                } else {
                    "💾" // Regular drive
                }
            }
        }
    }

    fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

#[derive(Clone)]
struct DriveInfo {
    name: String,
    path: PathBuf,
    total_space: u64,
    used_space: u64,
    usage_percent: f64,
}

pub fn create_home_screen() -> Box {
    let home_screen = HomeScreen::new();
    home_screen.container
}
