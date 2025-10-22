// Helper functions for file operations
// These are private helpers called from app.rs

use std::path::PathBuf;
use crate::core::operations::OperationSelection;

/// Validate operation paths
pub(crate) fn validate_operation_paths(paths: &[PathBuf]) -> bool {
    !paths.is_empty()
}

/// Create operation selection with selected paths
pub(crate) fn create_operation_selection(selected: Vec<PathBuf>) -> OperationSelection {
    OperationSelection {
        ignored: Vec::new(),
        selected,
    }
}

/// Check if operation is in progress
pub(crate) fn is_operation_running(running_count: usize) -> bool {
    running_count > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_operation_paths_empty() {
        assert!(!validate_operation_paths(&[]));
    }

    #[test]
    fn test_validate_operation_paths_not_empty() {
        let paths = vec![PathBuf::from("/test")];
        assert!(validate_operation_paths(&paths));
    }

    #[test]
    fn test_is_operation_running_false() {
        assert!(!is_operation_running(0));
    }

    #[test]
    fn test_is_operation_running_true() {
        assert!(is_operation_running(1));
    }
}
