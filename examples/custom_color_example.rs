// Custom Color Picker Example
// This example demonstrates the custom color picker feature in the settings dialog

use vortex_fm::utils::desktop_theme::{detect_desktop_environment, get_theme_manager};
use vortex_fm::utils::themes::manager::{ColorContext, ThemeStaged};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    println!("🎨 Custom Color Picker Example");
    println!("================================");
    
    // Detect desktop environment
    let desktop = detect_desktop_environment();
    println!("🖥️  Detected Desktop Environment: {:?}", desktop);
    
    // Check if advanced theming is available
    if let Some(theme_manager_mutex) = get_theme_manager() {
        println!("✅ Advanced theming is available on this desktop");
        
        // Demonstrate color selection
        let mut theme_manager_guard = theme_manager_mutex.lock().unwrap();
        if let Some(theme_manager) = theme_manager_guard.as_mut() {
            println!("🎨 Available color contexts:");
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
            
            // Demonstrate setting a custom color
            let custom_red = cosmic::iced::Color::from_rgb(0.86, 0.20, 0.20);
            if let Some(_staged) = theme_manager.set_color(Some(custom_red), ColorContext::CustomAccent) {
                println!("🎨 Set custom accent color to red: {:?}", custom_red);
                let _ = theme_manager.build_theme(ThemeStaged::Current);
                println!("✅ Custom color theme applied successfully");
            }
        }
    } else {
        println!("ℹ️  Advanced theming only available on Cosmic desktop");
        println!("   The custom color picker will be available in the settings dialog");
        println!("   when running on Cosmic desktop environment");
    }
    
    println!("\n🎉 Custom Color Picker Example completed!");
    println!("\nTo use the custom color picker:");
    println!("1. Run the main application: cargo run");
    println!("2. Open Settings (View → Settings)");
    println!("3. Look for the 'Custom Colors' section");
    println!("4. Click on any of the color buttons to apply that color");
    println!("5. The color will be applied to the Cosmic theme system");
    
    Ok(())
}
