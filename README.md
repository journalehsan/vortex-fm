# Vortex File Manager 🚀

A modern, Windows Explorer-like file manager built with GTK4 and Rust for Linux systems.

## Features

- 🖥️ **Windows Explorer-like Interface**: Familiar layout with sidebar and main content area
- 📁 **Quick Access Sidebar**: Easy navigation to common directories
- 🧭 **Path Navigation Bar**: Back, forward, up, and refresh buttons
- 🔍 **Search Functionality**: Built-in search for files and folders
- 📋 **Grid View**: Icon-based file display similar to Windows Explorer
- 🎨 **Modern Styling**: Clean, professional appearance with CSS theming
- ⚡ **Fast Performance**: Built with Rust for optimal speed

## Requirements

- Rust 1.70+
- GTK4 development libraries
- Linux (tested on PikaOS with Hyprland)

## Installation

### Install Dependencies

```bash
# Ubuntu/Debian/PikaOS
sudo apt update
sudo apt install libgtk-4-dev libadwaita-1-dev build-essential

# Arch Linux
sudo pacman -S gtk4 libadwaita base-devel

# Fedora
sudo dnf install gtk4-devel libadwaita-devel gcc
```

### Build and Run

```bash
# Clone the repository
git clone <repository-url>
cd vortex-fm

# Build and run
cargo run

# Or build release version
cargo build --release
./target/release/vortex-fm
```

## Project Structure

```
vortex-fm/
├── src/
│   └── main.rs          # Main application code
├── resources/
│   └── style.css        # CSS styling
├── Cargo.toml           # Rust dependencies
├── build.rs             # Build script
└── README.md            # This file
```

## Development

The application is structured with modular functions for different UI components:

- `build_ui()` - Main window setup
- `create_sidebar()` - Left sidebar with quick access
- `create_content_area()` - Main content area
- `create_path_bar()` - Navigation toolbar
- `create_file_list()` - File grid display
- `create_status_bar()` - Bottom status bar

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Roadmap

- [ ] File operations (copy, move, delete, rename)
- [ ] Context menus
- [ ] File preview
- [ ] Tabbed browsing
- [ ] Bookmarks
- [ ] Themes and customization
- [ ] Plugin system