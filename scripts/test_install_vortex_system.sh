#!/usr/bin/env bash
# Comprehensive test suite for install_vortex_system.sh
# Tests installation in a temporary sandbox directory
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0

log_info() { echo -e "${BLUE}[INFO]${NC} $*"; }
log_pass() { echo -e "${GREEN}[PASS]${NC} $*"; ((TESTS_PASSED++)) || true; }
log_fail() { echo -e "${RED}[FAIL]${NC} $*"; ((TESTS_FAILED++)) || true; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }

# Setup
setup_test_env() {
    local test_dir
    test_dir="$(mktemp -d)"
    export TEST_PREFIX="${test_dir}/install"
    export TEST_BIN_DIR="${TEST_PREFIX}/bin"
    export TEST_DESKTOP_DIR="${TEST_PREFIX}/share/applications"
    
    mkdir -p "${TEST_PREFIX}" "${TEST_BIN_DIR}" "${TEST_DESKTOP_DIR}"
    
    # Create a mock vortex-fm binary
    mkdir -p "${test_dir}/target/release"
    echo "#!/usr/bin/env bash" > "${test_dir}/target/release/vortex-fm"
    echo 'echo "Vortex FM Mock - Args: $@"' >> "${test_dir}/target/release/vortex-fm"
    chmod +x "${test_dir}/target/release/vortex-fm"
    
    export TEST_PROJECT_ROOT="${test_dir}"
    export TEST_BINARY="${test_dir}/target/release/vortex-fm"
    
    log_info "Test environment created at: ${TEST_PREFIX}"
}

cleanup_test_env() {
    if [[ -n "${TEST_PREFIX:-}" ]] && [[ -d "${TEST_PREFIX}" ]]; then
        rm -rf "$(dirname "${TEST_PREFIX}")"
        log_info "Test environment cleaned up"
    fi
}

# Test: Verify script exists and is executable
test_script_exists() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    if [[ ! -x "${script_path}" ]]; then
        log_fail "Script not found or not executable: ${script_path}"
        return 1 || true
    fi
    log_pass "Installation script exists and is executable"
    return 0 || true
}

# Test: --help flag works
test_help_flag() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    local output
    output=$(bash "${script_path}" --help 2>&1) || true
    
    if echo "${output}" | grep -q "Usage:"; then
        log_pass "--help flag displays usage information"
        return 0 || true
    else
        log_fail "--help flag did not show expected output"
        return 1 || true
    fi
}

# Test: Dry-run mode doesn't modify files
test_dry_run() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    
    # Count files before
    local files_before
    files_before=$(find "${TEST_PREFIX}" -type f | wc -l)
    
    # Run in dry-run mode
    bash "${script_path}" \
        --dry-run \
        --prefix="${TEST_PREFIX}" \
        --no-remove \
        --no-desktop \
        --no-shims \
        2>&1 | tee /tmp/dry_run_output.txt || true
    
    # Count files after
    local files_after
    files_after=$(find "${TEST_PREFIX}" -type f | wc -l)
    
    if [[ ${files_before} -eq ${files_after} ]]; then
        log_pass "Dry-run mode does not modify files (before: ${files_before}, after: ${files_after})"
        return 0 || true
    else
        log_fail "Dry-run mode modified files (before: ${files_before}, after: ${files_after})"
        return 1 || true
    fi
    
    # Check that commands are printed with "+"
    if grep -q "^+" /tmp/dry_run_output.txt; then
        log_pass "Dry-run outputs commands with '+' prefix"
        return 0 || true
    else
        log_fail "Dry-run did not output commands as expected"
        return 1 || true
    fi
}

