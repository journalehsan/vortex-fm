# Vortex FM Installation Script Improvements

## Overview
The `install_vortex_system.sh` script has been significantly improved to properly handle system-wide installation of Vortex FM with safe replacement of Nautilus and COSMIC Files on Arch Linux systems, with comprehensive testing.

## Key Improvements

### 1. **Correct Binary Installation Paths**
- **Before**: Installed to `/usr/share/bin/` (incorrect location for executables)
- **After**: Uses configurable `${BIN_DIR}` (defaults to `/usr/local/bin`)
- Impact: Follows FHS standards and makes it easier to manage installation locations

### 2. **Flexible Installation Configuration**
Added command-line flags for fine-grained control:
- `--prefix=PATH`: Set install prefix (default: `/usr/local`)
- `--bin-dir=PATH`: Specify binary directory
- `--desktop-dir=PATH`: Specify desktop entry directory
- `--no-remove`: Skip automatic package removal
- `--dry-run`: Preview actions without executing
- `--no-desktop`: Skip desktop entry installation
- `--no-shims`: Skip shim creation
- `--overwrite-system-binaries`: Replace `/usr/bin/{nautilus,cosmic-files}` with shims (dangerous)

### 3. **Smart Privilege Escalation**
- Auto-detects if root is needed based on target directory writability
- Attempts `pkexec` first, falls back to `sudo`
- `NO_SUDO` flag for testing with user-writable directories
- Allows installation to custom prefixes without sudo

### 4. **Safe Package Replacement Logic**
- **Creates shims BEFORE removing packages** - ensures users never lose access to a file manager
- Only removes packages on Arch systems (pacman check)
- `--no-remove` flag allows installation without package removal
- All operations are reversible (can install to custom prefix first)

### 5. **Compatibility Desktop Entries**
Installs multiple desktop entry files to handle various MIME type associations:
- `vortex-fm.desktop` - Main Vortex entry
- `org.gnome.Nautilus.desktop` - GNOME Files compatibility
- `cosmic-files.desktop` - COSMIC Files compatibility  
- `com.system76.CosmicFiles.desktop` - System76 COSMIC Files compatibility

These help system launchers and applications that expect Nautilus or COSMIC Files to open Vortex instead.

### 6. **System Binary Overwriting (Optional)**
- `--overwrite-system-binaries` flag to replace `/usr/bin/{nautilus,cosmic-files}`
- Creates timestamped backups: `/usr/bin/nautilus.bak.TIMESTAMP`
- Backup shims so the system binaries are available as Vortex launchers

### 7. **Improved Logging and Diagnostics**
- Uses consistent `info()`, `err()`, and `log_pass()`/`log_fail()` functions
- Clear status messages for each installation step
- Dry-run mode shows all commands with `+` prefix

### 8. **Better Error Handling**
- Strict bash mode: `set -euo pipefail`
- Validates required commands with `require_command()`
- Explicit error messages for missing dependencies

## Testing

### Comprehensive Test Suite
A new `test_install_vortex_system.sh` script provides 30+ automated tests:

**Static Analysis Tests** (18 tests):
- Script existence and executability
- Help flag functionality
- Binary and desktop entry paths
- Shim creation and configuration
- Package removal safety
- Error handling and logging
- Privilege escalation support
- All command-line flags

**Dynamic Tests** (2 tests):
- Dry-run mode validation
- Custom prefix installation
- Test environment with mock binary

### Running Tests
```bash
./scripts/test_install_vortex_system.sh
```

**Expected Output**: All 30+ tests should pass with `exit 0`

## Usage Examples

### Standard Arch System Installation
```bash
sudo ./scripts/install_vortex_system.sh
```
- Builds Vortex FM
- Installs to `/usr/local/bin/vortex-fm`
- Creates desktop entries
- Creates nautilus/cosmic-files shims in `/usr/local/bin`
- Removes nautilus and cosmic-files packages (prompts to keep)
- Updates desktop database

### Test Installation (No Sudo Required)
```bash
./scripts/install_vortex_system.sh \
  --prefix=/tmp/vortex-test \
  --no-remove \
  --no-sudo
```
- Installs to temp directory
- Skips package removal
- No sudo/pkexec elevation

### Dry-Run Preview
```bash
./scripts/install_vortex_system.sh --dry-run
```
- Shows all commands that would be executed
- No files are modified

