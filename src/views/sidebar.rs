use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Button, Separator};
use std::path::PathBuf;
use crate::core::file_manager::FileManagerState;
use crate::core::navigation::navigate_to_directory;

pub fn create_sidebar(_state: &FileManagerState) -> Box {
    let sidebar = Box::new(Orientation::Vertical, 12);
    sidebar.set_margin_start(12);
    sidebar.set_margin_end(12);
    sidebar.set_margin_top(12);
    sidebar.set_margin_bottom(12);
    sidebar.add_css_class("sidebar");
    
    // Quick access section
    let quick_access_label = Label::new(Some("Quick Access"));
    quick_access_label.add_css_class("title-4");
    sidebar.append(&quick_access_label);
    
    // Quick access buttons
    let home_btn = Button::with_label("🏠 Home");
    let docs_btn = Button::with_label("📄 Documents");
    let downloads_btn = Button::with_label("📥 Downloads");
    let pictures_btn = Button::with_label("🖼️ Pictures");
    let music_btn = Button::with_label("🎵 Music");
    let videos_btn = Button::with_label("🎬 Videos");
    
    // Connect navigation handlers
    let home_path = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
    
    home_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            navigate_to_directory(PathBuf::from(&home_path));
        }
    });
    
    docs_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let docs_path = PathBuf::from(&home_path).join("Documents");
            navigate_to_directory(docs_path);
        }
    });
    
    downloads_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let downloads_path = PathBuf::from(&home_path).join("Downloads");
            navigate_to_directory(downloads_path);
        }
    });
    
    pictures_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let pictures_path = PathBuf::from(&home_path).join("Pictures");
            navigate_to_directory(pictures_path);
        }
    });
    
    music_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let music_path = PathBuf::from(&home_path).join("Music");
            navigate_to_directory(music_path);
        }
    });
    
    videos_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let videos_path = PathBuf::from(&home_path).join("Videos");
            navigate_to_directory(videos_path);
        }
    });
    
    sidebar.append(&home_btn);
    sidebar.append(&docs_btn);
    sidebar.append(&downloads_btn);
    sidebar.append(&pictures_btn);
    sidebar.append(&music_btn);
    sidebar.append(&videos_btn);
    
    // Add separator
    let separator = Separator::new(Orientation::Horizontal);
    sidebar.append(&separator);
    
    // This PC section
    let this_pc_label = Label::new(Some("This PC"));
    this_pc_label.add_css_class("title-4");
    sidebar.append(&this_pc_label);
    
    let desktop_btn = Button::with_label("🖥️ Desktop");
    let documents_btn = Button::with_label("📁 Documents");
    let downloads_btn2 = Button::with_label("📥 Downloads");
    let pictures_btn2 = Button::with_label("🖼️ Pictures");
    let music_btn2 = Button::with_label("🎵 Music");
    let videos_btn2 = Button::with_label("🎬 Videos");
    
    // Connect navigation handlers for This PC section
    let home_path = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
    
    desktop_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let desktop_path = PathBuf::from(&home_path).join("Desktop");
            navigate_to_directory(desktop_path);
        }
    });
    
    documents_btn.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let docs_path = PathBuf::from(&home_path).join("Documents");
            navigate_to_directory(docs_path);
        }
    });
    
    downloads_btn2.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let downloads_path = PathBuf::from(&home_path).join("Downloads");
            navigate_to_directory(downloads_path);
        }
    });
    
    pictures_btn2.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let pictures_path = PathBuf::from(&home_path).join("Pictures");
            navigate_to_directory(pictures_path);
        }
    });
    
    music_btn2.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let music_path = PathBuf::from(&home_path).join("Music");
            navigate_to_directory(music_path);
        }
    });
    
    videos_btn2.connect_clicked({
        let home_path = home_path.clone();
        move |_| {
            let videos_path = PathBuf::from(&home_path).join("Videos");
            navigate_to_directory(videos_path);
        }
    });
    
    sidebar.append(&desktop_btn);
    sidebar.append(&documents_btn);
    sidebar.append(&downloads_btn2);
    sidebar.append(&pictures_btn2);
    sidebar.append(&music_btn2);
    sidebar.append(&videos_btn2);
    
    sidebar
}
