# Vortex FM Installation - Quick Start

## For Arch Linux (Easy Migration)

### Standard Installation
```bash
# Preview what will be done
./scripts/install_vortex_system.sh --dry-run

# Run the actual installation
sudo ./scripts/install_vortex_system.sh
```

This will:
✓ Build Vortex FM from source  
✓ Install binary to `/usr/local/bin/vortex-fm`  
✓ Create desktop entries  
✓ Create launcher shims for `nautilus` and `cosmic-files`  
✓ Remove old packages (optional)  
✓ Replace file manager in system menu  

### Safe Testing (No sudo needed)
```bash
./scripts/install_vortex_system.sh \
  --prefix=$HOME/.local \
  --no-remove
```

### Keep Old Packages Installed
```bash
sudo ./scripts/install_vortex_system.sh --no-remove
```
System will keep nautilus/cosmic-files but use Vortex when clicked.

### Replace System Binaries Directly
```bash
sudo ./scripts/install_vortex_system.sh --overwrite-system-binaries
```
⚠️ Backups created at `/usr/bin/{nautilus,cosmic-files}.bak.*`

## Verify Installation

```bash
# Check binary installed
which vortex-fm
vortex-fm --version

# Run tests
./scripts/test_install_vortex_system.sh
```

All tests should pass ✓

## Rollback

If something goes wrong:

```bash
# Restore original packages (if not removed)
sudo pacman -S nautilus cosmic-files

# Restore system binaries from backup
sudo mv /usr/bin/nautilus.bak.* /usr/bin/nautilus
sudo mv /usr/bin/cosmic-files.bak.* /usr/bin/cosmic-files

# Or if installed to ~/.local, just remove it
rm -rf ~/.local/bin/vortex-fm
```

## For omarchy Integration

Use in omarchy migration script:

```bash
#!/bin/bash
# Vortex FM installation for omarchy

echo "Installing Vortex File Manager..."

# Option 1: Full replacement (recommended)
bash vortex-fm/scripts/install_vortex_system.sh

# Option 2: Test first
# bash vortex-fm/scripts/install_vortex_system.sh --dry-run
# bash vortex-fm/scripts/install_vortex_system.sh

# Option 3: Keep old packages
# bash vortex-fm/scripts/install_vortex_system.sh --no-remove

echo "✓ Vortex File Manager installed!"
```

## Command Reference

```bash
./scripts/install_vortex_system.sh [OPTIONS]

OPTIONS:
  --prefix=PATH                 Install location (default: /usr/local)
  --bin-dir=PATH                Where to put binary (default: PREFIX/bin)
  --desktop-dir=PATH            Desktop entries (default: PREFIX/share/applications)
  --no-remove                   Don't remove nautilus/cosmic-files packages
  --dry-run                     Preview without changes
  --no-desktop                  Skip desktop entry installation
  --no-shims                    Don't create nautilus/cosmic-files shims
  --overwrite-system-binaries   Replace /usr/bin/ binaries (dangerous!)
  --help                        Show help
```

## Troubleshooting

**Q: Permission denied**  
A: Use `sudo` or custom prefix: `--prefix=$HOME/.local`

**Q: No binary found after build**  
A: Install Rust: `rustup install stable` and try again

**Q: Want to test without installing**  
A: Use `--dry-run` flag to see what would happen

**Q: How to uninstall?**  
A: Delete `/usr/local/bin/vortex-fm*` and desktop files in `/usr/local/share/applications/`

## System Requirements

- Arch Linux (or compatible)
- Rust toolchain (`rustup`)
- `cargo` (comes with Rust)
- `pacman` (for package detection)
- `sudo` or `pkexec` (for system install)

Install Rust if needed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## What Gets Installed

```
/usr/local/bin/vortex-fm          ← Main binary
/usr/local/bin/nautilus           ← Shim to vortex-fm
/usr/local/bin/cosmic-files       ← Shim to vortex-fm
/usr/local/share/applications/
  ├── vortex-fm.desktop           ← Main launcher
  ├── org.gnome.Nautilus.desktop  ← GNOME Files compat
  ├── cosmic-files.desktop        ← COSMIC Files compat
  └── com.system76.CosmicFiles.desktop ← System76 COSMIC compat
```

## After Installation

Vortex FM will open automatically when:
- Clicking file manager in app menu
- Opening folders from other apps
- Clicking folder links in web browsers
- Using keyboard shortcuts for file manager

All previous nautilus/cosmic-files functionality is preserved through compatibility shims!
