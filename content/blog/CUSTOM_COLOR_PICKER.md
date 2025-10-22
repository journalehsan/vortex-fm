+++
title = "Custom Color Picker Feature"
date = 2024-01-15
description = "Development insights and technical updates"
+++

# 🎨 Custom Color Picker Feature

## Overview

The custom color picker feature allows users to manually select from popular Omarchy theme colors directly in the settings dialog. This provides an easy way to customize the application's accent colors without needing to modify theme files manually.

## Features

### ✨ **Color Selection Interface**
- **Visual Color Buttons**: 8 popular Omarchy theme colors displayed as clickable buttons
- **Real-time Application**: Colors are applied immediately when selected
- **Desktop Integration**: Works seamlessly with the Cosmic theme system

### 🎯 **Available Colors**
The color picker includes accent colors from popular Omarchy themes:
- **Catppuccin**: Soft blue accent
- **Dracula**: Bold red accent  
- **Everforest**: Vibrant green accent
- **Gruvbox**: Warm yellow accent
- **Kanagawa**: Classic blue accent
- **Matte Black**: Modern blue accent
- **Nord**: Cool blue accent
- **Tokyo Night**: Deep blue accent

### 🔧 **Technical Implementation**

#### **Settings Integration**
- Added to the main settings dialog under "Custom Colors" section
- Integrated with existing settings UI structure
- Uses Cosmic widget system for consistent styling

#### **Message Handling**
```rust
Message::CustomColor(cosmic::iced::Color) => {
    // Apply custom color using ThemeManager
    if let Some(theme_manager) = get_theme_manager() {
        theme_manager.set_color(Some(color), ColorContext::CustomAccent);
        theme_manager.build_theme(ThemeStaged::Current);
    }
}
```

#### **Theme System Integration**
- Uses the advanced `ThemeManager` for color application
- Integrates with `ColorContext::CustomAccent` for accent color changes
- Applies changes using `ThemeStaged::Current` for immediate effect

## Usage

### 🚀 **How to Use**

1. **Open Settings**: Go to `View → Settings` in the main application
2. **Find Custom Colors**: Look for the "Custom Colors" section
3. **Select Color**: Click on any of the color buttons
4. **See Changes**: The accent color will be applied immediately

### 🖥️ **Desktop Compatibility**

- **Cosmic Desktop**: Full functionality with real-time color changes
- **Other Desktops**: Color picker is visible but changes are logged only
- **Fallback**: Graceful degradation on non-Cosmic environments

## Code Structure

### **Files Modified**

1. **`src/app.rs`**:
   - Added `CustomColor(cosmic::iced::Color)` message
   - Added custom color picker section to settings
   - Added message handler for color application

2. **`examples/custom_color_example.rs`**:
   - Demonstration of the color picker functionality
   - Shows desktop environment detection
   - Explains usage instructions

### **Key Components**

#### **Settings Section**
```rust
widget::settings::section()
    .title("Custom Colors")
    .add({
        // Color picker implementation
        let mut color_buttons = Vec::new();
        for theme in OMARCHY_THEMES.iter().take(8) {
            // Create color button for each theme
        }
        // Return settings item with color buttons
    })
```

#### **Message Handler**
```rust
Message::CustomColor(color) => {
    // Apply color using ThemeManager
    if let Some(theme_manager_mutex) = get_theme_manager() {
        let mut theme_manager_guard = theme_manager_mutex.lock().unwrap();
        if let Some(theme_manager) = theme_manager_guard.as_mut() {
            theme_manager.set_color(Some(color), ColorContext::CustomAccent);
            theme_manager.build_theme(ThemeStaged::Current);
        }
    }
}
```

## Benefits

### 🎨 **User Experience**
- **Easy Access**: Colors are just one click away in settings
- **Visual Selection**: See colors before applying them
- **Immediate Feedback**: Changes apply instantly
- **No File Editing**: No need to manually edit theme files

### 🔧 **Developer Experience**
- **Clean Integration**: Uses existing theme system architecture
- **Extensible**: Easy to add more colors or customize the interface
- **Maintainable**: Follows established patterns in the codebase

## Future Enhancements

### 🚀 **Potential Improvements**
- **More Colors**: Add additional Omarchy theme colors
- **Custom Colors**: Allow users to input custom hex colors
- **Color Categories**: Group colors by theme family
- **Preview**: Show color preview before applying
- **Persistence**: Save selected colors across sessions

### 🔧 **Technical Enhancements**
- **Color Validation**: Ensure colors meet accessibility standards
- **Theme Integration**: Apply colors to more UI elements
- **Export/Import**: Save and load color preferences
- **Advanced Picker**: Full color picker with hue/saturation controls

## Testing

### ✅ **Verification Steps**
1. Run the application: `cargo run`
2. Open Settings: `View → Settings`
3. Locate "Custom Colors" section
4. Click on color buttons
5. Verify color changes are applied (on Cosmic desktop)

### 🧪 **Example Usage**
```bash
# Run the demonstration example
cargo run --example custom_color_example

# Run the main application
cargo run
```

## Conclusion

The custom color picker feature provides a user-friendly way to customize the application's appearance using popular Omarchy theme colors. It integrates seamlessly with the existing theme system and provides immediate visual feedback to users.

The implementation follows the established patterns in the codebase and provides a solid foundation for future color customization features.
