#!/usr/bin/env bash
# Installs vortex-fm and wires shim binaries for Nautilus and COSMIC Files.
# Supports Arch-specific optional removal of nautilus and cosmic-files.
set -euo pipefail

# Defaults (overridable by flags/env)
PREFIX=${PREFIX:-/usr/local}
BIN_DIR=${BIN_DIR:-"${PREFIX}/bin"}
DESKTOP_DIR=${DESKTOP_DIR:-"${PREFIX}/share/applications"}
NO_REMOVE=${NO_REMOVE:-0}         # 1 to skip package removal
DRY_RUN=${DRY_RUN:-0}             # 1 to only print actions
OVERWRITE_SYSTEM_BINARIES=${OVERWRITE_SYSTEM_BINARIES:-0} # 1 to overwrite /usr/bin binaries (dangerous)
NO_SUDO=${NO_SUDO:-0}             # 1 to avoid sudo/pkexec (useful for testing with custom PREFIX)

refresh_desktop_databases() {
    # Update desktop application cache
    if command -v update-desktop-database >/dev/null 2>&1; then
        info "  Updating desktop database..."
        run_install update-desktop-database "${DESKTOP_DIR}" || true
        if [[ -d /usr/share/applications ]]; then
            run_install update-desktop-database /usr/share/applications || true
        fi
    fi

    # Update MIME type database
    if command -v update-mime-database >/dev/null 2>&1; then
        if [[ -d "${PREFIX}/share/mime" ]]; then
            info "  Updating MIME database..."
            run_install update-mime-database "${PREFIX}/share/mime" || true
        fi
    fi

    # Notify running desktop environment of changes
    # This allows apps to pick up the new .desktop entries without relogin
    if command -v systemctl >/dev/null 2>&1 && (( EUID != 0 )); then
        info "  Notifying session services of desktop entry changes..."
        # Try to signal session manager (works with systemd user sessions)
        systemctl --user try-restart xdg-desktop-portal 2>/dev/null || true
        # Try to reload XDG basedir (for freedesktop compliant DMs)
        dbus-send --print-reply --dest=org.freedesktop.DBus /org/freedesktop/DBus org.freedesktop.DBus.ReloadConfig 2>/dev/null || true
    fi
}

main() {
    parse_args "$@"

    local script_dir project_root binary_path
    script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
    project_root="$(realpath "${script_dir}/..")"
    binary_path="${project_root}/target/release/vortex-fm"

    require_command cargo

    info "Building vortex-fm in release mode..."
    run_cmd cargo build --release --manifest-path "${project_root}/Cargo.toml"

    if [[ ! -x "${binary_path}" ]]; then
        err "Build finished but ${binary_path} is missing or not executable."
        exit 1
    fi

    info "Installing vortex-fm binary to ${BIN_DIR}..."
    run_install install -Dm755 "${binary_path}" "${BIN_DIR}/vortex-fm"

    if (( NO_DESKTOP == 0 )); then
        info "Installing desktop entries to ${DESKTOP_DIR}..."
        install_desktop_entries
    else
        info "Skipping desktop entries as requested."
    fi

    # Create shims before removal, so users aren't left without a file manager
    if (( NO_SHIMS == 0 )); then
        info "Creating shims in ${BIN_DIR}..."
        create_shim_binary "nautilus"
        create_shim_binary "cosmic-files"
    else
        info "Skipping shim creation as requested."
    fi

    if command -v pacman >/dev/null 2>&1; then
        info "Arch system detected (pacman available)."
        if (( NO_REMOVE == 0 )); then
            remove_arch_packages nautilus cosmic-files
        else
            info "Skipping package removal as requested."
        fi
    else
        info "pacman not found; skipping Arch-specific package removal."
    fi

    if (( OVERWRITE_SYSTEM_BINARIES == 1 )); then
        info "Overwriting system binaries in /usr/bin (DANGEROUS)."
        overwrite_system_binary "/usr/bin/nautilus"
        overwrite_system_binary "/usr/bin/cosmic-files"
    fi

    info "Refreshing application databases..."
    refresh_desktop_databases

    info "Installation complete."
}

parse_args() {
    # Option flags
    NO_DESKTOP=0
    NO_SHIMS=0

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --prefix=*) PREFIX="${1#*=}"; BIN_DIR="${PREFIX}/bin"; DESKTOP_DIR="${PREFIX}/share/applications" ;;
            --bin-dir=*) BIN_DIR="${1#*=}" ;;
            --desktop-dir=*) DESKTOP_DIR="${1#*=}" ;;
            --no-remove) NO_REMOVE=1 ;;
            --dry-run) DRY_RUN=1 ;;
            --no-desktop) NO_DESKTOP=1 ;;
            --no-shims) NO_SHIMS=1 ;;
            --overwrite-system-binaries) OVERWRITE_SYSTEM_BINARIES=1 ;;
            --help|-h)
                cat <<USAGE
Usage: $0 [options]
  --prefix=PATH                 Install prefix (default: /usr/local)
  --bin-dir=PATH                Binary dir (default: PREFIX/bin)
  --desktop-dir=PATH            Desktop dir (default: PREFIX/share/applications)
  --no-remove                   Do not remove system packages (nautilus, cosmic-files)
  --dry-run                     Print actions without executing
  --no-desktop                  Do not install desktop entries
  --no-shims                    Do not create nautilus/cosmic-files shims
  --overwrite-system-binaries   Replace /usr/bin/{nautilus,cosmic-files} with shims (DANGEROUS)
