use gtk::prelude::*;
use gtk::{AboutDialog, ApplicationWindow};

pub fn show_about_dialog(parent: Option<&ApplicationWindow>) {
    let dialog = AboutDialog::new();
    
    // Basic info
    dialog.set_program_name(Some("Vortex FM"));
    dialog.set_version(Some("0.1.0"));
    dialog.set_copyright(Some("© 2025 Vortex FM Team"));
    dialog.set_license_type(gtk::License::Gpl30);
    
    dialog.set_website(Some("https://journalehsan.github.io"));
    dialog.set_website_label("Visit Website");
    
    // Authors - Human + AI Team
    let authors = vec![
        "Ehsan Tork (ehsan.tork@hey.com)",  // Human
        "Claude (Anthropic)",                // AI
        "DeepSeek AI",                       // AI
        "GitHub Copilot (Microsoft)",        // AI
    ];
    dialog.set_authors(&authors);
    
    // Comments/Description
    dialog.set_comments(Some(
        "A modern, feature-rich file manager built with GTK4 and Rust.\n\n\
         Crafted collaboratively by human developers and AI models \
         working together to create an intuitive file management experience."
    ));
    
    // Logo/icon
    dialog.set_logo_icon_name(Some("system-file-manager"));
    
    // Show the dialog
    if let Some(window) = parent {
        dialog.set_transient_for(Some(window));
    }
    dialog.set_modal(true);
    dialog.show();
}
