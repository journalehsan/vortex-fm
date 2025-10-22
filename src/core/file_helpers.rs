// Helper functions for file operations
// These are private helpers called from app.rs

use mime_guess::Mime;
use std::path::{Path, PathBuf};
use crate::app::App;
use crate::utils::spawn_detached::spawn_detached;
use crate::utils::mime_app;

/// Launch desktop entry files with Exec field
pub(crate) fn launch_desktop_entries(paths: &[impl AsRef<Path>]) {
    for path in paths.iter().map(AsRef::as_ref) {
        match freedesktop_entry_parser::parse_entry(path) {
            Ok(entry) => match entry.section("Desktop Entry").attr("Exec") {
                Some(exec) => match mime_app::exec_to_command(exec, &[] as &[&str; 0]) {
                    Some(commands) => {
                        for mut command in commands {
                            if let Err(err) = spawn_detached(&mut command) {
                                log::warn!("failed to execute {:?}: {}", path, err);
                            }
                        }
                    }
                    None => {
                        log::warn!("failed to parse {:?}: invalid Desktop Entry/Exec", path);
                    }
                },
                None => {
                    log::warn!("failed to parse {:?}: missing Desktop Entry/Exec", path);
                }
            },
            Err(err) => {
                log::warn!("failed to parse {:?}: {}", path, err);
            }
        }
    }
}

/// Check MIME app cache and launch application if found
pub(crate) fn launch_from_mime_cache<P>(app: &App, mime: &Mime, paths: &[P]) -> bool
where
    P: std::fmt::Debug + AsRef<Path> + AsRef<std::ffi::OsStr>,
{
    for app_mime in app.mime_app_cache.get(mime) {
        let Some(commands) = app_mime.command(paths) else {
            continue;
        };
        let len = commands.len();

        for (i, mut command) in commands.into_iter().enumerate() {
            match spawn_detached(&mut command) {
                Ok(()) => {
                    if i == len - 1 {
                        return true;
                    }
                }
                Err(err) => {
                    log::warn!("failed to execute command: {}", err);
                }
            }
        }
    }
    false
}