USAGE
                exit 0 ;;
            *) err "Unknown option: $1"; exit 2 ;;
        esac
        shift
    done
}

info() { echo "[INFO] $*"; }
err() { echo "[ERROR] $*" >&2; }

require_command() {
    if ! command -v "$1" >/dev/null 2>&1; then
        err "Required command '$1' not found in PATH."
        exit 1
    fi
}

run_cmd() {
    if (( DRY_RUN == 1 )); then
        echo "+ $*"
        return 0
    fi
    "$@"
}

run_as_root() {
    # Honor NO_SUDO for testing in a writable PREFIX
    if (( NO_SUDO == 1 )); then
        run_cmd "$@"
        return
    fi

    if (( EUID == 0 )); then
        run_cmd "$@"
        return
    fi

    local cmd_path
    cmd_path="$(command -v "$1")" || {
        err "Unable to elevate: command '$1' not found in PATH."
        exit 1
    }
    shift

    if command -v pkexec >/dev/null 2>&1; then
        run_cmd pkexec "${cmd_path}" "$@"
        return
    fi

    if command -v sudo >/dev/null 2>&1; then
        run_cmd sudo "${cmd_path}" "$@"
        return
    fi

    err "Root privileges required for '${cmd_path} $*', but neither pkexec nor sudo are available. Re-run this script as root."
    exit 1
}

# Like run_as_root, but auto-detects if elevation is needed based on target path writability
run_install() {
    local cmd="$1"; shift
    local last_arg="${@: -1}"

    # If the last arg is a path inside a directory writable by user, skip sudo/pkexec
    local target_dir
    target_dir="${last_arg}"
    if [[ -d "${target_dir}" ]]; then
        : # use as-is
    else
        target_dir="$(dirname -- "${target_dir}")"
    fi

    if [[ -w "${target_dir}" ]] || (( NO_SUDO == 1 )); then
        run_cmd "${cmd}" "$@"
    else
        run_as_root "${cmd}" "$@"
    fi
}

install_desktop_entries() {
    # Main desktop file for vortex-fm
    run_install install -d "${DESKTOP_DIR}"
    run_as_root tee "${DESKTOP_DIR}/vortex-fm.desktop" >/dev/null <<EOF
[Desktop Entry]
Name=Vortex File Manager
Comment=Browse and manage files with Vortex
Exec=${BIN_DIR}/vortex-fm %U
TryExec=${BIN_DIR}/vortex-fm
Icon=system-file-manager
Terminal=false
Type=Application
Categories=Utility;FileManager;System;
MimeType=inode/directory;application/x-directory;
StartupNotify=true
Keywords=files;file manager;folders;directories;
EOF

    # Compatibility desktop entries to emulate Nautilus and COSMIC Files
    # These help some launchers or MIME associations map to Vortex.
    install_compat_desktop "org.gnome.Nautilus.desktop" "Files (Vortex)"
    install_compat_desktop "cosmic-files.desktop" "COSMIC Files (Vortex)"
    install_compat_desktop "com.system76.CosmicFiles.desktop" "COSMIC Files (Vortex)"
}

install_compat_desktop() {
    local id name
    id="$1"; name="$2"
    run_as_root tee "${DESKTOP_DIR}/${id}" >/dev/null <<EOF
[Desktop Entry]
Name=${name}
Comment=Compatibility launcher that opens Vortex File Manager
Exec=${BIN_DIR}/vortex-fm %U
TryExec=${BIN_DIR}/vortex-fm
Icon=system-file-manager
Terminal=false
Type=Application
Categories=Utility;FileManager;System;
MimeType=inode/directory;application/x-directory;
StartupNotify=true
NoDisplay=true
EOF
}

remove_arch_packages() {
    local pkg
    for pkg in "$@"; do
        if pacman -Qi "${pkg}" >/dev/null 2>&1; then
            info "Removing package ${pkg}..."
            if (( DRY_RUN == 1 )); then
                echo "+ pacman -Rns --noconfirm ${pkg}"
            else
                run_as_root pacman -Rdd --noconfirm "${pkg}"
            fi
        else
            info "Package ${pkg} not installed; skipping removal."
        fi
    done
}

create_shim_binary() {
    local name target
    name="$1"
    target="${BIN_DIR}/${name}"

    info "Creating shim binary at ${target}..."
    if (( DRY_RUN == 1 )); then
        echo "+ install -Dm755 <shim> ${target}"
        return
    fi

    run_install install -Dm755 /dev/stdin "${target}" <<EOF
#!/usr/bin/env bash
exec "${BIN_DIR}/vortex-fm" "\$@"
EOF
}

overwrite_system_binary() {
    local path="$1"
    if [[ ! -e "${path}" ]]; then
        info "${path} not present; creating shim."
        run_as_root install -Dm755 /dev/stdin "${path}" <<EOF
#!/usr/bin/env bash
exec "${BIN_DIR}/vortex-fm" "\$@"
EOF
        return
    fi

    # Backup then replace
    local backup="${path}.bak.$(date +%s)"
    info "Backing up ${path} -> ${backup} and replacing with shim."
    if (( DRY_RUN == 1 )); then
        echo "+ mv ${path} ${backup}"
        echo "+ install -Dm755 <shim> ${path}"
        return
    fi

    run_as_root mv "${path}" "${backup}"
    run_as_root install -Dm755 /dev/stdin "${path}" <<EOF
#!/usr/bin/env bash
exec "${BIN_DIR}/vortex-fm" "\$@"
EOF
}

main "$@"
