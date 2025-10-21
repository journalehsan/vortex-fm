// Debug Color Changes Example
// This example demonstrates the comprehensive logging added to debug color changes

use vortex_fm::{
    utils::desktop_theme::{detect_desktop_environment, get_theme_manager},
    utils::themes::manager::{ColorContext, ThemeStaged},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    println!("🔍 Debug Color Changes Example");
    println!("==============================");
    
    // Detect desktop environment
    let desktop_environment = detect_desktop_environment();
    println!("🖥️  Detected Desktop Environment: {:?}", desktop_environment);
    
    if desktop_environment == vortex_fm::utils::desktop_theme::DesktopEnvironment::Cosmic {
        println!("✅ Cosmic desktop detected - advanced theme customization available");
        
        // Test the custom color picker functionality with logging
        if let Some(theme_manager_mutex) = get_theme_manager() {
            println!("🔧 Accessing theme manager for custom color selection");
            
            let mut theme_manager_guard = theme_manager_mutex.lock().unwrap();
            if theme_manager_guard.is_none() {
                println!("🎨 Initializing theme manager");
                *theme_manager_guard = Some(vortex_fm::utils::themes::manager::ThemeManager::new(desktop_environment));
            }
            
            if let Some(theme_manager) = theme_manager_guard.as_mut() {
                println!("🎨 Testing custom color application with logging:");
                
                // Test 1: Apply a custom color
                let custom_color = cosmic::iced::Color::from_rgb(0.86, 0.20, 0.20); // Dracula red
                println!("🎨 Testing color: Dracula red ({:?})", custom_color);
                
                if let Some(_staged) = theme_manager.set_color(Some(custom_color), ColorContext::CustomAccent) {
                    println!("✅ Custom accent color set successfully");
                    let _ = theme_manager.build_theme(ThemeStaged::Current);
                    println!("✅ Custom theme built and applied");
                    
                    // Check if the color was actually applied
                    if let Some(applied_color) = theme_manager.get_color(ColorContext::CustomAccent) {
                        println!("🎨 Verified applied color: {:?}", applied_color);
                    } else {
                        println!("❌ Could not verify applied color");
                    }
                } else {
                    println!("❌ Failed to set custom accent color");
                }
                
                // Test 2: Apply a different color
                let custom_color2 = cosmic::iced::Color::from_rgb(0.24, 0.60, 0.89); // Blue
                println!("\n🎨 Testing color: Blue ({:?})", custom_color2);
                
                if let Some(_staged) = theme_manager.set_color(Some(custom_color2), ColorContext::CustomAccent) {
                    println!("✅ Custom accent color set successfully");
                    let _ = theme_manager.build_theme(ThemeStaged::Current);
                    println!("✅ Custom theme built and applied");
                    
                    // Check if the color was actually applied
                    if let Some(applied_color) = theme_manager.get_color(ColorContext::CustomAccent) {
                        println!("🎨 Verified applied color: {:?}", applied_color);
                    } else {
                        println!("❌ Could not verify applied color");
                    }
                } else {
                    println!("❌ Failed to set custom accent color");
                }
                
                // Test 3: Get the cosmic theme
                println!("\n🎨 Testing cosmic theme creation:");
                let cosmic_theme = theme_manager.cosmic_theme();
                println!("✅ Cosmic theme created successfully");
                
            } else {
                println!("❌ Theme manager is None");
            }
        } else {
            println!("❌ Theme manager not available");
        }
    } else {
        println!("ℹ️  Custom color picker is optimized for Cosmic desktop");
        println!("   On other desktop environments, basic theme switching is available");
    }
    
    println!("\n🎉 Debug Color Changes Example completed!");
    println!("\n📋 What to look for in the logs:");
    println!("   1. 🎨 ThemeManager::set_color called with context and color");
    println!("   2. 🎨 ThemeCustomizer::set_accent called with color");
    println!("   3. 🎨 Setting accent color in ThemeBuilder");
    println!("   4. ✅ Successfully set accent color in ThemeBuilder");
    println!("   5. 🎨 ThemeManager::build_theme called with stage");
    println!("   6. 🎨 ThemeManager::cosmic_theme called");
    println!("   7. 🎨 Current theme with accent color");
    
    Ok(())
}
