// Helper functions for search functionality
// These are private helpers called from app.rs

use std::path::PathBuf;

/// Filter paths by search query
pub(crate) fn filter_by_search(paths: &[PathBuf], query: &str) -> Vec<PathBuf> {
    if query.is_empty() {
        return paths.to_vec();
    }
    
    let lower_query = query.to_lowercase();
    paths.iter()
        .filter(|path| {
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    return name_str.to_lowercase().contains(&lower_query);
                }
            }
            false
        })
        .cloned()
        .collect()
}

/// Check if search is active
pub(crate) fn is_search_active(query: &str) -> bool {
    !query.trim().is_empty()
}

/// Clear search results
pub(crate) fn clear_search_results() -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_by_search_empty_query() {
        let paths = vec![
            PathBuf::from("/test/file1.txt"),
            PathBuf::from("/test/file2.rs"),
        ];
        let result = filter_by_search(&paths, "");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_filter_by_search_match() {
        let paths = vec![
            PathBuf::from("/test/file1.txt"),
            PathBuf::from("/test/document.txt"),
        ];
        let result = filter_by_search(&paths, "file");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].file_name().unwrap(), "file1.txt");
    }

    #[test]
    fn test_filter_by_search_case_insensitive() {
        let paths = vec![PathBuf::from("/test/MyFile.TXT")];
        let result = filter_by_search(&paths, "myfile");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_is_search_active_false() {
        assert!(!is_search_active(""));
        assert!(!is_search_active("   "));
    }

    #[test]
    fn test_is_search_active_true() {
        assert!(is_search_active("test"));
    }

    #[test]
    fn test_clear_search_results() {
        let result = clear_search_results();
        assert_eq!(result, "");
    }
}