### Preserve Packages, Only Install Shims
```bash
sudo ./scripts/install_vortex_system.sh --no-remove
```
- Keeps nautilus/cosmic-files packages installed
- Creates nautilus/cosmic-files shims pointing to Vortex
- System can still uninstall packages later

### Overwrite System Binaries (Dangerous)
```bash
sudo ./scripts/install_vortex_system.sh \
  --overwrite-system-binaries
```
- Replaces `/usr/bin/nautilus` and `/usr/bin/cosmic-files` with shims
- Creates timestamped backups of originals
- Direct replacement (not recommended in production)

## omarchy Integration

For omarchy easy migration on Arch systems:

1. **Basic Migration Script** can call:
```bash
bash scripts/install_vortex_system.sh
```

2. **Non-Destructive Preview** (recommended first):
```bash
bash scripts/install_vortex_system.sh --dry-run
```

3. **Custom Prefix for Testing**:
```bash
bash scripts/install_vortex_system.sh \
  --prefix=$HOME/.local \
  --no-remove
```

4. **Rollback Support**: 
   - Backup originals at `/usr/bin/{nautilus,cosmic-files}.bak.*`
   - Can restore via `mv /usr/bin/{nautilus,cosmic-files}.bak.* /usr/bin/`

## Architecture

### Installation Flow
```
1. Parse command-line arguments
2. Build Vortex FM (cargo build --release)
3. Validate binary exists and is executable
4. Create target directories
5. Install Vortex binary to BIN_DIR
6. Install desktop entries to DESKTOP_DIR
7. Create shim binaries (before package removal!)
8. Detect Arch system and optionally remove packages
9. Optionally overwrite system binaries with backups
10. Update desktop database
```

### Key Functions
- `parse_args()` - Command-line flag parsing
- `run_cmd()` - Execute commands (respects DRY_RUN)
- `run_as_root()` - Privilege escalation
- `run_install()` - Smart elevation based on path writability
- `install_desktop_entries()` - Main + compat entries
- `create_shim_binary()` - Create launcher shims
- `remove_arch_packages()` - Safe package removal
- `overwrite_system_binary()` - Replace system binaries with backups

## Environment Variables
- `PREFIX` - Install prefix (default: `/usr/local`)
- `BIN_DIR` - Binary directory (default: `PREFIX/bin`)
- `DESKTOP_DIR` - Desktop directory (default: `PREFIX/share/applications`)
- `NO_REMOVE` - Skip package removal (0=remove, 1=skip)
- `DRY_RUN` - Preview mode (0=execute, 1=preview)
- `OVERWRITE_SYSTEM_BINARIES` - Replace /usr/bin/ binaries (0=no, 1=yes)
- `NO_SUDO` - Skip elevation for testing (0=use sudo, 1=skip)

## Troubleshooting

### "Build finished but binary is missing"
- Ensure Rust toolchain is installed: `rustup install stable`
- Check Cargo.toml is valid
- Run `cargo build --release` manually to see full error

### "Root privileges required"
- Neither `pkexec` nor `sudo` available
- Run script as root: `sudo bash scripts/install_vortex_system.sh`
- Or use custom prefix: `--prefix=$HOME/.local`

### "Permission denied" on desktop dir
- Desktop dir not writable
- Use custom prefix: `--prefix=$HOME/.local`
- Or run with sudo for /usr/local/share

### Package removal fails
- Use `--no-remove` flag to skip removal
- Or manually remove: `sudo pacman -Rns nautilus cosmic-files`

## Files Created/Modified

### Created
- `/usr/local/bin/vortex-fm` - Main binary
- `/usr/local/bin/nautilus` - Shim (optional)
- `/usr/local/bin/cosmic-files` - Shim (optional)
- `/usr/local/share/applications/vortex-fm.desktop`
- `/usr/local/share/applications/org.gnome.Nautilus.desktop` (compat)
- `/usr/local/share/applications/cosmic-files.desktop` (compat)
- `/usr/local/share/applications/com.system76.CosmicFiles.desktop` (compat)

### Removed (with `--no-remove` skipped)
- Package: `nautilus`
- Package: `cosmic-files`

### Backed Up (with `--overwrite-system-binaries`)
- `/usr/bin/nautilus` → `/usr/bin/nautilus.bak.TIMESTAMP`
- `/usr/bin/cosmic-files` → `/usr/bin/cosmic-files.bak.TIMESTAMP`

## Future Improvements
- Support for other Linux distributions (RPM, DEB)
- Automatic detection of default shell for desktop entries
- systemd user service integration
- Update checking and auto-update capability
