# Vortex File Manager 🚀

A modern, Windows Explorer-like file manager built with Iced and libcosmic for Linux systems.

> [!NOTE]
> This project is a fork of [cosmic-files](https://github.com/pop-os/cosmic-files) by [Ehsan Tork](https://journalehsan.github.io/) and is still a work in progress

## Features

- 🖥️ **Windows Explorer-like Interface**: Familiar layout with sidebar and main content area
- 📁 **Quick Access Sidebar**: Easy navigation to common directories
- 🧭 **Path Navigation Bar**: Back, forward, up, and refresh buttons
- 🔍 **Search Functionality**: Built-in search for files and folders
- 📋 **Grid View**: Icon-based file display similar to Windows Explorer
- 🎨 **Modern Styling**: Clean, professional appearance with libcosmic theming
- ⚡ **Fast Performance**: Built with Rust for optimal speed
- 🦀 **Rust Native**: Uses Iced and libcosmic instead of GTK4 for better Rust integration

## Requirements

- Rust 1.70+
- libcosmic development libraries
- Linux (tested on Arch Linux)

## Installation

### Install Dependencies

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libcosmic-dev build-essential

# Arch Linux
sudo pacman -S libcosmic base-devel

# Fedora
sudo dnf install libcosmic-devel gcc
```

### Build and Run

```bash
# Clone the repository
git clone https://github.com/journalehsan/vortex-fm.git
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
│   ├── app.rs              # Main application logic
│   ├── lib.rs              # Library entry point
│   └── main.rs             # Application entry point
├── vortex-files-applet/    # Applet components
├── examples/               # Example code
├── i18n/                   # Internationalization
├── res/                    # Resources and icons
├── Cargo.toml              # Rust dependencies
└── README.md               # This file
```

## Development

The application is structured with modular components:

- **App Module**: Main application logic and state management
- **Tab System**: Multi-tab file browsing
- **Navigation**: File system navigation and history
- **Operations**: File operations (copy, move, delete, etc.)
- **UI Components**: Modern UI built with Iced and libcosmic

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under [GPLv3](LICENSE)

## Acknowledgments

This project is a fork of [cosmic-files](https://github.com/pop-os/cosmic-files) by [Ehsan Tork](https://journalehsan.github.io/). The original COSMIC desktop environment is maintained by System76 for use in Pop!_OS. A list of all COSMIC projects can be found in the [cosmic-epoch](https://github.com/pop-os/cosmic-epoch) project's README.

## Roadmap

- [ ] File operations (copy, move, delete, rename)
- [ ] Context menus
- [ ] File preview
- [ ] Tabbed browsing
- [ ] Bookmarks
- [ ] Themes and customization
- [ ] Plugin system
