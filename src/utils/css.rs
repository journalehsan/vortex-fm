use gtk::{CssProvider};

pub fn load_css() {
    let provider = CssProvider::new();
    let css = include_str!("../../resources/style.css");
    
    provider.load_from_data(css);
    
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
