// Simplified Custom Color Picker Example
// This example demonstrates the new simplified custom color picker feature

use vortex_fm::{
    utils::desktop_theme::{detect_desktop_environment, get_theme_manager},
    utils::themes::manager::{ColorContext, ThemeStaged},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    println!("🎨 Simplified Custom Color Picker Example");
    println!("==========================================");
    
    // Detect desktop environment
    let desktop_environment = detect_desktop_environment();
    println!("🖥️  Detected Desktop Environment: {:?}", desktop_environment);
    
    if desktop_environment == vortex_fm::utils::desktop_theme::DesktopEnvironment::Cosmic {
        println!("✅ Cosmic desktop detected - advanced theme customization available");
        
        // Demonstrate the custom color picker functionality
        if let Some(theme_manager_mutex) = get_theme_manager() {
            println!("🔧 Accessing theme manager for custom color selection");
            
            let mut theme_manager_guard = theme_manager_mutex.lock().unwrap();
            if theme_manager_guard.is_none() {
                *theme_manager_guard = Some(vortex_fm::utils::themes::manager::ThemeManager::new(desktop_environment));
            }
            
            if let Some(theme_manager) = theme_manager_guard.as_mut() {
                println!("🎨 Available color schemes:");
                println!("   • catppuccin - Soft blue accent");
                println!("   • dracula - Bold red accent");
                println!("   • everforest - Vibrant green accent");
                println!("   • gruvbox - Warm yellow accent");
                println!("   • kanagawa - Classic blue accent");
                println!("   • matte-black - Modern blue accent");
                println!("   • nord - Cool blue accent");
                println!("   • tokyo-night - Dark blue accent");
                
                // Simulate selecting a custom color
                let custom_color = cosmic::iced::Color::from_rgb(0.86, 0.20, 0.20); // Dracula red
                println!("🎨 Simulating custom color selection: Dracula red");
                
                if let Some(_staged) = theme_manager.set_color(Some(custom_color), ColorContext::CustomAccent) {
                    println!("✅ Custom accent color set successfully");
                    let _ = theme_manager.build_theme(ThemeStaged::Current);
                    println!("✅ Custom theme applied successfully");
                }
                
                // Show current color
                if let Some(current_color) = theme_manager.get_color(ColorContext::CustomAccent) {
                    println!("🎨 Current accent color: {:?}", current_color);
                }
            }
        } else {
            println!("❌ Theme manager not available");
        }
    } else {
        println!("ℹ️  Custom color picker is optimized for Cosmic desktop");
        println!("   On other desktop environments, basic theme switching is available");
    }
    
    println!("\n🎉 Simplified Custom Color Picker Example completed!");
    println!("\n📋 How to use in the application:");
    println!("   1. Open Settings");
    println!("   2. Go to Appearance section");
    println!("   3. Select 'Custom' from the theme dropdown");
    println!("   4. Choose a color scheme from the dropdown");
    println!("   5. Click colors in the palette for preview");
    println!("   6. Click 'Apply' to apply the custom theme");
    
    Ok(())
}
