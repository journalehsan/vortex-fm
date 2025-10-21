// Enhanced drive detection for Vortex File Manager
// Based on comprehensive cross-platform drive detection

use crate::utils::command_utils::SafeCommand;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DriveInfo {
    pub path: PathBuf,
    pub label: String,
    pub drive_type: DriveType,
    pub total_space: Option<u64>,
    pub free_space: Option<u64>,
    pub is_accessible: bool,
}

#[derive(Debug, Clone)]
pub enum DriveType {
    FixedDisk,
    RemovableDisk,
    NetworkDrive,
    CDRom,
    RamDisk,
    Unknown,
}

impl DriveInfo {
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

    pub fn get_usage_percent(&self) -> Option<f64> {
        match (self.total_space, self.free_space) {
            (Some(total), Some(free)) if total > 0 => {
                let used = total - free;
                Some((used as f64 / total as f64) * 100.0)
            }
            _ => None,
        }
    }
}

pub struct DriveDetector;

impl DriveDetector {
    /// Get all available drives for the current platform
    pub fn get_available_drives() -> Vec<DriveInfo> {
        #[cfg(target_os = "windows")]
        {
            Self::get_windows_drives()
        }

        #[cfg(target_os = "linux")]
        {
            Self::get_linux_drives()
        }

        #[cfg(target_os = "macos")]
        {
            Self::get_macos_drives()
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            Vec::new()
        }
    }

    /// Get Windows drives using WMIC
    #[cfg(target_os = "windows")]
    fn get_windows_drives() -> Vec<DriveInfo> {
        let mut drives = Vec::new();

        // Method 1: Try WMIC with CSV format for better parsing
        if let Ok(wmic_drives) = Self::get_windows_drives_wmic() {
            if !wmic_drives.is_empty() {
                return wmic_drives;
            }
        }

        // Method 2: Fallback to simple letter enumeration
        drives.extend(Self::get_windows_drives_simple());

        drives
    }

    #[cfg(target_os = "windows")]
    fn get_windows_drives_wmic() -> Result<Vec<DriveInfo>, Box<dyn std::error::Error>> {
        let mut drives = Vec::new();

        let output = SafeCommand::new("wmic")
            .args(&[
                "logicaldisk",
                "get",
                "DeviceID,DriveType,Size,FreeSpace,VolumeName",
                "/format:csv",
            ])
            .output()?;

        if !output.status.success() {
            return Err("WMIC command failed".into());
        }

        let stdout = String::from_utf8(output.stdout)?;

        for line in stdout.lines().skip(1) {
            // Skip header
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 6 {
                // CSV format: Node,DeviceID,DriveType,FreeSpace,Size,VolumeName
                if let (
                    Some(device_id),
                    Some(drive_type_str),
                    free_space_str,
                    size_str,
                    volume_name,
                ) = (
                    parts.get(1),
                    parts.get(2),
                    parts.get(3),
                    parts.get(4),
                    parts.get(5),
                ) {
                    if device_id.is_empty() {
                        continue;
                    }

                    let path = PathBuf::from(format!("{}\\", device_id));
                    let drive_type = match drive_type_str.parse::<u32>().unwrap_or(0) {
                        2 => DriveType::RemovableDisk,
                        3 => DriveType::FixedDisk,
                        4 => DriveType::NetworkDrive,
                        5 => DriveType::CDRom,
                        6 => DriveType::RamDisk,
                        _ => DriveType::Unknown,
                    };

                    let total_space = size_str.and_then(|s| s.parse::<u64>().ok());
                    let free_space = free_space_str.and_then(|s| s.parse::<u64>().ok());

                    let label = if volume_name.map_or(true, |s| s.is_empty()) {
                        format!("Local Disk ({})", device_id)
                    } else {
                        format!("{} ({})", volume_name.map_or("", |v| v), device_id)
                    };

                    let is_accessible = path.exists();

                    drives.push(DriveInfo {
                        path,
                        label,
                        drive_type,
                        total_space,
                        free_space,
                        is_accessible,
                    });
                }
            }
        }

        Ok(drives)
    }

    #[cfg(target_os = "windows")]
    fn get_windows_drives_simple() -> Vec<DriveInfo> {
        let mut drives = Vec::new();

        // Check drive letters A through Z
        for letter in 'A'..='Z' {
            let path = PathBuf::from(format!("{}:\\", letter));

            if path.exists() {
                // Try to get drive info
                let (total_space, free_space) = Self::get_windows_drive_space(&letter.to_string());

                let label = format!("Local Disk ({}:)", letter);

                drives.push(DriveInfo {
                    path,
                    label,
                    drive_type: DriveType::FixedDisk, // Default assumption
                    total_space,
                    free_space,
                    is_accessible: true,
                });
            }
        }

        drives
    }