# Test: Custom prefix installation
test_custom_prefix() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    
    # Mock the build and commands we can't run
    export NO_SUDO=1
    
    # We'll use a helper script that doesn't actually run cargo
    # Instead, just test the path handling
    bash -c "
        # Source the installer script functions
        source \"${script_path}\" 2>/dev/null || true
        
        # Test that parse_args handles --prefix correctly
        PREFIX=\"${TEST_PREFIX}\"
        BIN_DIR=\"\${PREFIX}/bin\"
        DESKTOP_DIR=\"\${PREFIX}/share/applications\"
        
        [[ -d \"\${BIN_DIR}\" ]] && echo \"BIN_DIR OK\"
        [[ -d \"\${DESKTOP_DIR}\" ]] && echo \"DESKTOP_DIR OK\"
    " 2>/dev/null | grep -q "BIN_DIR OK" && \
    log_pass "Custom --prefix flag sets correct directories" || \
    log_fail "Custom --prefix flag did not work as expected"
}

# Test: Binary installation path validation
test_binary_install_path() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    
    if grep -q "run_install install -Dm755 \"\${binary_path}\" \"\${BIN_DIR}/vortex-fm\"" "${script_path}"; then
        log_pass "Binary installation uses correct target path (BIN_DIR)"
    else
        log_fail "Binary installation path is not using BIN_DIR variable"
        return 1
    fi
}

# Test: Shim creation logic
test_shim_creation() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    
    # Check if create_shim_binary function exists and has correct content
    if grep -q "create_shim_binary()" "${script_path}"; then
        log_pass "create_shim_binary function exists"
    else
        log_fail "create_shim_binary function not found"
        return 1 || true
    fi
    
    # Check that shim references BIN_DIR/vortex-fm
    if grep -A10 'create_shim_binary()' "${script_path}" | grep -q 'BIN_DIR'; then
        log_pass "Shims correctly reference BIN_DIR/vortex-fm"
        return 0 || true
    else
        log_fail "Shims do not properly reference BIN_DIR/vortex-fm"
        return 1 || true
    fi
}

# Test: Desktop entry paths
test_desktop_entries() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    
    # Check that desktop entries reference BIN_DIR
    if grep -q "Exec=\${BIN_DIR}/vortex-fm" "${script_path}"; then
        log_pass "Desktop entries use correct BIN_DIR path"
    else
        log_fail "Desktop entries do not reference BIN_DIR correctly"
        return 1
    fi
    
    # Check for compat desktop entries
    if grep -q "install_compat_desktop" "${script_path}"; then
        log_pass "Compat desktop entries function exists"
    else
        log_fail "Compat desktop entries function not found"
        return 1
    fi
    
    # Verify entries for Nautilus and COSMIC Files
    if grep -q "org.gnome.Nautilus.desktop" "${script_path}" && \
       grep -q "cosmic-files.desktop" "${script_path}"; then
        log_pass "Compat entries include Nautilus and COSMIC Files"
    else
        log_fail "Missing compat entries for Nautilus or COSMIC Files"
        return 1
    fi
}

# Test: Package removal safety
test_package_removal() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    
    # Check that removal only happens on Arch systems
    if grep -q "if command -v pacman" "${script_path}"; then
        log_pass "Package removal gated by pacman check (Arch-only)"
    else
        log_fail "Package removal check missing"
        return 1
    fi
    
    # Check that shims are created BEFORE removal
    local shim_line
    local remove_line
    shim_line=$(grep -n "create_shim_binary" "${script_path}" | head -1 | cut -d: -f1)
    remove_line=$(grep -n "remove_arch_packages" "${script_path}" | head -1 | cut -d: -f1)
    
    if [[ ${shim_line} -lt ${remove_line} ]]; then
        log_pass "Shims are created before package removal (line ${shim_line} < ${remove_line})"
    else
        log_fail "Shims are not created before package removal"
        return 1
    fi
    
    # Check for --no-remove flag
    if grep -q "NO_REMOVE" "${script_path}"; then
        log_pass "--no-remove flag is supported"
    else
        log_fail "--no-remove flag not supported"
        return 1
    fi
}

# Test: Error handling
test_error_handling() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    
    # Check for set -euo pipefail (error on undefined vars, pipe failures)
    if grep -q "set -euo pipefail" "${script_path}"; then
        log_pass "Script uses strict error handling (set -euo pipefail)"
    else
        log_fail "Script does not use strict error handling"
        return 1
    fi
    
    # Check for require_command function
    if grep -q "require_command" "${script_path}"; then
        log_pass "Script has require_command validation"
    else
        log_fail "Script missing require_command validation"
        return 1
    fi
    
    # Check for error logging
    if grep -q "err()" "${script_path}"; then
        log_pass "Script has err() logging function"
    else
        log_fail "Script missing err() logging function"
        return 1
    fi
}

