use gtk::prelude::*;
use gtk::{gio, glib, Application};
use crate::common::constants::APP_ID;
use crate::views::main_window::build_ui;
// use crate::utils::css::load_css; // Disabled for now

mod core;
mod views;
mod widgets;
mod common;
mod utils;

fn main() -> glib::ExitCode {
    // Create application
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_startup(|_| {
        // Initialize debug system
        crate::utils::simple_debug::init_debug();
        
        // crate::utils::css::load_css(); // Temporarily disabled due to GTK issues
        crate::utils::simple_debug::debug_info("MAIN", "Vortex FM starting up...");
    });

    app.connect_activate(build_ui);
    app.connect_open(|app, files, _hint| {
        // Handle opening files/folders
        if let Some(file) = files.first() {
            if let Some(path) = file.path() {
                println!("📁 Opening: {}", path.display());
            }
        }
        build_ui(app);
    });

    app.run()
}