    #[cfg(target_os = "windows")]
    fn get_windows_drive_space(drive_letter: &str) -> (Option<u64>, Option<u64>) {
        let output = SafeCommand::new("wmic")
            .args(&[
                "logicaldisk",
                "where",
                &format!("DeviceID='{}:'", drive_letter),
                "get",
                "Size,FreeSpace",
                "/format:list",
            ])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut total = None;
                let mut free = None;

                for line in stdout.lines() {
                    let line = line.trim();
                    if line.starts_with("Size=") && line.len() > 5 {
                        if let Ok(size) = line[5..].parse::<u64>() {
                            total = Some(size);
                        }
                    } else if line.starts_with("FreeSpace=") && line.len() > 10 {
                        if let Ok(free_space) = line[10..].parse::<u64>() {
                            free = Some(free_space);
                        }
                    }
                }

                return (total, free);
            }
        }

        (None, None)
    }

    /// Get Linux mounted filesystems
    #[cfg(target_os = "linux")]
    fn get_linux_drives() -> Vec<DriveInfo> {
        let mut drives = Vec::new();

        // Method 1: Parse /proc/mounts for mounted filesystems
        if let Ok(mounted) = Self::get_linux_mounted_drives() {
            drives.extend(mounted);
        }

        // Method 2: Add unmounted drives if easily detectable
        if let Ok(unmounted) = Self::get_linux_unmounted_drives() {
            drives.extend(unmounted);
        }

        // Always add root filesystem if not already present
        if !drives.iter().any(|d| d.path == PathBuf::from("/")) {
            if let Some(root_drive) = Self::create_linux_drive("/", "Root Filesystem") {
                drives.insert(0, root_drive);
            }
        }

        drives
    }

    #[cfg(target_os = "linux")]
    fn get_linux_mounted_drives() -> Result<Vec<DriveInfo>, Box<dyn std::error::Error>> {
        let mut drives = Vec::new();
        let mounts_content = fs::read_to_string("/proc/mounts")?;

        for line in mounts_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let device = parts[0];
                let mount_point = parts[1];
                let fs_type = parts[2];

                // Filter for relevant filesystem types and mount points
                if Self::is_relevant_linux_mount(device, mount_point, fs_type) {
                    if let Some(drive) = Self::create_linux_drive(
                        mount_point,
                        &Self::format_linux_drive_label(device, mount_point),
                    ) {
                        drives.push(drive);
                    }
                }
            }
        }

        // Sort by mount point length (root first, then deeper mounts)
        drives.sort_by(|a, b| {
            a.path
                .to_string_lossy()
                .len()
                .cmp(&b.path.to_string_lossy().len())
        });

        Ok(drives)
    }

    #[cfg(target_os = "linux")]
    fn get_linux_unmounted_drives() -> Result<Vec<DriveInfo>, Box<dyn std::error::Error>> {
        let mut drives = Vec::new();

        // Try to list block devices using lsblk
        let output = SafeCommand::new("lsblk")
            .args(&["-rno", "NAME,SIZE,TYPE,MOUNTPOINT"])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;

            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let device_name = parts[0];
                    let size = parts[1];
                    let device_type = parts[2];
                    let mount_point = parts[3];

                    // Look for unmounted disks or partitions
                    if (device_type == "disk" || device_type == "part") && mount_point.is_empty() {
                        let device_path = format!("/dev/{}", device_name);
                        let label = format!("Unmounted {} ({})", device_name, size);

                        drives.push(DriveInfo {
                            path: PathBuf::from(&device_path),
                            label,
                            drive_type: DriveType::FixedDisk,
                            total_space: None,
                            free_space: None,
                            is_accessible: false, // Not accessible until mounted
                        });
                    }
                }
            }
        }

        Ok(drives)
    }

    #[cfg(target_os = "linux")]
    fn is_relevant_linux_mount(device: &str, mount_point: &str, fs_type: &str) -> bool {
        // Include important mount points and filesystem types
        let relevant_fs_types = [
            "ext2", "ext3", "ext4", "xfs", "btrfs", "ntfs", "vfat", "exfat",
        ];
        let relevant_mounts = ["/", "/home", "/boot", "/var", "/usr", "/tmp"];

        // Include if it's a relevant filesystem type
        if relevant_fs_types.contains(&fs_type) {
            return true;
        }

        // Include important mount points
        if relevant_mounts.contains(&mount_point) {
            return true;
        }

        // Include /mnt/* and /media/* (common mount points)
        if mount_point.starts_with("/mnt/") || mount_point.starts_with("/media/") {
            return true;
        }

        // Include if it's a block device
        if device.starts_with("/dev/sd")
            || device.starts_with("/dev/nvme")
            || device.starts_with("/dev/hd")
        {
            return true;
        }

        false
    }

    #[cfg(target_os = "linux")]
    fn format_linux_drive_label(device: &str, mount_point: &str) -> String {
        match mount_point {
            "/" => "Root Filesystem".to_string(),
            "/home" => "Home".to_string(),
            "/boot" => "Boot".to_string(),
            "/var" => "Var".to_string(),
            "/usr" => "Usr".to_string(),
            "/tmp" => "Temporary".to_string(),
            _ => {
                if mount_point.starts_with("/mnt/") {
                    format!("Mounted: {}", &mount_point[5..])
                } else if mount_point.starts_with("/media/") {
                    format!("Media: {}", &mount_point[7..])
                } else {
                    format!("{} ({})", device, mount_point)
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn create_linux_drive(mount_point: &str, label: &str) -> Option<DriveInfo> {
        let path = PathBuf::from(mount_point);

        if !path.exists() {
            return None;
        }

        // Get disk usage using statvfs
        let (total_space, free_space) = Self::get_linux_disk_usage(mount_point);

        Some(DriveInfo {
            path,
            label: label.to_string(),
            drive_type: DriveType::FixedDisk,
            total_space,
            free_space,
            is_accessible: true,
        })
    }

    #[cfg(target_os = "linux")]
    fn get_linux_disk_usage(path: &str) -> (Option<u64>, Option<u64>) {
        // Try using df command
        let output = SafeCommand::new("df")
            .args(&["-B1", path]) // Get bytes directly
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.lines().collect();

                if lines.len() >= 2 {
                    let data_line = lines[1];
                    let parts: Vec<&str> = data_line.split_whitespace().collect();

                    if parts.len() >= 4 {
                        let total = parts[1].parse::<u64>().ok();
                        let available = parts[3].parse::<u64>().ok();
                        return (total, available);
                    }
                }
            }
        }

        (None, None)
    }

    /// Get macOS drives (placeholder for future implementation)
    #[cfg(target_os = "macos")]
    fn get_macos_drives() -> Vec<DriveInfo> {
        let mut drives = Vec::new();

        // Basic implementation - add root and common mount points
        if let Some(root_drive) = Self::create_macos_drive("/", "Macintosh HD") {
            drives.push(root_drive);
        }

        // Add /Volumes/* entries
        if let Ok(entries) = fs::read_dir("/Volumes") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown");

                    if let Some(drive) = Self::create_macos_drive(&path.to_string_lossy(), name) {
                        drives.push(drive);
                    }
                }
            }
        }

        drives
    }

    #[cfg(target_os = "macos")]
    fn create_macos_drive(mount_point: &str, label: &str) -> Option<DriveInfo> {
        let path = PathBuf::from(mount_point);

        if !path.exists() {
            return None;
        }

        Some(DriveInfo {
            path,
            label: label.to_string(),
            drive_type: DriveType::FixedDisk,
            total_space: None,
            free_space: None,
            is_accessible: true,
        })
    }

    /// Mount an unmounted Linux device (requires sudo)
    #[cfg(target_os = "linux")]
    pub fn mount_linux_device(
        device_path: &str,
        mount_point: Option<&str>,
    ) -> Result<PathBuf, String> {
        // Create mount point if not provided
        let mount_path = if let Some(mp) = mount_point {
            PathBuf::from(mp)
        } else {
            // Extract device name and create mount point in /mnt/
            let device_name = device_path.split('/').last().unwrap_or("unknown");
            PathBuf::from(format!("/mnt/{}", device_name))
        };

        // Create mount directory if it doesn't exist
        if !mount_path.exists() {
            let output = SafeCommand::new("sudo")
                .args(&["mkdir", "-p", &mount_path.to_string_lossy()])
                .output()
                .map_err(|e| format!("Failed to create mount directory: {}", e))?;

            if !output.status.success() {
                return Err("Failed to create mount directory".to_string());
            }
        }

        // Mount the device
        let output = SafeCommand::new("sudo")
            .args(&["mount", device_path, &mount_path.to_string_lossy()])
            .output()
            .map_err(|e| format!("Failed to mount device: {}", e))?;

        if output.status.success() {
            Ok(mount_path)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Mount failed: {}", stderr))
        }
    }
}