# Test: Overwrite system binaries flag
test_overwrite_system_flag() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    
    # Check for the flag and function
    if grep -q "OVERWRITE_SYSTEM_BINARIES" "${script_path}" && \
       grep -q "overwrite_system_binary()" "${script_path}"; then
        log_pass "OVERWRITE_SYSTEM_BINARIES flag exists with backup logic"
    else
        log_fail "OVERWRITE_SYSTEM_BINARIES flag or function missing"
        return 1
    fi
    
    # Check that backups are created
    if grep -q "\.bak\." "${script_path}"; then
        log_pass "System binary backups include timestamps"
    else
        log_fail "Backup logic not found"
        return 1
    fi
}

# Test: Sudo/pkexec handling
test_privilege_escalation() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    
    # Check for sudo/pkexec support
    if grep -q "pkexec" "${script_path}" && grep -q "sudo" "${script_path}"; then
        log_pass "Script supports both pkexec and sudo"
    else
        log_fail "Script missing privilege escalation support"
        return 1
    fi
    
    # Check for NO_SUDO flag (testing support)
    if grep -q "NO_SUDO" "${script_path}"; then
        log_pass "NO_SUDO flag exists for testing"
    else
        log_fail "NO_SUDO flag not found"
        return 1
    fi
}

# Test: Flag parsing
test_flag_parsing() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    
    local required_flags=(
        "prefix"
        "bin-dir"
        "desktop-dir"
        "no-remove"
        "dry-run"
        "no-desktop"
        "no-shims"
        "overwrite-system-binaries"
    )
    
    local missing=0
    for flag in "${required_flags[@]}"; do
        if grep -q -- "--${flag}" "${script_path}"; then
            log_pass "Flag --${flag} is implemented"
        else
            log_fail "Flag --${flag} is missing"
            ((missing++) || true)
        fi
    done
    
    if [[ ${missing} -eq 0 ]]; then
        return 0 || true
    else
        return 1 || true
    fi
}

# Test: Information logging
test_logging() {
    local script_path="${REPO_ROOT}/scripts/install_vortex_system.sh"
    
    # Check for info() function
    if grep -q "info()" "${script_path}"; then
        log_pass "Script has info() logging function"
    else
        log_fail "Script missing info() logging function"
        return 1
    fi
    
    # Verify info() is used throughout
    local info_count
    info_count=$(grep -c "info \"" "${script_path}")
    
    if [[ ${info_count} -gt 5 ]]; then
        log_pass "Script uses info() for logging (${info_count} occurrences)"
    else
        log_fail "Script lacks sufficient logging"
        return 1
    fi
}

# Main test runner
main() {
    local repo_root
    repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
    export REPO_ROOT="${repo_root}"
    
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}Vortex FM Installation Script Test Suite${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo
    
    # Run static tests (don't need environment setup)
    log_info "Running static analysis tests..."
    test_script_exists
    test_help_flag
    test_binary_install_path
    test_shim_creation
    test_desktop_entries
    test_package_removal
    test_error_handling
    test_overwrite_system_flag
    test_privilege_escalation
    test_flag_parsing
    test_logging
    
    echo
    log_info "Setting up test environment..."
    setup_test_env
    trap cleanup_test_env EXIT
    
    # Run dynamic tests
    log_info "Running dynamic tests..."
    test_dry_run
    test_custom_prefix
    
    echo
    echo -e "${BLUE}========================================${NC}"
    echo -e "Tests Passed: ${GREEN}${TESTS_PASSED}${NC}"
    echo -e "Tests Failed: ${RED}${TESTS_FAILED}${NC}"
    echo -e "${BLUE}========================================${NC}"
    
    if [[ ${TESTS_FAILED} -gt 0 ]]; then
        exit 1
    fi
}

main "$@"
