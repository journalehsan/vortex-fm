// Example demonstrating advanced theme customization using ThemeBuilder
// This shows how to apply custom colors from external themes (Omarchy, etc.) to Cosmic

use cosmic::iced::Color;
use vortex_fm::utils::desktop_theme::{get_desktop_theme, apply_advanced_theme, get_theme_manager, detect_desktop_environment};
use vortex_fm::utils::themes::manager::{ColorContext, ThemeStaged, ThemeManager};

fn main() {
    // Initialize logging
    env_logger::init();
    
    println!("🎨 Vortex FM Advanced Theme Example");
    println!("=====================================");
    
    // Detect and get the current desktop theme
    let theme_info = get_desktop_theme();
    println!("📱 Detected theme: {}", theme_info.name);
    println!("🌓 Theme type: {}", if theme_info.is_light { "Light" } else { "Dark" });
    println!("🎨 Colors:");
    println!("  - Window BG: {:?}", theme_info.window_background);
    println!("  - View BG: {:?}", theme_info.view_background);
    println!("  - Accent: {:?}", theme_info.accent_color);
    println!("  - Text: {:?}", theme_info.foreground);
    
    // Apply the theme with advanced customization
    let cosmic_theme = apply_advanced_theme(&theme_info);
    println!("✅ Applied advanced theme to Cosmic");
    
    // If we're on Cosmic desktop, we can do additional customization
    if let Some(theme_manager_mutex) = get_theme_manager() {
        println!("🔧 Advanced customization available on Cosmic desktop");
        
        let mut theme_manager_guard = theme_manager_mutex.lock().unwrap();
        
        // Initialize theme manager if needed
        if theme_manager_guard.is_none() {
            let desktop = detect_desktop_environment();
            *theme_manager_guard = Some(ThemeManager::new(desktop));
        }
        
        if let Some(theme_manager) = theme_manager_guard.as_mut() {
            // Example: Get current accent color
            if let Some(accent_color) = theme_manager.get_color(ColorContext::CustomAccent) {
                println!("🎨 Current accent color: {:?}", accent_color);
            }
            
            // Example: Set a custom accent color (red)
            let custom_red = Color::from_rgb(0.86, 0.20, 0.20); // Dracula red
            if let Some(staged) = theme_manager.set_color(Some(custom_red), ColorContext::CustomAccent) {
                println!("🎨 Set custom accent color to red");
                
                // Build the theme with the new color
                let _ = theme_manager.build_theme(staged);
                println!("✅ Applied custom accent color");
            }
            
            // Example: Get all available colors
            println!("🎨 Available theme colors:");
            for context in [
                ColorContext::CustomAccent,
                ColorContext::ApplicationBackground,
                ColorContext::ContainerBackground,
                ColorContext::InterfaceText,
                ColorContext::ControlComponent,
                ColorContext::AccentWindowHint,
            ] {
                if let Some(color) = theme_manager.get_color(context) {
                    println!("  - {:?}: {:?}", context, color);
                }
            }
        }
    } else {
        println!("ℹ️  Advanced customization only available on Cosmic desktop");
    }
    
    println!("🎉 Theme example completed!");
}
