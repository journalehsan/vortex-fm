+++
title = "Installation Guide"
description = "Complete guide to installing Vortex File Manager on your Linux system"
+++

# Installation Guide

This guide will help you install Vortex File Manager on your Linux system.

## Prerequisites

Before installing Vortex File Manager, ensure you have:

- **Rust 1.70+** installed on your system
- **libcosmic development libraries** for your distribution
- **Git** for cloning the repository
- **Build tools** (gcc, make, etc.)

## Distribution-Specific Installation

### Ubuntu/Debian

```bash
# Update package list
sudo apt update

# Install dependencies
sudo apt install libcosmic-dev build-essential git

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Arch Linux

```bash
# Install dependencies
sudo pacman -S libcosmic base-devel git

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Fedora

```bash
# Install dependencies
sudo dnf install libcosmic-devel gcc git

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Building from Source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/journalehsan/vortex-fm.git
   cd vortex-fm
   ```

2. **Build the application:**
   ```bash
   # Debug build
   cargo build
   
   # Release build (recommended)
   cargo build --release
   ```

3. **Run the application:**
   ```bash
   # Debug version
   cargo run
   
   # Release version
   ./target/release/vortex-fm
   ```

## Installation Options

### System-wide Installation

To install Vortex FM system-wide:

```bash
# Build release version
cargo build --release

# Copy binary to system path
sudo cp target/release/vortex-fm /usr/local/bin/

# Make executable
sudo chmod +x /usr/local/bin/vortex-fm
```

### User Installation

To install for your user only:

```bash
# Build release version
cargo build --release

# Create local bin directory
mkdir -p ~/.local/bin

# Copy binary
cp target/release/vortex-fm ~/.local/bin/

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Desktop Integration

### Desktop File

Create a desktop file for your application launcher:

```bash
# Create desktop file
cat > ~/.local/share/applications/vortex-fm.desktop << EOF
[Desktop Entry]
Name=Vortex File Manager
Comment=Modern file manager with system-wide theme sync
Exec=vortex-fm
Icon=vortex-fm
Terminal=false
Type=Application
Categories=System;FileManager;
EOF
```

### Set as Default File Manager

To set Vortex FM as your default file manager:

```bash
# Update MIME associations
xdg-mime default vortex-fm.desktop inode/directory
```

## Verification

After installation, verify everything works:

```bash
# Check if binary is accessible
which vortex-fm

# Run the application
vortex-fm --version

# Test theme synchronization
vortex-fm --enable-system-themes
```

## Troubleshooting

### Common Issues

**"libcosmic not found" error:**
- Ensure libcosmic development packages are installed
- Check that pkg-config can find the libraries: `pkg-config --libs libcosmic`

**Build fails with "rustc not found":**
- Ensure Rust is properly installed and in your PATH
- Run `rustup update` to ensure you have the latest version

**Application won't start:**
- Check that all dependencies are installed
- Verify the binary has execute permissions
- Check system logs for error messages

### Getting Help

If you encounter issues:

1. Check the [Troubleshooting Guide](#troubleshooting)
2. Search existing [GitHub Issues](https://github.com/journalehsan/vortex-fm/issues)
3. Create a new issue with detailed information about your system and the problem

## Next Steps

Once installed, check out:

- **First Steps** - Getting started with Vortex FM
- **Theme System** - Understanding the Omarchy theme system
- **Basic Usage** - Learning the interface
